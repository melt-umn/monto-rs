use std::cell::RefCell;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::rc::Rc;

use futures::{Async, Future};
use notify::DebouncedEvent;

use super::cache::Cache;

/// A future for filesystem events. Will never resolve.
pub struct Watcher {
    cache: Rc<RefCell<Cache>>,
}

impl Watcher {
    pub fn new(cache: Rc<RefCell<Cache>>) -> Watcher {
        Watcher { cache }
    }

}

impl Future for Watcher {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<()>, ()> {
        let mut cache = self.cache.borrow_mut();
        let cache = cache.deref_mut();
        while let Some(ev) = cache.event() {
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
        }
        Ok(Async::NotReady)
    }
}

fn recursive_evict(cache: &mut Cache, mut path: PathBuf) {
    info!("Evicting path {} from cache", path.display());
    cache.evict_by_path(path.clone());
    while path.pop() {
        info!("Evicting path {} from cache", path.display());
        cache.evict_by_path(path.clone());
    }
}
