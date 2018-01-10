use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel as mpsc_channel, RecvTimeoutError};
use std::thread::{spawn, JoinHandle};

use futures::{Async, Future, Poll, Sink, Stream};
use futures::sync::mpsc::{unbounded as futures_channel,
                          UnboundedReceiver as Receiver};
use futures::sync::oneshot::{channel as oneshot_channel, Canceled,
                             Sender as OneshotSender};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode,
             Watcher as WatcherTrait};

use consts::{WATCHER_DEBOUNCE_TIME, WATCHER_RECV_LOOP_TIMEOUT};
use errors::{Error, Result};

/// A stream of paths that have been modified.
///
/// TODO: This currently starts a helper thread. When [`notify`
/// 5.0.0](https://github.com/passcod/notify/issues/117) comes out, it should
/// be possible to eliminate that too.
pub struct Watcher {
    /// A channel of changed paths.
    event_recv: Receiver<WatchEvent>,

    /// A channel to tell the thread to stop.
    ///
    /// `stop_send` should always be `Some(_)` until the `Watcher` is dropped.
    stop_send: Option<OneshotSender<()>>,

    /// A handle to the thread that waits on the watcher's event channel,
    /// sending the events to the path channel.
    ///
    /// Since `std::sync::mpsc` channels can't be converted to
    /// `futures::sync::mpsc` channels, this thread is spun up to listen for
    /// events from the `notify` channel.
    thread: JoinHandle<()>,

    /// The watcher object this struct wraps.
    watcher: RecommendedWatcher,
}

impl Watcher {
    /// Creates a new `Watcher`.
    pub fn new() -> Result<Watcher> {
        let (notify_send, notify_recv) = mpsc_channel();
        let (event_send, event_recv) = futures_channel();
        let (stop_send, stop_recv) = oneshot_channel();
        let watcher =
            RecommendedWatcher::new(notify_send, *WATCHER_DEBOUNCE_TIME)?;
        let thread = spawn(move || {
            let event_send = &event_send;
            let mut stop_recv = stop_recv;
            loop {
                match notify_recv.recv_timeout(*WATCHER_RECV_LOOP_TIMEOUT) {
                    Ok(ev) => {
                        let ev = match ev {
                            DebouncedEvent::NoticeWrite(path)
                            | DebouncedEvent::NoticeRemove(path)
                            | DebouncedEvent::Create(path)
                            | DebouncedEvent::Write(path)
                            | DebouncedEvent::Chmod(path) => {
                                Some(WatchEvent::Modify(path))
                            }
                            DebouncedEvent::Remove(path)
                            | DebouncedEvent::Rename(path, _) => {
                                Some(WatchEvent::Delete(path))
                            }
                            DebouncedEvent::Rescan => None,
                            DebouncedEvent::Error(err, path) => {
                                error!("Error at {:?}: {}", path, err);
                                None
                            }
                        };
                        if let Some(ev) = ev {
                            event_send.wait().send(ev).unwrap()
                        }
                    }
                    Err(RecvTimeoutError::Timeout) => match stop_recv.poll() {
                        Ok(Async::Ready(())) | Err(Canceled) => break,
                        Ok(Async::NotReady) => {}
                    },
                    Err(RecvTimeoutError::Disconnected) => break,
                }
            }
        });
        Ok(Watcher {
            event_recv,
            stop_send: Some(stop_send),
            thread,
            watcher,
        })
    }

    /// Adds a path to be watched by the `Watcher`.
    pub fn add_watch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.watcher
            .watch(path, RecursiveMode::NonRecursive)
            .map_err(Error::from)
    }
}

impl Drop for Watcher {
    fn drop(&mut self) {
        let stop_send = self.stop_send.take().expect("Either Watcher got double-dropped or something messed with stop_send");
        stop_send.send(()).unwrap();
    }
}

impl Stream for Watcher {
    type Item = WatchEvent;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<WatchEvent>, ()> {
        self.event_recv.poll()
    }
}

/// Events sent from the `Watcher`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WatchEvent {
    /// A file was modified.
    Modify(PathBuf),

    /// A file was deleted.
    Delete(PathBuf),
}
