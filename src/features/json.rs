use leptos::prelude::*;
use leptos_shadcn_textarea::Textarea;

#[component]
pub fn JsonParserFormatter() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (result, set_result) = signal::<Result<String, String>>(Ok(String::new()));

    view! {
        <div class="flex flex-col gap-2">
            // Input textarea
            <app-input class="flex-1">
                <Textarea
                    class=Some("h-full w-full".to_string())
                    value=Some(input.get())
                    on_change=Callback::new(move |val: String| set_input.set(val))
                />
            </app-input>
            // Result area
            <app-result class="flex-1">
                {move || match result.get() {
                    Ok(pretty) => view! {
                        <Textarea
                            class=Some("h-full w-full".to_string())
                            value=Some(pretty.clone())
                            disabled=true
                        />
                    }
                    // https://github.com/leptos-rs/leptos/discussions/1276
                    .into_any(),
                    Err(err) => view! {
                        <p class="text-red-500">{err}</p>
                    }
                    // https://github.com/leptos-rs/leptos/discussions/1276
                    .into_any(),
                }}
            </app-result>
        </div>
    }
}
