//! 言語選択。英語を既定とし、第二言語との併記(ハイブリッド表示、既定ON)に対応する。
//! 元のTypeScript版(`i18n.ts`)と同じ仕様・同じ保存キー(localStorage)で、
//! 辞書は`i18n_data.rs`(元の辞書から生成)にある。

use crate::i18n_data::{Dict, AR, DE, EN, FA, FR, IT, JA, RU, UK};

pub const LANGS: [(&str, &str); 9] = [
    ("en", "English"),
    ("ja", "日本語"),
    ("it", "Italiano"),
    ("fr", "Français"),
    ("de", "Deutsch"),
    ("ru", "Русский"),
    ("uk", "Українська"),
    ("ar", "العربية"),
    ("fa", "فارسی"),
];

const KEY_LANG: &str = "open_runo_installer-lang";
const KEY_LANG2: &str = "open_runo_installer-lang2";
const KEY_HYBRID: &str = "open_runo_installer-hybrid";

fn dict(lang: &str) -> Dict {
    match lang {
        "ja" => JA,
        "it" => IT,
        "fr" => FR,
        "de" => DE,
        "ru" => RU,
        "uk" => UK,
        "ar" => AR,
        "fa" => FA,
        _ => EN,
    }
}

pub fn is_known(lang: &str) -> bool {
    LANGS.iter().any(|(c, _)| *c == lang)
}

pub fn is_rtl(lang: &str) -> bool {
    lang == "ar" || lang == "fa"
}

/// 1言語分の訳。無ければ英語、それも無ければキー自体を返す。
pub fn t(key: &str, lang: &str) -> String {
    let find = |d: Dict| d.iter().find(|(k, _)| *k == key).map(|(_, v)| (*v).to_string());
    find(dict(lang)).or_else(|| find(EN)).unwrap_or_else(|| key.to_string())
}

/// 表示用。ハイブリッドONで第二言語が異なり訳も異なるときだけ「1番目 / 第二」の併記にする。
pub fn t_display(key: &str, lang: &str, lang2: &str, hybrid: bool) -> String {
    let primary = t(key, lang);
    if !hybrid || lang2 == lang {
        return primary;
    }
    let secondary = t(key, lang2);
    if secondary == primary {
        primary
    } else {
        format!("{primary} / {secondary}")
    }
}

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn load(key: &str) -> Option<String> {
    storage()?.get_item(key).ok().flatten()
}

pub fn save_lang(lang: &str) {
    if let Some(s) = storage() {
        let _ = s.set_item(KEY_LANG, lang);
    }
    apply_direction(lang);
}

pub fn save_lang2(lang: &str) {
    if let Some(s) = storage() {
        let _ = s.set_item(KEY_LANG2, lang);
    }
}

pub fn save_hybrid(on: bool) {
    if let Some(s) = storage() {
        let _ = s.set_item(KEY_HYBRID, if on { "1" } else { "0" });
    }
}

pub fn load_lang() -> String {
    load(KEY_LANG).filter(|l| is_known(l)).unwrap_or_else(|| "en".to_string())
}

pub fn load_lang2() -> String {
    load(KEY_LANG2).filter(|l| is_known(l)).unwrap_or_else(|| "ja".to_string())
}

pub fn load_hybrid() -> bool {
    load(KEY_HYBRID).map(|v| v == "1").unwrap_or(true)
}

/// RTL言語(アラビア語・ペルシャ語)では文書全体の向きを右から左にする。
pub fn apply_direction(lang: &str) {
    if let Some(el) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
        let _ = el.set_attribute("dir", if is_rtl(lang) { "rtl" } else { "ltr" });
        let _ = el.set_attribute("lang", lang);
    }
}
