use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_model_api::WebView;

thread_local! {
    static MACHINE: RefCell<Vec<Box<dyn WebView>>> =
        RefCell::new(vec![]);
}

#[wasm_bindgen]
pub fn create_model_a_web_view(initial_count: String) -> Result<usize, JsValue> {
    let model =
        model_a_web::create_model_a_web_view(initial_count).map_err(|e| JsValue::from_str(&e))?;
    MACHINE.with(|machine| {
        let mut machine = machine.borrow_mut();
        machine.push(Box::new(model));
        Ok(machine.len() - 1)
    })
}
