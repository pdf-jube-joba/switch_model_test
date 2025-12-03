use model_b::ModelB;
use web_model_api::WebView;

pub struct ModelBWebView {
    model: ModelB,
}

impl WebView for ModelBWebView {
    fn current(&self) -> Result<serde_json::Value, String> {
        Ok(serde_json::json!(self.model.current()))
    }

    fn step(&mut self, _input: serde_json::Value) -> Result<serde_json::Value, String> {
        let output = self.model.step();
        Ok(serde_json::json!(output))
    }

    fn output_if_terminal(&self) -> Result<Option<serde_json::Value>, String> {
        Ok(None)
    }
}

pub fn create_model_b_web_view(initial_state: String) -> Result<ModelBWebView, String> {
    let state = initial_state
        .parse::<bool>()
        .map_err(|e| format!("Invalid bool for model_b: {e}"))?;
    Ok(ModelBWebView {
        model: ModelB::new(state),
    })
}
