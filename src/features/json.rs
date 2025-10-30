use leptos::{prelude::*, reactive::spawn_local};
use leptos_meta::Title;
use leptos_shadcn_button::Button;
use leptos_shadcn_label::Label;
use leptos_shadcn_textarea::Textarea;
use serde_json::{Value, to_string_pretty};

use crate::components::{code_result::CodeResult, copy_button::CopyButton, toolbar::Toolbar};

// Copy this in to test:
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
        if !form_touched.get() {
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
        <div class="flex flex-row gap-x-4 h-full">
            <Title text="JSON Parser and Formatter"/>
            // Input textarea
            <feature-input class="flex-1 flex flex-col h-full">
                <Toolbar>
                    <Label>
                        "Input"
                    </Label>
                </Toolbar>
                <Textarea
                    class=Some("flex-1 w-full resize-none font-mono".to_string())
                    value=Some(input.get())
                    on_change=Callback::new(move |val: String| {
                        set_form_touched.set(true);
                        set_input.set(val);
                    })
                    placeholder=EXAMPLE_JSON
                />
            </feature-input>
            // Result area
            <feature-result class="flex flex-col flex-1 h-full w-full min-w-0">
                <Toolbar>
                    <Label>
                        "Output"
                    </Label>
                    <CopyButton
                        disabled=Signal::derive(move || result.get().is_err())
                        text_to_copy=Signal::derive(move || result.get().unwrap_or("".to_string()))
                    />
                </Toolbar>
                <CodeResult result=result />
            </feature-result>
        </div>
    }
}
