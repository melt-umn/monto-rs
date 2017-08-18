use std::cell::RefCell;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, TryRecvError};

use futures::{Async, Future, Stream};
use notify::DebouncedEvent;
use tokio_core::reactor::Handle;
use void::Void;

use super::cache::Cache;

/// A future for filesystem events. Will never resolve.
pub fn watch_future(cache: Rc<RefCell<Cache>>, chan: Receiver<DebouncedEvent>) -> Box<Future<Item=(), Error=()>> {
    Box::new(chan.for_each(|ev| {
        info!("{:?}", ev);

        let cache = cache.borrow_mut();
        let cache = cache.deref_mut();

        match ev {
            DebouncedEvent::NoticeWrite(path) => recursive_evict(cache, path),
            DebouncedEvent::NoticeRemove(path) => recursive_evict(cache, path),
            DebouncedEvent::Create(path) => recursive_evict(cache, path),
            DebouncedEvent::Write(path) => recursive_evict(cache, path),
            DebouncedEvent::Chmod(path) => recursive_evict(cache, path),
            DebouncedEvent::Remove(path) => recursive_evict(cache, path),
            DebouncedEvent::Rename(path, _) => recursive_evict(cache, path),
            DebouncedEvent::Rescan => {},
            DebouncedEvent::Error(err, path) => {
                error!("{}", err);
                if let Some(path) = path {
                    recursive_evict(cache, path)
                }
            },
        }
    }))
}

fn recursive_evict(cache: &mut Cache, mut path: PathBuf) {
    info!("Evicting path {} from cache", path.display());
    cache.evict_by_path(path.clone());
    while path.pop() {
        info!("Evicting path {} from cache", path.display());
        cache.evict_by_path(path.clone());
    }
}
