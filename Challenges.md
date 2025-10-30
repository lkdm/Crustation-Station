Some challenges I have had with this project

- Branching in RSX panics with strange error if they are not the same type.
  - I solved this by searching Github Discussions and asking the Rust Discord.
    - Solution 1: append `into_any()` to the view macro `view! {}.into_any()`
    - Solution 2: use an `<ErrorBoundary/>`
    - My solution: They both worked okay for me. But I didn't end up using a different component for errors.
  - CSS issue: overflow inside flexbox
    - Story: Anytime I would test my JSON parser with a large blob of JSON, the
      result div would expand and eat the entire page.
    - Attempt 1: Set min-w-0 on child (failed)
    - Attempt 2: Remove `h-full`
    - Solution: Removed classes until I found the culprit- process of
      elimination. Needed to add `min-w-0` to. Similar to running `git bisect`
  - [Conversion of derived signal to ReadSignal](https://github.com/leptos-rs/leptos/discussions/4421)
    - Asked Github Discussion
    - [Found section in documentation regarding `into` Props](https://book.leptos.dev/view/03_components.html#into-props)
    - Realised they use `Signal<T>` instead of `ReadSignal<T>`
    - Turned copy button into reusable component

Other problems I solved;

I'm still working on;

- Because JS promises don't automatically map to Rust Futures, I need to
  manually convert it. Trying to figure out how to asyncronously set a timer.
- Trying to find a generalised way to make a `signal()` load from LocalStorage
  on page load, and save to it reactively.
- Want to pass a RSX component to a struct, so I can unify routes and list items
  under a single vector.
