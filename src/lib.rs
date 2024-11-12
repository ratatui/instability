//! This crate provides attribute macros for specifying API stability of public API items of a
//! crate.
//!
//! The Rust standard library has a concept of [API stability] and custom attributes for managing
//! that on a per-item basis, but most of these attributes are not available for normal crates to
//! use, with the exception of the [`deprecated`] attribute. This crate seeks to provide similar
//! attributes on stable Rust, though tuned more toward what the needs of normal crate authors.
//!
//! For complete examples of how to use this crate, check out the source code for the
//! [`instability-example`] crate in the repository
//!
//! Currently, only the [`unstable`] attribute is available. Please see the documentation of that
//! macro for an explanation on what it does and how to use it.
//!
//! [API stability]: https://rustc-dev-guide.rust-lang.org/stability.html
//! [`deprecated`]:
//!     https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-deprecated-attribute
//! [`instability-example`]: https://github.com/ratatui-org/instability/tree/main/example
//! [`unstable`]: macro@unstable

use proc_macro::TokenStream;
use unstable::unstable_macro;

mod item_like;
mod unstable;

/// Mark an API as unstable.
///
/// You can apply this attribute to an item in your public API that you would like to expose to
/// users, but are not yet ready for general use. This is useful when you want to let users try out
/// some new functionality for an API you haven't finished testing or designing, or for whatever
/// reason do not want to commit any stability guarantees for.
///
/// This attribute does the following things to annotated items:
///
/// - Changes the visibility of the item from `pub` to `pub(crate)` unless a certain crate feature
///   is enabled. This ensures that internal code within the crate can always use the item, but
///   downstream consumers cannot access it unless they opt-in to the unstable API.
/// - Changes the Visibility of certain child items of the annotated item (such as struct fields) to
///   match the item's visibility. Children that are not public will not be affected.
/// - Appends an "Availability" section to the item's documentation that notes that the item is
///   unstable and indicates the name of the crate feature to enable it.
///
/// Child items of annotated modules are unchanged, as it might be desirable to be able to re-export
/// them even if the module visibility is restricted. You should apply the attribute to each item
/// within the module with the same feature name if you want to restrict the module's contents
/// itself and not just the module namespace.
///
/// Note that unlike the [`unstable`][std-unstable] attribute used in the standard library, this attribute does
/// not apply itself recursively to child items.
///
/// [std-unstable]: https://rustc-dev-guide.rust-lang.org/stability.html
///
/// Applying this attribute to non-`pub` items is pointless and does nothing.
///
/// # Arguments
///
/// The `unstable` attribute supports optional arguments that can be passed to control its behavior.
///
/// - `feature`: the name of the unstable feature that should control this item's availability. This
///   will have the string `unstable-` prepended to it. If not specified, the item will instead be
///   guarded by a catch-all `unstable` feature.
/// - `issue`: a link or reference to a tracking issue for the unstable feature. This will be
///   included in the item's documentation.
///
/// # Examples
///
/// We can apply the attribute to a public function like so:
///
/// ```
/// /// This function does something really risky!
/// ///
/// /// Don't use it yet!
/// #[instability::unstable(feature = "risky-function")]
/// pub fn risky_function() {
///     unimplemented!()
/// }
/// ```
///
/// This will essentially be expanded to the following:
///
/// ```
/// /// This function does something really risky!
/// ///
/// /// Don't use it yet!
/// ///
/// /// # Availability
/// ///
/// /// **This API is marked as unstable** and is only available when the `unstable-risky-function`
/// /// crate feature is enabled. This comes with no stability guarantees, and could be changed or
/// /// removed at any time.
/// #[cfg(feature = "unstable-risky-function")]
/// pub fn risky_function() {
///     unimplemented!()
/// }
///
/// /// This function does something really risky!
/// ///
/// /// Don't use it yet!
/// #[cfg(not(feature = "unstable-risky-function"))]
/// pub(crate) fn risky_function() {
///     unimplemented!()
/// }
/// ```
#[proc_macro_attribute]
pub fn unstable(args: TokenStream, input: TokenStream) -> TokenStream {
    unstable_macro(args.into(), input.into()).into()
}
