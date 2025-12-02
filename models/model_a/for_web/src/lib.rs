use model_a::ModelA;
use web_model_api::WebView;

pub struct ModelAWebView {
    model: ModelA,
}

impl WebView for ModelAWebView {
    fn current(&self) -> Result<serde_json::Value, String> {
        Ok(serde_json::json!(self.model.current()))
    }

    fn step(&mut self, _input: serde_json::Value) -> Result<serde_json::Value, String> {
        let output = self.model.step();
        Ok(serde_json::json!(output))
    }

    fn output_if_terminal(&self) -> Result<Option<serde_json::Value>, String> {
        match self.model.output_if_terminal() {
            Some(value) => Ok(Some(serde_json::json!(value))),
            None => Ok(None),
        }
    }
}

pub fn create_model_a_web_view(initial_count: String) -> Result<ModelAWebView, String> {
    let count = initial_count.parse::<i32>().map_err(|e| e.to_string())?;
    Ok(ModelAWebView {
        model: ModelA::new(count),
    })
}
