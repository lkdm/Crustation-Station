use leptos::prelude::*;
use leptos_shadcn_textarea::Textarea;

#[component]
pub fn JsonParserFormatter() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (result, set_result) = signal::<Result<String, Vec<String>>>(Ok(String::new()));

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
                <ErrorBoundary
                    fallback=|errors| view! {
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    }>
                    <Textarea
                        class=Some("h-full w-full".to_string())
                        value=Some(result.get().clone().unwrap())
                        disabled=true
                    />
                </ErrorBoundary>
            </app-result>
        </div>
    }
}
