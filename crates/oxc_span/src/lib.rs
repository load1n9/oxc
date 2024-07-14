//! Source positions and related helper functions.
//!
//! <https://doc.rust-lang.org/beta/nightly-rustc/rustc_span>
#![cfg_attr(not(feature = "std"), no_std)]

mod atom;
mod source_type;
mod span;

extern crate alloc;

pub use crate::{
    atom::{Atom, CompactStr, MAX_INLINE_LEN as ATOM_MAX_INLINE_LEN},
    source_type::{Language, LanguageVariant, ModuleKind, SourceType, VALID_EXTENSIONS},
    span::{GetSpan, Span, SPAN},
};
