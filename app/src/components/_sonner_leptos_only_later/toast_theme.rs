#[derive(Clone)]
pub enum ToastTheme {
    Light,
    Dark,
}

impl std::fmt::Display for ToastTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToastTheme::Light => write!(f, "light"),
            ToastTheme::Dark => write!(f, "dark"),
        }
    }
}
