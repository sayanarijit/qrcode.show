use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::ColorType;
use image::Luma;
use qrcode::render::svg;
use qrcode::render::unicode;
use qrcode::types::QrError;
use qrcode::EcLevel;
use qrcode::QrCode;
use qrcode::QrResult;
use qrcode::Version;

use image::EncodableLayout;

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Svg,
    Html,
    Unicode,
    PlainText,
    Png,
    Jpeg,
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
            "image/png" => Self::Png,
            "image/jpeg" => Self::Jpeg,
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

    pub quiet_zone: Option<bool>,
}

impl Generator {
    pub fn generate(&self, input: &[u8]) -> QrResult<Vec<u8>> {
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
            Format::Svg | Format::Html => {
                let mut bytes = code
                    .render()
                    .min_dimensions(
                        self.min_width.unwrap_or(240),
                        self.min_height.unwrap_or(240),
                    )
                    .dark_color(svg::Color(
                        self.dark_color.as_deref().unwrap_or("#000"),
                    ))
                    .light_color(svg::Color(
                        self.light_color.as_deref().unwrap_or("#fff"),
                    ))
                    .quiet_zone(self.quiet_zone.unwrap_or(true))
                    .build()
                    .into_bytes();
                bytes.push(b'\n');
                bytes
            }

            Format::Png => {
                let image = code
                    .render::<Luma<u8>>()
                    .min_dimensions(
                        self.min_width.unwrap_or(240),
                        self.min_height.unwrap_or(240),
                    )
                    .quiet_zone(self.quiet_zone.unwrap_or(true))
                    .build();
                let bytes = image.as_bytes();
                let mut result: Vec<u8> = Default::default();
                let encoder = PngEncoder::new(&mut result);
                encoder
                    .encode(bytes, image.width(), image.height(), ColorType::L8)
                    .map_err(|_| QrError::UnsupportedCharacterSet)?;
                result
            }

            Format::Jpeg => {
                let image = code
                    .render::<Luma<u8>>()
                    .min_dimensions(
                        self.min_width.unwrap_or(240),
                        self.min_height.unwrap_or(240),
                    )
                    .quiet_zone(self.quiet_zone.unwrap_or(true))
                    .build();
                let bytes = image.as_bytes();
                let mut result: Vec<u8> = Default::default();
                let mut encoder = JpegEncoder::new(&mut result);
                encoder
                    .encode(bytes, image.width(), image.height(), ColorType::L8)
                    .map_err(|_| QrError::UnsupportedCharacterSet)?;
                result
            }

            Format::PlainText => {
                let mut bytes = code
                    .render::<char>()
                    .module_dimensions(2, 1)
                    .quiet_zone(self.quiet_zone.unwrap_or(true))
                    .build()
                    .into_bytes();
                bytes.push(b'\n');
                bytes
            }

            Format::Unicode => {
                let mut bytes = code
                    .render::<unicode::Dense1x2>()
                    .min_dimensions(
                        self.min_width.unwrap_or(20),
                        self.min_height.unwrap_or(20),
                    )
                    .dark_color(unicode::Dense1x2::Dark)
                    .light_color(unicode::Dense1x2::Light)
                    .quiet_zone(self.quiet_zone.unwrap_or(true))
                    .build()
                    .into_bytes();
                bytes.push(b'\n');
                bytes
            }
        };

        Ok(image)
    }
}
