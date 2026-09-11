// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod app;
mod components;
mod types;
mod version_ops;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
