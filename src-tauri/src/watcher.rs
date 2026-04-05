use std::collections::HashMap;
use std::sync::Mutex;
use tokio::task::AbortHandle;

pub struct WatcherState {
    pub watchers: Mutex<HashMap<u16, AbortHandle>>,
}

impl WatcherState {
    pub fn new() -> Self {
        WatcherState {
            watchers: Mutex::new(HashMap::new()),
        }
    }
}

/*
- HashMap<u16, AbortHandle> - It is a dictionary where the key is the
port number (u16 = unsigned 16-bit integer, ports go 0 - 65535) and
the value is an AbortHandle (a remote control that lets you cancel a running task)
-Mutex - a lock around the HasMap, Because multiple things could try to read/write the list at the same time
(the ui calls watch, unwatch, etc.) The Mutex enseured only one thinkg touches it at a time. Think of it as
"one person in the room at a time" rule.
- impl WatcherState { pub fn new()} - this is just a constructor , the Rust equivalent of a class method
that creates a fress instance.
 */