use leptos::{attr::Attribute, prelude::*};

const TEXTAREA_CLASS: &str = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

/// Returns classes for a styled `<textarea>`.
///
/// The Shadcn `<TextArea/>` isn't reactive, so we just style a plain HTML `<textarea>` instead.
pub fn textarea_attrs(extra_classes: &str) -> impl Attribute {
    let full_class = format!("{} {}", extra_classes, TEXTAREA_CLASS);

    view! {
        <{..} class=full_class />
    }
}

pub const BUTTON_CLASS: &str = "h-9 rounded-md px-3 inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
pub fn button_attrs(extra_classes: impl Fn() -> String + 'static) -> impl Attribute {
    move || {
        let full_class = format!("{} {}", extra_classes(), BUTTON_CLASS);
        view! { <div class=full_class /> } // placeholder element
    }
}
