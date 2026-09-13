# WebAssembly (WASM) Support 🌐

The `bump2version` core modules (`config`, `version`, `files`, `error`) natively compile to `wasm32-unknown-unknown`, no filesystem, no OS threads, no unsafe code. This makes them an excellent drop-in for browser-side version management tools, editor extensions, and Rust frontend apps.

## Framework Compatibility

Because the core logic has zero OS dependencies, `bump2version` works out-of-the-box with all major Rust frontend frameworks:

- **[Yew](https://yew.rs/)**
- **[Dioxus](https://dioxuslabs.com/)**
- **[Leptos](https://leptos.dev/)**
- **[Sycamore](https://sycamore.dev/)**

## 📦 Usage

Add `bump2version` with `no_std` (no `std` feature) to your WASM project's `Cargo.toml`:

```toml
[dependencies]
bump2version = { version = "0.2.2", default-features = false }
```

## Minimal Example (Yew)

The following component lets the user enter a version string and instantly see the bumped result, entirely in the browser with zero network requests:

```rust
use bump2version::config::BumpConfig;
use bump2version::version::{BumpPart, bump_version, parse_version, serialize_version};
use yew::prelude::*;

#[function_component(VersionBumper)]
pub fn version_bumper() -> Html {
    let version = use_state(|| "1.2.3".to_string());
    let result = use_state(|| "-".to_string());

    let on_bump = {
        let version = version.clone();
        let result = result.clone();
        Callback::from(move |part: &'static str| {
            let cfg = BumpConfig::default();
            if let Ok(parsed) = parse_version(&*version, &cfg) {
                if let Ok(bumped) = bump_version(&parsed, &BumpPart::from(part), &cfg) {
                    result.set(serialize_version(&bumped, &cfg));
                }
            }
        })
    };

    html! {
        <div class="bumper">
            <input
                value={(*version).clone()}
                oninput={Callback::from({
                    let version = version.clone();
                    move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        version.set(input.value());
                    }
                })}
                placeholder="e.g. 1.2.3"
            />
            <button onclick={on_bump.reform(|_| "patch")}>{ "Bump Patch" }</button>
            <button onclick={on_bump.reform(|_| "minor")}>{ "Bump Minor" }</button>
            <button onclick={on_bump.reform(|_| "major")}>{ "Bump Major" }</button>
            <p>{ format!("New version: {}", *result) }</p>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! { <VersionBumper /> }
}

// fn main() {
//     yew::Renderer::<App>::new().render();
// }
```

## Full Yew App Example

A reference implementation of a version-bump UI tool built with Yew and `bump2version` is available in [`examples/yew-app`](examples/yew-app/). It includes:

- Input field for the current version
- Patch / Minor / Major bump buttons
- Pre-release cycling (`alpha → beta → rc → stable`) via custom `BumpConfig`
- Live output of the new version string

### Running the Example

```sh
cd examples/yew-app
cargo install trunk
trunk serve --port 3000
# Open http://localhost:3000 in your browser
```

## Limitations

The WASM build excludes:

| Module    | Available in WASM? | Reason                          |
| --------- | ------------------ | ------------------------------- |
| `config`  | ✅                 | no_std + alloc                  |
| `version` | ✅                 | no_std + alloc                  |
| `files`   | ✅                 | no_std + alloc                  |
| `error`   | ✅                 | no_std + alloc                  |
| `git`     | ❌                 | requires `std::fs` and OS APIs  |
| `watch`   | ❌                 | requires OS file-system events  |
| `detect`  | ❌                 | requires `walkdir` + filesystem |
| CLI       | ❌                 | requires `std::env`             |

## Example: Building a Version Management Tool

You can use `bump2version` as the versioning backend for a browser-based release management dashboard. A reference Yew application is in [`examples/yew-app`](examples/yew-app/) demonstrating full pre-release lifecycle management in the browser.
