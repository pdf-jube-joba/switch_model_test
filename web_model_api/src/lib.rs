use serde_json::Value;

pub trait WebView {
    fn current(&self) -> Result<Value, String>;
    fn step(&mut self, input: Value) -> Result<Value, String>;
    fn output_if_terminal(&self) -> Result<Option<Value>, String>;
}
