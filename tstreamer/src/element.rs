pub mod prompt;

use crate::{info::Info, new::New};

/// A trait for elements in a [`Pipeline`].
pub trait Element: New + Info {}

static_assertions::assert_obj_safe!(Info);
