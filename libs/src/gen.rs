use qrcode::render::svg;
use qrcode::render::unicode;
use qrcode::EcLevel;
use qrcode::QrCode;
use qrcode::QrResult;
use qrcode::Version;

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Svg,
    Html,
    Unicode,
    PlainText,
}

impl Default for Format {
    fn default() -> Self {
        Self::Unicode
    }
}

impl From<&str> for Format {
    fn from(headerval: &str) -> Self {
        match headerval.to_lowercase().as_str() {
            "text/html" => Self::Html,
            "image/svg+xml" => Self::Svg,
            "text/plain" => Self::PlainText,
            _ => Self::default(),
        }
    }
}

impl From<&String> for Format {
    fn from(headerval: &String) -> Self {
        headerval.as_str().into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VersionType {
    NormalVersion,
    MicroVersion,
}

impl Default for VersionType {
    fn default() -> Self {
        Self::MicroVersion
    }
}

impl From<&str> for VersionType {
    fn from(string: &str) -> Self {
        match string {
            "n" | "normal" => Self::NormalVersion,
            "m" | "micro" => Self::MicroVersion,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Generator {
    pub format: Format,

    pub min_width: Option<u32>,
    pub min_height: Option<u32>,

    pub dark_color: Option<String>,
    pub light_color: Option<String>,

    pub version_type: VersionType,
    pub version_number: Option<i16>,

    pub error_correction_level: Option<EcLevel>,
}

impl Generator {
    pub fn generate(&self, input: &[u8]) -> QrResult<String> {
        use EcLevel::*;
        use Version::*;
        use VersionType::*;

        let code = match (
            self.version_type,
            self.version_number,
            self.error_correction_level,
        ) {
            (MicroVersion, Some(n), Some(e)) => {
                QrCode::with_version(input, Micro(n), e)
            }
            (MicroVersion, Some(n), None) => {
                QrCode::with_version(input, Micro(n), L)
            }
            (NormalVersion, Some(n), Some(e)) => {
                QrCode::with_version(input, Normal(n), e)
            }
            (NormalVersion, Some(n), None) => {
                QrCode::with_version(input, Normal(n), L)
            }
            (_, _, Some(e)) => QrCode::with_error_correction_level(input, e),
            (_, _, _) => QrCode::new(input),
        };

        let code = code?;

        let image = match self.format {
            Format::Svg | Format::Html => code
                .render()
                .min_dimensions(
                    self.min_width.unwrap_or(240),
                    self.min_height.unwrap_or(240),
                )
                .dark_color(svg::Color(
                    self.dark_color
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or("#000"),
                ))
                .light_color(svg::Color(
                    self.light_color
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or("#fff"),
                ))
                .build(),

            Format::PlainText => code
                .render::<char>()
                .module_dimensions(2, 1)
                .build(),

            Format::Unicode => code
                .render::<unicode::Dense1x2>()
                .min_dimensions(
                    self.min_width.unwrap_or(20),
                    self.min_height.unwrap_or(20),
                )
                .dark_color(unicode::Dense1x2::Dark)
                .light_color(unicode::Dense1x2::Light)
                .build(),
        };

        Ok(image)
    }
}
