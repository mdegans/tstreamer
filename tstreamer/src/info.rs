use std::borrow::Cow;

/// A trait for information about a type.
pub trait Info {
    /// Name of the object.
    fn name<'a>(&'a self) -> Cow<'a, str>;
    /// Description of the object.
    fn description<'a>(&'a self) -> Cow<'a, str>;
}
static_assertions::assert_obj_safe!(Info);
