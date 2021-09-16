mod gen;

pub use gen::Format;
pub use gen::Generator;
pub use gen::VersionType;
pub use qrcode::EcLevel;
pub use qrcode::QrCode;
pub use qrcode::QrResult;
pub use qrcode::Version;

pub const TEMPLATE: &str = include_str!("../../templates/base.html");
pub const HELP: &str = include_str!("../../README.txt");

use lazy_static::lazy_static;

lazy_static! {
    pub static ref HTML_HELP: String = txt_to_html(HELP);
}

fn txt_to_html(txt: &str) -> String {
    let mut result = String::new();
    for word in txt.split(' ') {
        if word.starts_with("http://")
            || word.starts_with("https://")
            || word.starts_with("qrcode.show")
            || word.starts_with("qrqr.show")
        {
            let link = word.split_whitespace().next().unwrap_or_default();
            if link.starts_with("http://") || link.starts_with("https://") {
                result.push_str(&format!(
                    r#"<a href="{link}">{link}</a>"#,
                    link = link
                ));
            } else {
                result.push_str(&format!(
                    r#"<a href="//{link}">{link}</a>"#,
                    link = link
                ));
            };

            result.push_str(&word.replacen(link, "", 1));
        } else {
            result.push_str(word);
        }

        result.push(' ');
    }

    result
}
