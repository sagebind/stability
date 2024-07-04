#![forbid(unreachable_pub)]

//! This is an example library demonstrating various attributes from the
//! stability crate.

/// This function does something really risky!
///
/// Don't use it yet!
#[stability::unstable(feature = "risky-function", issue = "#101")]
pub fn risky_function() {
    unimplemented!()
}

/// This struct does something really risky!
///
/// Don't use it yet!
#[stability::unstable(feature = "risky-struct", issue = "#102")]
pub struct RiskyStruct {
    pub x: u8,
}

mod private {
    /// This function does something really risky!
    ///
    /// Don't use it yet!
    #[stability::unstable(feature = "risky-private-function")]
    pub fn risky_private_function() {
        unimplemented!()
    }
}

#[allow(unused_imports)]
#[stability::unstable(feature = "risky-private-function")]
pub use private::risky_private_function;
