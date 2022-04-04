mod utils;

use rxrust::prelude::*;
use std::time::Duration;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    use rxrust::scheduler::LocalSpawner;
    use web_sys::console;

    observable::interval(Duration::from_secs(1), LocalSpawner)
        .take(5)
        .subscribe(|n| console::log_2(&"Logging items emitted from observable ".into(), &n.into()));
}
