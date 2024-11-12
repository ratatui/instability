#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(dead_code)]

//! This is an example library demonstrating various attributes from the
//! stability crate.

/// A stable type alias
///
/// This type alias is stable
pub type StableTypeAlias = u8;

/// An unstable type alias
///
/// This type alias is unstable
#[instability::unstable(feature = "type-alias")]
pub type UnstableTypeAlias = u8;

/// A stable constant
///
/// This constant is stable
pub const STABLE_CONSTANT: u8 = 42;

/// An unstable constant
///
/// This constant is unstable
#[instability::unstable(feature = "constant")]
pub const UNSTABLE_CONSTANT: u8 = 42;

/// A stable static
///
/// This static is stable
pub static STABLE_STATIC: u8 = 42;

/// An unstable static
///
/// This static is unstable
#[instability::unstable(feature = "static")]
pub static UNSTABLE_STATIC: u8 = 42;

/// A stable function
///
/// This function is stable
pub fn stable_function() {
    unimplemented!()
}

/// An unstable function
///
/// This function is unstable
#[instability::unstable(feature = "function")]
pub fn unstable_function() {
    unimplemented!()
}

/// A stable struct
///
/// This struct is stable
pub struct StableStruct {
    pub x: u8,
}

impl StableStruct {
    /// An unstable method
    ///
    /// This method is unstable
    #[instability::unstable(feature = "method")]
    pub fn unstable_method(&self) {
        unimplemented!()
    }

    /// A stable method
    ///
    /// This method is stable
    pub fn stable_method(&self) {
        unimplemented!()
    }
}

/// An unstable struct
///
/// This struct is unstable
#[instability::unstable(feature = "struct")]
pub struct UnstableStruct {
    pub x: u8,
}

impl UnstableStruct {
    /// An unstable method
    ///
    /// This method is unstable
    #[instability::unstable(feature = "method")]
    pub fn unstable_method(&self) {
        unimplemented!()
    }

    /// A stable method
    ///
    /// This method is stable
    #[expect(
        unreachable_pub,
        reason = "The unstable macros cannot make the method pub(crate)"
    )]
    pub fn stable_method(&self) {
        unimplemented!()
    }
}

/// An unstable struct with an issue link
///
/// This struct is unstable and has an issue link.
#[instability::unstable(feature = "struct-with-issue", issue = "#123")]
pub struct UnstableStructWithIssue {
    pub x: u8,
}

/// A stable trait
///
/// This trait is stable
pub trait StableTrait {
    /// A stable trait method
    ///
    /// This method is stable.
    fn stable_trait_method(&self) {
        unimplemented!()
    }

    // Not yet supported
    // /// An unstable trait method
    // ///
    // /// This method is unstable.
    // #[instability::unstable(feature = "trait-method")]
    // fn unstable_trait_method(&self);
}

/// An unstable trait
///
/// This trait is unstable
#[instability::unstable(feature = "trait")]
pub trait UnstableTrait {
    /// A stable trait method
    ///
    /// This method is stable.
    fn stable_trait_method(&self) {
        unimplemented!()
    }

    // Not yet supported
    // /// An unstable trait method
    // ///
    // /// This method is not implemented yet.
    // #[instability::unstable(feature = "trait-method")]
    // fn unstable_trait_method(&self);
}

/// A stable enum
///
/// This enum is stable.
pub enum StableEnum {
    /// An enum variant
    ///
    /// This variant is stable.
    Variant,
}

/// An unstable enum
///
/// This enum is unstable.
#[instability::unstable(feature = "enum")]
pub enum UnstableEnum {
    /// An enum variant
    ///
    /// This variant is stable.
    Variant,
    // Not yet supported
    // /// An unstable enum variant
    // ///
    // /// This variant is not implemented yet.
    // #[instability::unstable(feature = "enum-variant")]
    // UnstableVariant,
}

/// A stable module
///
/// This module is stable.
pub mod stable {
    /// A stable function
    ///
    /// This function is stable.
    pub fn stable_function() {
        unimplemented!()
    }

    /// An unstable function
    ///
    /// This function is unstable.
    #[instability::unstable(feature = "function")]
    pub fn unstable_function() {
        unimplemented!()
    }
}

/// An unstable module
///
/// This module is unstable.
#[instability::unstable(feature = "module")]
pub mod unstable {
    /// A stable function
    ///
    /// This function is stable.
    pub fn stable_function() {
        unimplemented!()
    }

    /// An unstable function
    ///
    /// This function is unstable.
    #[instability::unstable(feature = "function")]
    pub fn unstable_function() {
        unimplemented!()
    }
}

/// A private module
///
/// This module is private.
mod private {
    /// A private function
    ///
    /// This function is private.
    pub fn private_function() {
        unimplemented!()
    }

    /// An unstable private function
    ///
    /// This function is unstable.
    #[instability::unstable(feature = "private-function")]
    pub fn unstable_private_function() {
        unimplemented!()
    }
}

/// A stable re-export of a private stable item
///
/// This re-export is stable.
pub use private::private_function as stable_reexport;

/// An unstable re-export of a private stable item
///
/// This re-export is unstable.
#[instability::unstable(feature = "reexport")]
#[expect(unused_imports)]
pub use private::private_function as unstable_reexport;

// This does not work as the unstable_private_function is only public within the crate and cannot
// be re-exported
// /// A stable reexport of a private unstable item
// ///
// /// This export is stable.
// pub use private::unstable_private_function as stable_unstable_reexport;

/// An unstable reexport of a private unstable item
///
/// This export is unstable. The availability section on this will be appended to the availability
/// section of the unstable_private_function, which will look odd. Consider avoiding re-exporting
/// unstable items like this, and instead only mark the re-export itself as unstable.
#[instability::unstable(feature = "reexport")]
#[expect(unused_imports)]
pub use private::unstable_private_function as unstable_unstable_export;
