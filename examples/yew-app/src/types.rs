// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The version component to bump.
#[derive(Debug, Clone, PartialEq)]
pub enum BumpPart {
    Patch,
    Minor,
    Major,
    Custom(String),
}

impl BumpPart {
    /// Returns a static label suitable for display and ARIA attributes.
    pub fn label(&self) -> String {
        match self {
            Self::Patch => "patch".into(),
            Self::Minor => "minor".into(),
            Self::Major => "major".into(),
            Self::Custom(s) => s.clone(),
        }
    }

    /// Returns the CSS modifier class for the badge.
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Patch => "part-badge--patch",
            Self::Minor => "part-badge--minor",
            Self::Major => "part-badge--major",
            Self::Custom(_) => "part-badge--custom",
        }
    }

    /// Returns a Unicode emoji used in the avatar and badge.
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Patch => "🩹",
            Self::Minor => "✨",
            Self::Major => "🚀",
            Self::Custom(_) => "🔧",
        }
    }

    /// Returns all fixed (non-custom) bump parts.
    pub fn standard() -> Vec<BumpPart> {
        vec![Self::Patch, Self::Minor, Self::Major]
    }

    /// Returns descriptive text for the preset sidebar.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Patch => "Bug fixes, no API changes",
            Self::Minor => "New features, backwards compatible",
            Self::Major => "Breaking changes",
            Self::Custom(_) => "Custom named component",
        }
    }
}

/// A single bump operation recorded in the history.
#[derive(Debug, Clone, PartialEq)]
pub struct BumpRecord {
    pub id: usize,
    pub old_version: String,
    pub new_version: String,
    pub part: BumpPart,
}
