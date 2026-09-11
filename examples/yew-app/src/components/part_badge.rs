// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::types::BumpPart;
use yew::prelude::*;

/// Props for [`PartBadge`].
#[derive(Properties, PartialEq)]
pub struct PartBadgeProps {
    /// The bump part to represent.
    pub part: BumpPart,
}

#[function_component(PartBadge)]
pub fn part_badge(props: &PartBadgeProps) -> Html {
    let label = props.part.label();
    let class = format!("part-badge {}", props.part.css_class());
    let aria = format!("Bump type: {label}");

    html! {
        <span class={class} aria-label={aria}>
            <span aria-hidden="true">{ props.part.emoji() }</span>
            { &label }
        </span>
    }
}
