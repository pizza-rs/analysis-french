#![cfg_attr(not(feature = "std"), no_std)]
//! French language analysis for Pizza search engine.
//!
//! Provides a full-featured French analyzer with elision removal,
//! light stemming, and stop words matching Lucene/ES conventions.
//!
//! # Components
//!
//! - [`FrenchElisionFilter`] — Removes French article elisions (l', d', qu', etc.)
//! - [`FrenchLightStemFilter`] — Light suffix-stripping stemmer
//! - [`FrenchMinimalStemFilter`] — Minimal stemmer (plural/gender only)
//! - [`FrenchStopFilter`] — French stop words filter
extern crate alloc;
mod elision;
mod stem;
mod stop;

pub mod register;

pub use elision::FrenchElisionFilter;
pub use register::register_all;
pub use stem::{FrenchLightStemFilter, FrenchMinimalStemFilter};
pub use stop::FrenchStopFilter;
