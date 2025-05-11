#![allow(
    clippy::approx_constant,
    clippy::doc_lazy_continuation,
    clippy::double_parens,
    clippy::eq_op,
    clippy::identity_op,
    clippy::missing_safety_doc,
    clippy::needless_bool,
    clippy::needless_return,
    clippy::nonminimal_bool,
    clippy::ptr_eq,
    clippy::too_long_first_doc_paragraph,
    clippy::unnecessary_cast,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    unused_parens,
    unused_unsafe,
    unused_variables
)]

pub mod mixer;

/// Reexports of everything from the other modules
pub mod everything {
    #[doc(no_inline)]
    pub use super::mixer::*;
}
