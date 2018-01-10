//! Constants that might need tweaking.
//!
//! TODO: Make the `Duration`s `const` once possible.

use std::time::Duration;

lazy_static! {
    pub static ref WATCHER_DEBOUNCE_TIME: Duration = Duration::from_millis(100);
    pub static ref WATCHER_RECV_LOOP_TIMEOUT: Duration = Duration::from_millis(100);
}
