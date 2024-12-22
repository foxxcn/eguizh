//! Internationalization support for egui applications.
//!
//! # Quick Start
//! ``` no_run
//! use egui_i18n::{tr, I18nManager, Language};
//!
//! // Use tr!() macro to wrap text that needs translation
//! ui.label(tr!("Hello"));
//!
//! // Add language switching UI
//! if ui.button("Switch to Chinese").clicked() {
//!     I18nManager::set_global_language(Language::Chinese);
//! }
//! ```
//!
//! # Adding New Language Support
//! See the documentation in:
//! - `Language` enum for adding new languages
//! - `translations/mod.rs` for adding translation files
//! - `I18nManager` for implementation details
#![allow(rustdoc::private_doc_tests)]

mod language;
mod manager;
mod translations;

pub use language::Language;
pub use manager::I18nManager;

#[macro_export]
macro_rules! tr {
    ($text:expr) => {
        $crate::I18nManager::tr($text)
    };
}
