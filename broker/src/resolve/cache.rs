use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::time::Duration;

use notify::{DebouncedEvent, Error as NotifyError, RecommendedWatcher, RecursiveMode,
             Watcher as NotifyWatcher};
use serde_json::Value;
use tokio_core::reactor::Handle;

use monto3_common::messages::{Product, ProductDescriptor, ProductIdentifier};

use resolve::watcher::Watcher;

/// A cache for products.
pub struct Cache {
    products: BTreeMap<PathBuf, BTreeMap<ProductDescriptor, Value>>,
    watcher: RecommendedWatcher,
    watching: BTreeSet<PathBuf>,
    watch_chan: Receiver<DebouncedEvent>,
}

impl Cache {
    /// Creates a new cache.
    pub fn new(handle: &Handle) -> Result<Rc<RefCell<Cache>>, NotifyError> {
        let (send, recv) = channel();
        let watcher = RecommendedWatcher::new(send, Duration::from_millis(100))?;
        let cache = Rc::new(RefCell::new(Cache {
            products: BTreeMap::new(),
            watcher: watcher,
            watching: BTreeSet::new(),
            watch_chan: recv,
        }));
        handle.spawn(Watcher::new(cache.clone()));
        Ok(cache)
    }

    /// Returns the next event received by the FS watcher.
    pub(crate) fn event(&self) -> Option<DebouncedEvent> {
        match self.watch_chan.try_recv() {
            Ok(ev) => Some(ev),
            Err(TryRecvError::Disconnected) => {
                error!(
                    "The filesystem watcher died; https://twitter.com/rob_pike/status/447202124753952768"
                );
                None
            }
            Err(TryRecvError::Empty) => None,
        }
    }

    /// Adds a product to the cache, replacing any other product that was
    /// previously present.
    pub fn add(&mut self, product: Product) {
        let Product {
            name,
            language,
            path,
            value,
        } = product;
        info!("Added to cache: {} {} {}", name, language, path);

        let desc = ProductDescriptor { name, language };
        let path = PathBuf::from(path);
        self.products
            .entry(path.clone())
            .or_insert_with(BTreeMap::new)
            .insert(desc, value);
        if self.watching.insert(path.clone()) {
            if let Err(err) = self.watcher.watch(path, RecursiveMode::Recursive) {
                error!("{}", err);
            }
        }
    }

    /// Removes all products with the given path from the cache.
    pub fn evict_by_path(&mut self, path: PathBuf) {
        let _ = self.products.remove(&path);
        if self.watching.remove(&path) {
            if let Err(err) = self.watcher.unwatch(path) {
                error!("{}", err);
            }
        }
    }

    /// Retrieves a product from the cache.
    pub fn get(&self, pi: ProductIdentifier) -> Option<Product> {
        info!("Cache request for {:?}", pi);

        let path = PathBuf::from(&pi.path);
        self.products.get(&path).and_then(move |m| {
            let ProductIdentifier {
                language,
                name,
                path,
            } = pi;
            let pd = ProductDescriptor { language, name };
            m.get(&pd).map(move |value| {
                Product {
                    language: pd.language,
                    name: pd.name,
                    path: path,
                    value: value.clone(),
                }
            })
        })
    }
}

impl Debug for Cache {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("Cache")
            .field("products", &self.products)
            .field("watching", &self.watching)
            .finish()
    }
}
