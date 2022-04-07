mod utils;

use wasm_bindgen::prelude::*;
use rxrust::prelude::*;
use std::time::Duration;
use web_sys::console;

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
    use rxrust::scheduler::LocalSpawner;

    observable::interval(Duration::from_secs(1), LocalSpawner)
      .take(5)
      .subscribe(|n| console::log_2(&"Logging items emitted from observable ".into(), &n.into()));
}

#[wasm_bindgen]
pub fn greet2() {
    use web_sys::console;
  console::log_1(&"Hello world".into());
}

#[wasm_bindgen]
pub fn greet3() -> Result<(), JsValue> {
  Err(JsValue::from("Raising exception"))
}

#[derive(Clone)]
pub struct SceneManagerEngineCreated(js_sys::Function);

impl SceneManagerEngineCreated {
    pub fn new(callback: js_sys::Function) -> Self {
        Self(callback)
    }

    pub fn invoke(&self, scene_id: &str, engine: SceneManagerEngineInner, object_controller_engines: Vec<ObjectControllerEngineInner>) {
        let scene_id = JsValue::from(scene_id);
        let engine = JsValue::from(SceneManagerEngine(engine));
        let object_controller_engines: js_sys::Array =
            object_controller_engines
                .into_iter()
            .map(|engine| JsValue::from(ObjectControllerEngine(engine)))
            .collect();
        let this = JsValue::null();
        let _ = self
            .0
            .call3(&this, &scene_id, &engine, &object_controller_engines);
    }
}

#[derive(Clone)]
pub struct ScenePlayerEngineInner(SceneManagerEngineCreated);

#[wasm_bindgen]
#[derive(Clone)]
pub struct ScenePlayerEngine(ScenePlayerEngineInner);

#[wasm_bindgen]
impl ScenePlayerEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(
      scene_manager_engine_created: js_sys::Function,
    ) -> ScenePlayerEngine {
      Self(ScenePlayerEngineInner(SceneManagerEngineCreated::new(scene_manager_engine_created)))
    }

  pub fn foo(&self) {
    self.0.0.invoke("test_scene_id", SceneManagerEngineInner, vec![ObjectControllerEngineInner, ObjectControllerEngineInner]);
  }
}

#[derive(Clone)]
pub struct SceneManagerEngineInner;

#[wasm_bindgen]
#[derive(Clone)]
pub struct SceneManagerEngine(#[wasm_bindgen(skip)] pub SceneManagerEngineInner);


#[derive(Clone)]
pub struct ObjectControllerEngineInner;

#[wasm_bindgen]
#[derive(Clone)]
pub struct ObjectControllerEngine(
    #[wasm_bindgen(skip)] pub ObjectControllerEngineInner,
);
