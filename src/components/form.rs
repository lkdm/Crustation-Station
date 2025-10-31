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
