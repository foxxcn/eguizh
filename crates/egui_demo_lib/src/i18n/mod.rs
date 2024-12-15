use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::HashMap;

mod translations;

static I18N: Lazy<Mutex<I18nManager>> = Lazy::new(|| {
    Mutex::new(I18nManager {
        current: I18nManager::detect_system_language(),
        translations: {
            let mut map = HashMap::new();
            map.insert(
                Language::Chinese,
                translations::zh::get_chinese_translations(),
            );
            map
        },
    })
});

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
}

pub struct I18nManager {
    current: Language,
    translations: HashMap<Language, HashMap<&'static str, &'static str>>,
}

impl I18nManager {
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
