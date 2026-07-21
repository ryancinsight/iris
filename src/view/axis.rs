//! Borrow-or-own axis metadata.

use alloc::borrow::Cow;

/// Axis semantics independent of any plotting backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Axis<'a> {
    label: Cow<'a, str>,
    unit: Option<Cow<'a, str>>,
}

impl<'a> Axis<'a> {
    /// Construct a unitless axis.
    #[must_use]
    pub fn unitless(label: impl Into<Cow<'a, str>>) -> Self {
        Self {
            label: label.into(),
            unit: None,
        }
    }

    /// Construct an axis with an explicit display unit.
    #[must_use]
    pub fn with_unit(label: impl Into<Cow<'a, str>>, unit: impl Into<Cow<'a, str>>) -> Self {
        Self {
            label: label.into(),
            unit: Some(unit.into()),
        }
    }

    /// Borrow the axis label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Borrow the optional unit label.
    #[must_use]
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }
}
