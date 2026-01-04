use gtk4::{gdk, style_context_add_provider_for_display, Builder, ComboBoxText, CssProvider, Paned, ScrolledWindow};

pub struct ConsoleView {
    pub root: gtk4::Box
}

impl ConsoleView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/smudgetimer/rust/res/ui/console_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/smudgetimer/rust/res/ui/console_view.css");
        style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in console_view.ui");

        Self {
            root
        }
    }
}
