use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_model_api::WebView;

thread_local! {
    static MACHINE: RefCell<Vec<Box<dyn WebView>>> = RefCell::new(vec![]);
}

#[cfg(feature = "model_b")]
fn create_model(initial_state: String) -> Result<Box<dyn WebView>, JsValue> {
    let model =
        model_b_web::create_model_b_web_view(initial_state).map_err(|e| JsValue::from_str(&e))?;
    Ok(Box::new(model))
}

#[cfg(not(feature = "model_b"))]
fn create_model(initial_count: String) -> Result<Box<dyn WebView>, JsValue> {
    // we use model_a as the default
    let model =
        model_a_web::create_model_a_web_view(initial_count).map_err(|e| JsValue::from_str(&e))?;
    Ok(Box::new(model))
}

#[wasm_bindgen]
pub fn create_view(initial: String) -> Result<usize, JsValue> {
    let model = create_model(initial)?;
    MACHINE.with(|machine| {
        let mut machine = machine.borrow_mut();
        machine.push(model);
        Ok(machine.len() - 1)
    })
}
