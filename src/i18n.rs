use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use sys_locale::get_locale;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lang {
    En,
    Ja,
}

/// Embedded fallback translations (English)
fn embedded_en() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert(
        "welcome".to_string(),
        "Welcome to SureSign - Self-Signed Certificate Generator".to_string(),
    );
    m.insert("saved_to".to_string(), "Certificate saved to:".to_string());
    m.insert("error".to_string(), "Error:".to_string());
    m.insert(
        "interactive_mode".to_string(),
        "Entering interactive mode...".to_string(),
    );
    m.insert(
        "enter_cn".to_string(),
        "Common Name (CN) e.g., myserver.local:".to_string(),
    );
    m.insert(
        "enter_san".to_string(),
        "Subject Alternative Names (SANs) - comma separated (e.g., 192.168.1.1, dns.local):"
            .to_string(),
    );
    m.insert("enter_days".to_string(), "Validity days:".to_string());
    m.insert(
        "generating".to_string(),
        "Generating certificate...".to_string(),
    );
    m.insert("success".to_string(), "Success!".to_string());
    m.insert(
        "cmdlist_header".to_string(),
        "Available Commands & Presets:".to_string(),
    );
    m.insert(
        "enter_country".to_string(),
        "Country Name (2 letter code):".to_string(),
    );
    m.insert(
        "enter_state".to_string(),
        "State or Province Name (full name):".to_string(),
    );
    m.insert(
        "enter_city".to_string(),
        "Locality Name (eg, city):".to_string(),
    );
    m.insert(
        "enter_org".to_string(),
        "Organization Name (eg, company):".to_string(),
    );
    m.insert(
        "enter_org_unit".to_string(),
        "Organizational Unit Name (eg, section):".to_string(),
    );
    m.insert(
        "select_key_type".to_string(),
        "Select Key Type:".to_string(),
    );
    m.insert(
        "enter_pfx_password".to_string(),
        "PFX Password (leave empty for no password):".to_string(),
    );
    m.insert(
        "files_exist_warning".to_string(),
        "Warning: The following files already exist:".to_string(),
    );
    m.insert(
        "overwrite_prompt".to_string(),
        "Overwrite? (y/N):".to_string(),
    );
    m.insert("aborted".to_string(), "Aborted.".to_string());
    m
}

/// Embedded fallback translations (Japanese)
fn embedded_ja() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert(
        "welcome".to_string(),
        "SureSignへようこそ - 自己署名証明書発行ツール".to_string(),
    );
    m.insert("saved_to".to_string(), "保存先:".to_string());
    m.insert("error".to_string(), "エラー:".to_string());
    m.insert(
        "interactive_mode".to_string(),
        "対話モードを開始します...".to_string(),
    );
    m.insert(
        "enter_cn".to_string(),
        "コモンネーム (CN) 例: myserver.local:".to_string(),
    );
    m.insert(
        "enter_san".to_string(),
        "サブジェクト代替名 (SANs) - カンマ区切り (例: 192.168.1.1, dns.local):".to_string(),
    );
    m.insert("enter_days".to_string(), "有効期限 (日数):".to_string());
    m.insert("generating".to_string(), "証明書を生成中...".to_string());
    m.insert("success".to_string(), "成功！".to_string());
    m.insert(
        "cmdlist_header".to_string(),
        "利用可能なコマンドとプリセット:".to_string(),
    );
    m.insert(
        "enter_country".to_string(),
        "国名 (2文字コード):".to_string(),
    );
    m.insert("enter_state".to_string(), "都道府県名:".to_string());
    m.insert("enter_city".to_string(), "市区町村名:".to_string());
    m.insert("enter_org".to_string(), "組織名 (会社名など):".to_string());
    m.insert("enter_org_unit".to_string(), "部署名:".to_string());
    m.insert("select_key_type".to_string(), "鍵の種類を選択:".to_string());
    m.insert(
        "enter_pfx_password".to_string(),
        "PFXパスワード (空でパスワードなし):".to_string(),
    );
    m.insert(
        "files_exist_warning".to_string(),
        "警告: 以下のファイルが既に存在します:".to_string(),
    );
    m.insert(
        "overwrite_prompt".to_string(),
        "上書きしますか? (y/N):".to_string(),
    );
    m.insert("aborted".to_string(), "中断しました。".to_string());
    m
}

/// Load translations from JSON file, falling back to embedded translations
fn load_translations(lang: Lang) -> HashMap<String, String> {
    let filename = match lang {
        Lang::En => "locales/en.json",
        Lang::Ja => "locales/ja.json",
    };

    // Try to read from file
    if let Ok(content) = fs::read_to_string(filename) {
        if let Ok(json) = serde_json::from_str::<Value>(&content) {
            if let Some(obj) = json.as_object() {
                return obj
                    .iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect();
            }
        }
    }

    // Fallback to embedded
    match lang {
        Lang::En => embedded_en(),
        Lang::Ja => embedded_ja(),
    }
}

pub struct Text {
    pub lang: Lang,
    translations: HashMap<String, String>,
}

impl Text {
    pub fn new() -> Self {
        let locale = get_locale().unwrap_or_else(|| "en-US".to_string());
        let lang = if locale.starts_with("ja") {
            Lang::Ja
        } else {
            Lang::En
        };
        let translations = load_translations(lang);
        Self { lang, translations }
    }

    pub fn get(&self, key: &str) -> String {
        self.translations
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
}

lazy_static! {
    pub static ref I18N: Mutex<Text> = Mutex::new(Text::new());
}

pub fn t(key: &str) -> String {
    let i18n = I18N.lock().unwrap();
    i18n.get(key)
}
