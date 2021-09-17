use worker::*;

mod utils;

use libs::EcLevel;
use libs::Format;
use libs::Generator;
use libs::HELP;
use libs::HTML_HELP;
use libs::TEMPLATE;

fn log_request(req: &Request) {
    console_log!("{} - [{}]", Date::now().to_string(), req.path(),);
}

fn get_first_header_value(headers: &Headers, key: &str) -> Option<String> {
    headers
        .get(key)
        .unwrap_or_default()
        .and_then(|s| s.split(',').next().map(String::from))
        .and_then(|s| s.split(';').next().map(String::from))
}

fn generator_from_headers(headers: &Headers) -> Result<Generator> {
    let mut gen = Generator::default();

    if let Some(val) = get_first_header_value(headers, "accept") {
        gen.format = Format::from(&val);
    }

    if let Some(val) = get_first_header_value(headers, "x-qr-min-width") {
        gen.min_width =
            val.parse().map(Some).or_else(|_| Err("Bad Request"))?;
    }

    if let Some(val) = get_first_header_value(headers, "x-qr-min-height") {
        gen.min_height =
            val.parse().map(Some).or_else(|_| Err("Bad Request"))?;
    }

    if let Some(val) = get_first_header_value(headers, "x-qr-dark-color") {
        gen.dark_color = Some(format!("#{}", val));
    }

    if let Some(val) = get_first_header_value(headers, "x-qr-light-color") {
        gen.light_color = Some(format!("#{}", val));
    }

    if let Some(val) = get_first_header_value(headers, "x-qr-version-type") {
        gen.version_type = val.as_str().into();
    }

    if let Some(val) = get_first_header_value(headers, "x-qr-version-number") {
        gen.version_number =
            val.parse().map(Some).or_else(|_| Err("Bad Request"))?;
    }

    if let Some(val) = get_first_header_value(headers, "x-qr-ec-level") {
        gen.error_correction_level = match val.as_str() {
            "L" => Ok(Some(EcLevel::L)),
            "M" => Ok(Some(EcLevel::M)),
            "Q" => Ok(Some(EcLevel::Q)),
            "H" => Ok(Some(EcLevel::H)),
            _ => Err("Bad Request"),
        }?;
    }

    Ok(gen)
}

fn generate(bytes: &[u8], gen: &Generator) -> Result<Response> {
    match gen.generate(&bytes) {
        Err(_) => Response::error("Bad Request", 400),

        Ok(image) => match gen.format {
            Format::Html => {
                let html = TEMPLATE
                    .replace("{{ content }}", &image)
                    .replace("{{ help }}", &HTML_HELP);

                Response::from_html(html)
            }

            Format::Svg => {
                let mut headers = Headers::new();
                headers.set("Content-Type", "image/svg+xml")?;

                Response::ok(image).map(|r| r.with_headers(headers))
            }

            Format::Unicode => {
                Response::from_bytes(format!("{}\n", image).into())
            }

            Format::PlainText => Response::ok(format!("{}\n", image)),
        },
    }
}

#[event(fetch)]
pub async fn main(mut req: Request, _env: Env) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. The Router takes some data with its `new` method
    // that can be shared throughout all routes. If you don't need any shared data, use `()`.
    // let router = Router::new(());

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    match req.method() {
        Method::Post => {
            let bytes = req.bytes().await.unwrap();

            if bytes.is_empty() {
                Response::error("Bad Request", 400)
            } else {
                let gen = generator_from_headers(req.headers())?;
                generate(&bytes, &gen)
            }
        }

        Method::Head | Method::Get => {
            let gen = generator_from_headers(req.headers())?;

            let path = req
                .path()
                .split_once('/')
                .map(|(_, p)| p.to_string())
                .unwrap_or_default();

            if path.is_empty() {
                match gen.format {
                    Format::Html => {
                        let html = TEMPLATE
                            .replace("{{ content }}", "")
                            .replace("{{ help }}", &HTML_HELP);
                        Response::from_html(html)
                    }
                    Format::PlainText | Format::Unicode => Response::ok(HELP),
                    Format::Svg => Response::error("Bad request", 400),
                }
            } else {
                let path = req
                    .url()?
                    .query()
                    .map(|q| format!("{}?{}", path, q))
                    .unwrap_or_else(|| path.to_string());

                let bytes = path.as_bytes();
                generate(bytes, &gen)
            }
        }
        _ => Response::error("Method Not Allowed", 405),
    }
}
