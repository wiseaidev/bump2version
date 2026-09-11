// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use yew::prelude::*;

/// Props for [`Header`].
#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    /// Whether the mobile sidebar is currently open.
    pub sidebar_open: bool,
    /// Callback invoked when the sidebar toggle button is pressed.
    pub on_sidebar_toggle: Callback<()>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let on_toggle = {
        let cb = props.on_sidebar_toggle.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    html! {
        <header class="app-header" role="banner">
            <div class="header-brand" aria-label="bump2version logo and title">
                <div class="header-logo" aria-hidden="true">{ "⬆️" }</div>
                <div>
                    <h1 class="header-title">{ "bump2version" }</h1>
                    <p class="header-subtitle">{ "Browser-side version bumper · WebAssembly" }</p>
                </div>
            </div>
            <nav class="header-actions" aria-label="External links and controls">
                <a
                    href="https://github.com/wiseaidev/bump2version"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="btn btn-ghost"
                    style="padding: .3rem .6rem; font-size: .75rem;"
                    aria-label="bump2version on GitHub (opens in new tab)"
                    title="GitHub"
                >
                    { "GitHub" }
                </a>
                <a
                    href="https://docs.rs/bump2version"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="btn btn-ghost"
                    style="padding: .3rem .6rem; font-size: .75rem;"
                    aria-label="bump2version on docs.rs (opens in new tab)"
                    title="docs.rs"
                >
                    { "Docs" }
                </a>
                <div class="header-divider" aria-hidden="true" />
                <span class="badge">{ "WASM" }</span>
                <span class="badge">{ "safe Rust" }</span>
                <div class="header-divider" aria-hidden="true" />
                <button
                    class="btn btn-ghost"
                    onclick={on_toggle}
                    aria-label={if props.sidebar_open { "Close configuration panel" } else { "Open configuration panel" }}
                    aria-expanded={props.sidebar_open.to_string()}
                    aria-controls="bump-sidebar"
                    style="padding: .35rem .7rem; font-size: .8rem;"
                >
                    { "⚙ Config" }
                </button>
            </nav>
        </header>
    }
}
