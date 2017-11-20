#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref CACHE: Mutex<HashSet<Arc<str>>> = Default::default();
}

pub fn interned(s: &str) -> Arc<str> {
    let mut cache = CACHE.lock().expect("Poisoned Mutex");
    if !cache.contains(s) {
        cache.insert(Arc::from(s));
    }
    cache
        .get(s)
        .unwrap_or_else(|| unreachable!("Just added"))
        .clone()
}
