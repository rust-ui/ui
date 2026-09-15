pub struct Crate {
    pub name: &'static str,
    #[allow(unused)] // TODO. Find a way to set the version to "0.8" instead of "0.8.2".
    pub version: Option<&'static str>,
    pub features: Option<&'static [&'static str]>,
}

impl Crate {
    const fn new(
        name: &'static str,
        version: Option<&'static str>,
        features: Option<&'static [&'static str]>,
    ) -> Self {
        Crate { name, version, features }
    }
}

pub const INIT_CRATES: [Crate; 4] = [
    // TODO. Handle leptos csr or ssr based on user input.
    Crate::new("leptos", None, Some(&["csr"])),
    Crate::new("tw_merge", None, Some(&["variant"])),
    Crate::new("icons", None, Some(&["leptos"])),
    Crate::new("leptos_ui", None, None),
];

pub const DIOXUS_INIT_CRATES: [Crate; 3] = [
    Crate::new("dioxus", None, Some(&["router", "fullstack"])),
    Crate::new("tw_merge", None, Some(&["debug"])),
    Crate::new("icons", None, Some(&["dioxus"])),
];
