use gloo_console::log;
use leptos::prelude::*;
use leptos_shadcn_textarea::Textarea;
use serde_json::{Value, to_string_pretty};

// {"foo":"bar","baz":["qux"]}

#[component]
pub fn JsonParserFormatter() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (result, set_result) = signal::<Result<String, String>>(Ok(String::new()));

    Effect::new(move |_| {
        let text = input.get();
        log!("text", text.to_string());
        let parsed: Result<Value, serde_json::Error> = serde_json::from_str(&text);
        log!("parsed", format!("{:?}", parsed));
        match parsed {
            Ok(val) => {
                set_result.set(Ok(to_string_pretty(&val).unwrap()));
            }
            Err(err) => {
                set_result.set(Err(err.to_string()));
            }
        }
    });

    view! {
        <div class="flex flex-row gap-2 h-full">
            // Input textarea
            <app-input class="flex-1 h-full">
                <Textarea
                    class=Some("h-full w-full".to_string())
                    value=Some(input.get())
                    on_change=Callback::new(move |val: String| set_input.set(val))
                />
            </app-input>
            // Result area
            <app-result class="flex-1 h-full">
                <pre class="h-full w-full">
                    {move || match result.get() {
                        Ok(val) => val.clone(),
                        Err(err) => err,
                    }}
                </pre>
            </app-result>
        </div>
    }
}
