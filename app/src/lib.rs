mod utils;

use wasm_bindgen::prelude::*;
use rxrust::prelude::*;
use std::time::Duration;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    use web_sys::console;
    use rxrust::scheduler::LocalSpawner;

    observable::js_interval(Duration::from_secs(1))
      .take(5)
      .subscribe(|n| console::log_2(&"Logging items emitted from observable ".into(), &n.into()));
}
