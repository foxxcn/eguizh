//! Supported languages in the internationalization system.
//!
//! # Adding a new language
//! 1. Add a new variant to this enum
//! 2. Create a new translation file in `translations/` directory
//! 3. Update `I18nManager::new()` to include the new translations
//! 4. Add language detection in `I18nManager::detect_system_language()`
//!
//! # Example
//! ``` ignore
//! pub enum Language {
//!     English,
//!     Chinese,
//!     Japanese,  // New language
//! }
//! impl Language {
//!     pub fn display_name(&self) -> &'static str {
//!         match self {
//!             Self::English => "English",
//!             Self::Chinese => "中文",
//!             Self::Japanese => "日本語",  // Add native name
//!         }
//!     }
//!     pub fn available_languages() -> &'static [Self] {
//!         &[Self::English, Self::Chinese, Self::Japanese]  // Add here
//!     }
//! }
//! ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
    // Japanese,  // Example of adding a new language
}

impl Language {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Chinese => "中文",
        }
    }

    pub fn available_languages() -> &'static [Self] {
        &[Self::English, Self::Chinese]
    }
}
