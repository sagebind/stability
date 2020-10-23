//! This crate provides attribute macros for specifying API stability of public
//! API items of a crate.
//!
//! The Rust standard library has a concept of [API
//! stability](https://rustc-dev-guide.rust-lang.org/stability.html) and custom
//! attributes for managing that on a per-item basis, but most of these
//! attributes are not available for normal crates to use, with the exception of
//! the
//! [`#[deprecated]`](https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-deprecated-attribute)
//! attribute. This crate seeks to provide similar attributes on stable Rust,
//! though tuned more toward what the needs of normal crate authors.
//!
//! For complete examples of how to use this crate, check out the source code
//! for the [`stability-example`
//! crate](https://github.com/sagebind/stability/tree/master/example) included
//! in the stability repository.
//!
//! Currently, only the [`#[unstable]`][macro@unstable] attribute is available.

use proc_macro::TokenStream;
use syn::{Item, parse_macro_input};

mod unstable;

/// Mark an API as unstable.
///
/// You can apply this attribute to an item in your public API that you would
/// like to expose to users, but are not yet ready for general use. This is
/// useful when you want to let users try out some new functionality for an API
/// you haven't finished testing or designing, or for whatever reason do not
/// want to commit any stability guarantees for.
///
/// This attribute does the following things to annotated items:
///
/// - Changes the visibility of the item from `pub` to `pub(crate)`, unless a
///   certain crate feature is enabled.
/// - Appends an "Availability" section to the item's documentation that notes
///   that the item is unstable, and indicates the name of the crate feature to
///   enable it.
///
/// Note that unlike the `#[unstable]` attribute used [in the standard
/// library](https://rustc-dev-guide.rust-lang.org/stability.html), this
/// attribute does not apply itself recursively to child items.
///
/// Applying this attribute to non-`pub` items is pointless and does nothing.
///
/// # Arguments
///
/// The `unstable` attribute supports optional arguments that can be passed to
/// control its behavior.
///
/// - `feature`: Specify the name of the unstable feature that should control
///   this item's availability. The crate feature will have the string
///   `unstable-` prepended to it. If not specified, it will be guarded by a
///   catch-all `unstable` feature.
///
/// # Examples
///
/// ```
/// /// This function does something really risky!
/// ///
/// /// Don't use it yet!
/// #[stability::unstable(feature = "risky-function")]
/// pub fn risky_function() {
///     unimplemented!()
/// }
/// ```
#[proc_macro_attribute]
pub fn unstable(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let attr = unstable::UnstableAttribute::from(args);

    match parse_macro_input!(item as Item) {
        Item::Enum(item_enum) => attr.expand(item_enum),
        Item::Fn(item_fn) => attr.expand(item_fn),
        Item::Mod(item_mod) => attr.expand(item_mod),
        Item::Struct(item_struct) => attr.expand(item_struct),
        Item::Trait(item_trait) => attr.expand(item_trait),
        _ => panic!("unsupported item type"),
    }
}
