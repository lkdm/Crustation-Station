use leptos::prelude::*;

#[component]
pub fn CodeResult(#[prop(into)] result: Signal<Result<String, String>>) -> impl IntoView {
    view! {
        <pre class="h-full w-full font-mono text-sm flex-1 border border-border rounded-md overflow-auto p-4 bg-secondary text-secondary-foreground">
            {move || match result.get() {
                Ok(val) => val.clone(),
                Err(err) => err,
            }}
        </pre>
    }
}
