pub trait WebView
where
    Self: Sized,
{
    fn state_from_string(s: &str) -> Result<Self, String>;
    fn step(&mut self, input: &str) -> Result<Self, String>;
    fn result(&self) -> Result<String, String>;
}
