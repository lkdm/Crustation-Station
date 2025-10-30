use leptos::prelude::*;
use leptos_meta::Title;
use leptos_shadcn_textarea::Textarea;
use serde_json::{Value, to_string_pretty};

// {"foo":"bar","baz":["qux"]}

const EXAMPLE_JSON: &str = &"{\"foo\":\"bar\",\"baz\":[\"qux\"]}";
const EXAMPLE_RESULT: &str = r#"{
  "baz": [
    "qux"
  ],
  "foo": "bar"
}"#;

#[component]
pub fn JsonParserFormatter() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (result, set_result) = signal::<Result<String, String>>(Ok(EXAMPLE_RESULT.to_string()));
    let (form_touched, set_form_touched) = signal(false);

    Effect::new(move |_| {
        let text = input.get();
        if (!form_touched.get()) {
            // If user has not touched the form, don't change the result
            return;
        }
        if text.is_empty() {
            // But if the user has touched the form, and it's empty, change the result to nothing
            return set_result.set(Ok(String::new()));
        }
        let parsed: Result<Value, serde_json::Error> = serde_json::from_str(&text);
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
            <Title text="JSON Parser and Formatter"/>
            // Input textarea
            <feature-input class="flex-1 h-full">
                <Textarea
                    class=Some("h-full w-full resize-none".to_string())
                    value=Some(input.get())
                    on_change=Callback::new(move |val: String| {
                        set_form_touched.set(true);
                        set_input.set(val);
                    })
                    placeholder=EXAMPLE_JSON
                />
            </feature-input>
            // Result area
            <feature-result class="flex-1 h-full border border-border rounded-md overflow-auto p-4 bg-secondary text-secondary-foreground">
                <pre class="h-full w-full font-mono text-xs">
                    {move || match result.get() {
                        Ok(val) => val.clone(),
                        Err(err) => err,
                    }}
                </pre>
            </feature-result>
        </div>
    }
}
