use axum::{
    async_trait, body::Bytes, body::Full, extract::FromRequest,
    extract::OriginalUri, extract::RawBody, extract::RequestParts,
    handler::get, http::header, http::header::HeaderValue, http::Response,
    http::StatusCode, response::IntoResponse, service, Router,
};
use qrcode::render::svg;
use qrcode::render::unicode;
use qrcode::QrCode;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};

const HELP: &str = include_str!("../README.txt");
const TEMPLATE: &str = include_str!("./templates/base.html");

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

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info")
    }

    tracing_subscriber::fmt::init();

    let app =
        Router::new()
            .nest("/", get(get_handler).post(post_handler))
            .nest(
                "/favicon.ico",
                service::get(ServeDir::new("./src/static/__res__"))
                    .handle_error(|error: std::io::Error| {
                        Ok::<_, Infallible>((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }),
            )
            .nest(
                "/manifest.json",
                service::get(ServeDir::new("./src/static/__res__"))
                    .handle_error(|error: std::io::Error| {
                        Ok::<_, Infallible>((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }),
            )
            .nest(
                "/browserconfig.xml",
                service::get(ServeDir::new("./src/static/__res__"))
                    .handle_error(|error: std::io::Error| {
                        Ok::<_, Infallible>((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }),
            )
            .nest(
                "/__res__",
                service::get(ServeDir::new("./src/static/__res__"))
                    .handle_error(|error: std::io::Error| {
                        Ok::<_, Infallible>((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }),
            )
            .layer(TraceLayer::new_for_http())
            .check_infallible();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_header<B>(
    req: &RequestParts<B>,
    key: header::HeaderName,
) -> Option<Vec<String>> {
    req.headers()
        .and_then(|h| h.get(key))
        .and_then(|v| v.to_str().ok())
        .map(|s| {
            s.split(',')
                .map(|part| {
                    if let Some((_, first)) = part.split_once(';') {
                        first
                    } else {
                        part
                    }
                })
                .map(str::trim)
                .map(String::from)
                .collect()
        })
}

struct Accept(Option<Vec<String>>);

#[async_trait]
impl<B> FromRequest<B> for Accept
where
    B: Send, // required by `async_trait`
{
    type Rejection = StatusCode;

    async fn from_request(
        req: &mut RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(get_header(req, header::ACCEPT)))
    }
}

enum QRResponse {
    Plain(String),
    Html(String),
    Svg(String),
    Unicode(String),
}

impl IntoResponse for QRResponse {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        match self {
            Self::Plain(text) => {
                let mut res = Response::new(text.into());
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("text/plain"),
                );
                res
            }

            Self::Svg(svg) => {
                let mut res = Response::new(svg.into());
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("image/svg+xml"),
                );
                res
            }

            Self::Html(html) => {
                let mut res = Response::new(html.into());
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("text/html"),
                );
                res
            }

            Self::Unicode(data) => {
                let mut res = Response::new(data.into());
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static(
                        "application/octet-stream",
                    ),
                );
                res
            }
        }
    }
}

async fn post_handler(
    OriginalUri(uri): OriginalUri,
    Accept(accept): Accept,
    RawBody(body): RawBody,
) -> Result<QRResponse, StatusCode> {
    let (_, path) = uri.path().split_once('/').unwrap_or_default();
    if !path.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    };

    let bytes = hyper::body::to_bytes(body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if bytes.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let accept = accept
        .and_then(|a| {
            a.into_iter()
                .find(|a| a == "image/svg+xml" || a == "text/html")
                .map(String::from)
        })
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    let code = QrCode::new(bytes).or_else(|_| Err(StatusCode::BAD_REQUEST))?;

    match accept.as_str() {
        "image/svg+xml" => {
            let image = code
                .render()
                .min_dimensions(200, 200)
                .dark_color(svg::Color("#000"))
                .light_color(svg::Color("#fff".into()))
                .build();

            Ok(QRResponse::Svg(image.into()))
        }

        "text/html" => {
            let image = code
                .render()
                .min_dimensions(200, 200)
                .dark_color(svg::Color("#000"))
                .light_color(svg::Color("#fff".into()))
                .build();

            let html = TEMPLATE
                .replace("{{ content }}", &image)
                .replace("{{ help }}", &txt_to_html(HELP));
            Ok(QRResponse::Html(html))
        }

        _ => {
            let image = code
                .render::<unicode::Dense1x2>()
                .min_dimensions(20, 20)
                .dark_color(unicode::Dense1x2::Dark)
                .light_color(unicode::Dense1x2::Light)
                .build();

            Ok(QRResponse::Unicode(format!("{}\n", image)))
        }
    }
}

async fn get_handler(
    OriginalUri(uri): OriginalUri,
    Accept(accept): Accept,
) -> Result<QRResponse, StatusCode> {
    let (_, path) = uri.path().split_once('/').unwrap_or_default();

    let accept = accept
        .and_then(|a| {
            a.into_iter()
                .find(|a| a == "image/svg+xml" || a == "text/html")
                .map(String::from)
        })
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    if path.is_empty() {
        return match accept.as_str() {
            "text/html" => {
                let html = TEMPLATE
                    .replace("{{ content }}", "")
                    .replace("{{ help }}", &txt_to_html(HELP));
                Ok(QRResponse::Html(html))
            }
            _ => Ok(QRResponse::Plain(HELP.into())),
        };
    }

    let input = uri
        .query()
        .map(|q| format!("{}?{}", path, q))
        .unwrap_or_else(|| path.to_string());

    let code = QrCode::new(input.as_bytes())
        .or_else(|_| Err(StatusCode::BAD_REQUEST))?;

    match accept.as_str() {
        "image/svg+xml" => {
            let image = code
                .render()
                .min_dimensions(200, 200)
                .dark_color(svg::Color("#000"))
                .light_color(svg::Color("#fff".into()))
                .build();

            Ok(QRResponse::Svg(image.into()))
        }

        "text/html" => {
            let image = code
                .render()
                .min_dimensions(200, 200)
                .dark_color(svg::Color("#000"))
                .light_color(svg::Color("#fff".into()))
                .build();

            let html = TEMPLATE
                .replace("{{ content }}", &image)
                .replace("{{ help }}", &txt_to_html(HELP));
            Ok(QRResponse::Html(html))
        }

        _ => {
            let image = code
                .render::<unicode::Dense1x2>()
                .min_dimensions(20, 20)
                .dark_color(unicode::Dense1x2::Dark)
                .light_color(unicode::Dense1x2::Light)
                .build();

            Ok(QRResponse::Unicode(format!("{}\n", image)))
        }
    }
}
