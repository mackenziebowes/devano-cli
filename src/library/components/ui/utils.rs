pub struct UiComponent {
    pub name: &'static str,
    pub description: &'static str,
    #[allow(dead_code)]
    pub long_description: &'static str,
    pub filename: &'static str,
    pub contents: &'static str,
    pub required_npm: &'static [&'static str],
}