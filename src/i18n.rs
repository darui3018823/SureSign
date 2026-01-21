use sys_locale::get_locale;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lang {
    En,
    Ja,
}

pub struct Text {
    pub lang: Lang,
}

impl Text {
    pub fn new() -> Self {
        let locale = get_locale().unwrap_or_else(|| "en-US".to_string());
        let lang = if locale.starts_with("ja") {
            Lang::Ja
        } else {
            Lang::En
        };
        Self { lang }
    }

    pub fn get(&self, key: &str) -> String {
        match self.lang {
            Lang::Ja => get_ja(key),
            Lang::En => get_en(key),
        }
    }
}

lazy_static! {
    pub static ref I18N: Mutex<Text> = Mutex::new(Text::new());
}

pub fn t(key: &str) -> String {
    let i18n = I18N.lock().unwrap();
    i18n.get(key)
}

fn get_en(key: &str) -> String {
    match key {
        "welcome" => "Welcome to SureSign - Self-Signed Certificate Generator".to_string(),
        "saved_to" => "Certificate saved to:".to_string(),
        "error" => "Error:".to_string(),
        "interactive_mode" => "Entering interactive mode...".to_string(),
        "enter_cn" => "Common Name (CN) e.g., myserver.local:".to_string(),
        "enter_san" => "Subject Alternative Names (SANs) - comma separated (e.g., 192.168.1.1, dns.local):".to_string(),
        "enter_days" => "Validity days:".to_string(),
        "generating" => "Generating certificate...".to_string(),
        "success" => "Success!".to_string(),
        "cmdlist_header" => "Available Commands & Presets:".to_string(),
        "enter_country" => "Country Name (2 letter code) [st]:".to_string(),
        "enter_state" => "State or Province Name (full name):".to_string(),
        "enter_city" => "Locality Name (eg, city):".to_string(),
        "enter_org" => "Organization Name (eg, company):".to_string(),
        "enter_org_unit" => "Organizational Unit Name (eg, section):".to_string(),
        "select_key_type" => "Select Key Type:".to_string(),
        _ => key.to_string(),
    }
}

fn get_ja(key: &str) -> String {
    match key {
        "welcome" => "SureSignへようこそ - 自己署名証明書発行ツール".to_string(),
        "saved_to" => "保存先:".to_string(),
        "error" => "エラー:".to_string(),
        "interactive_mode" => "対話モードを開始します...".to_string(),
        "enter_cn" => "コモンネーム (CN) 例: myserver.local:".to_string(),
        "enter_san" => "サブジェクト代替名 (SANs) - カンマ区切り (例: 192.168.1.1, dns.local):".to_string(),
        "enter_days" => "有効期限 (日数):".to_string(),
        "generating" => "証明書を生成中...".to_string(),
        "success" => "成功！".to_string(),
        "cmdlist_header" => "利用可能なコマンドとプリセット:".to_string(),
        "enter_country" => "国名 (2文字コード) [JP]:".to_string(),
        "enter_state" => "都道府県名:".to_string(),
        "enter_city" => "市区町村名:".to_string(),
        "enter_org" => "組織名 (会社名など):".to_string(),
        "enter_org_unit" => "部署名:".to_string(),
        "select_key_type" => "鍵の種類を選択:".to_string(),
        _ => key.to_string(),
    }
}
