//! Core internationalization management functionality.
//!
//! The `I18nManager` provides a global translation system that:
//! - Maps UI text from English to other languages
//! - Detects system language settings
//! - Allows runtime language switching
//!
//! # Example
//! ```no_run
//! use egui_i18n::{tr, I18nManager, Language};
//!
//! // Switch to Chinese
//! I18nManager::set_global_language(Language::Chinese);
//!
//! // Use tr!() macro to wrap text that needs translation
//! assert_eq!(tr!("File"), "文件");
//! assert_eq!(tr!("Edit"), "编辑");
//! ```
use crate::Language;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::HashMap;

static I18N: Lazy<Mutex<I18nManager>> = Lazy::new(|| Mutex::new(I18nManager::new()));
/// Manages internationalization (i18n) for the application.
///
/// # Adding support for a new language
///
/// 1. Add the language to the `Language` enum:
/// ``` no_run
/// pub enum Language {
///     English,
///     Chinese,
///     Japanese,  // New language
/// }
/// ```
///
/// 2. Create a new translation file (e.g., `translations/ja.rs`):
/// ``` no_run
/// pub fn get_translations() -> HashMap<&'static str, &'static str> {
///     let mut trans = HashMap::new();
///     trans.insert("File", "ファイル");
///     trans.insert("Edit", "編集");
///     trans
/// }
/// ```
///
/// 3. Register the translations in `I18nManager::new()`:
/// ``` no_run
/// fn new() -> Self {
///     Self {
///         current: Self::detect_system_language(),
///         translations: {
///             let mut map = HashMap::new();
///             map.insert(Language::Chinese, translations::zh::get_translations());
///             map.insert(Language::Japanese, translations::ja::get_translations());
///             map
///         },
///     }
/// }
/// ```
///
/// 4. Add language detection in `detect_system_language()`:
/// ``` no_run
/// fn detect_system_language() -> Language {
///     #[cfg(not(target_arch = "wasm32"))]
///     if let Ok(lang) = std::env::var("LANG") {
///         if lang.starts_with("zh") {
///             return Language::Chinese;
///         }
///         if lang.starts_with("ja") {
///             return Language::Japanese;
///         }
///     }
///     Language::English
/// }
/// ```
pub struct I18nManager {
    current: Language,
    translations: HashMap<Language, HashMap<&'static str, &'static str>>,
}

impl I18nManager {
    fn new() -> Self {
        Self {
            current: Self::detect_system_language(),
            translations: {
                let mut map = HashMap::new();
                map.insert(
                    Language::Chinese,
                    crate::translations::zh::get_translations(),
                );
                map
            },
        }
    }

    pub fn tr(text: &'static str) -> &'static str {
        let i18n = I18N.lock();
        if i18n.current == Language::English {
            return text;
        }
        i18n.translations
            .get(&i18n.current)
            .and_then(|m| m.get(text))
            .copied()
            .unwrap_or(text)
    }

    pub fn set_global_language(lang: Language) {
        let mut i18n = I18N.lock();
        i18n.current = lang;
    }

    pub fn current_language() -> Language {
        I18N.lock().current
    }

    // todo! need to auto detect environment language
    fn detect_system_language() -> Language {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(lang) = std::env::var("LANG") {
                if lang.starts_with("zh") {
                    return Language::Chinese;
                }
            }
        }
        Language::English
    }
}
