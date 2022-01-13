mod utils;

use wasm_bindgen::prelude::*;
use rxrust::prelude::*;

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

    observable::from_iter(0..100)
      .take(5)
      .subscribe(|n| console::log_2(&"Logging items emitted from observable ".into(), &n.into()));
}
