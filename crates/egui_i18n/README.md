# egui-i18n

[![Latest version](https://img.shields.io/crates/v/egui-i18n.svg)](https://crates.io/crates/egui-i18n)
[![Documentation](https://docs.rs/egui-i18n/badge.svg)](https://docs.rs/egui-i18n)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![Apache](https://img.shields.io/badge/license-Apache-blue.svg)

This crate provides internationalization (i18n) support for [`egui`](https://github.com/emilk/egui) applications.

## Usage
``` rust
use egui_i18n::{tr, I18nManager, Language};
// Use tr!() macro to wrap text that needs translation
ui.label(tr!("Hello"));
// Add language switching UI
if ui.button("Switch to Chinese").clicked() {
I18nManager::set_global_language(Language::Chinese);
}
```


## Features

- Simple translation macro `tr!()`
- Global language management
- Built-in support for multiple languages
- Easy to add new language translations
