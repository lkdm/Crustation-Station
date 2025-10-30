use leptos::{prelude::*, reactive::spawn_local};
use leptos_meta::Title;
use leptos_shadcn_button::Button;
use leptos_shadcn_label::Label;
use leptos_shadcn_textarea::Textarea;
use serde_json::{Value, to_string_pretty};

use crate::components::toolbar::Toolbar;

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
    let (result_copied, set_result_copied) = signal(false);
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
        set_result_copied.set(false);

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

    // TODO: Move with Copy button into its own component
    // This Tailwind Button component is broken.
    // When the disabled attribute is enabled it puts`disabled=""` into the HTML element.
    // The problem is that this does not work with the CSS disabled attributes.
    // So our solution is to manually set new classes.
    // TODO: Show that the button has been pressed
    let is_error = Memo::new(move |_| result.get().is_err());
    let button_class = Memo::new(move |_| {
        if is_error.get() {
            "w-24 bg-secondary text-muted-foreground cursor-not-allowed".to_string()
        } else {
            "w-24".to_string()
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
                    <Button
                        class=button_class
                        disabled=is_error
                        on_click=Callback::new(move |_| {
                            match result.get() {
                                Ok(val) => {
                                    // Get the window from web_sys
                                    let window = web_sys::window().unwrap();
                                    let navigator = window.navigator();
                                    let clipboard = navigator.clipboard();
                                    let _ = clipboard.write_text(&val);

                                    // Handles setting the label briefly
                                    set_result_copied.set(true)
                                }
                                Err(_) => {}
                            }
                        })
                    >
                        {move || if result_copied.get() { view! { "Copied!"} } else { view! { "Copy" } }}
                    </Button>
                </Toolbar>
                <pre class="h-full w-full font-mono text-sm flex-1 border border-border rounded-md overflow-auto p-4 bg-secondary text-secondary-foreground">
                    {move || match result.get() {
                        Ok(val) => val.clone(),
                        Err(err) => err,
                    }}
                </pre>
            </feature-result>
        </div>
    }
}
