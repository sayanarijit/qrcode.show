use axum::{
    async_trait, body::Bytes, body::Full, extract::FromRequest,
    extract::OriginalUri, extract::RawBody, extract::RequestParts,
    handler::get, http::header, http::header::HeaderName,
    http::header::HeaderValue, http::Response, http::StatusCode,
    response::IntoResponse, service, Router,
};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};

use libs::EcLevel;
use libs::Format;
use libs::Generator;
use libs::HELP;
use libs::HTML_HELP;
use libs::TEMPLATE;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info")
    }

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .nest("/", get(get_handler).post(post_handler))
        .nest(
            "/favicon.ico",
            service::get(ServeDir::new("./static/__res__")).handle_error(
                |error: std::io::Error| {
                    Ok::<_, Infallible>((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    ))
                },
            ),
        )
        .nest(
            "/manifest.json",
            service::get(ServeDir::new("./static/__res__")).handle_error(
                |error: std::io::Error| {
                    Ok::<_, Infallible>((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    ))
                },
            ),
        )
        .nest(
            "/browserconfig.xml",
            service::get(ServeDir::new("./static/__res__")).handle_error(
                |error: std::io::Error| {
                    Ok::<_, Infallible>((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    ))
                },
            ),
        )
        .nest(
            "/__res__",
            service::get(ServeDir::new("./static/__res__")).handle_error(
                |error: std::io::Error| {
                    Ok::<_, Infallible>((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    ))
                },
            ),
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

fn get_first_header_value<B>(
    req: &RequestParts<B>,
    key: header::HeaderName,
) -> Option<String> {
    req.headers()
        .and_then(|h| h.get(key))
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.split(';').next())
        .map(String::from)
}

struct QRGenerator(Generator);

#[async_trait]
impl<B> FromRequest<B> for QRGenerator
where
    B: Send, // required by `async_trait`
{
    type Rejection = StatusCode;

    async fn from_request(
        req: &mut RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        let mut gen = Generator::default();

        if let Some(val) = get_first_header_value(&req, header::ACCEPT) {
            gen.format = Format::from(&val);
        }

        if let Some(val) = get_first_header_value(
            &req,
            HeaderName::from_static("x-qr-min-width"),
        ) {
            gen.min_width = val
                .parse()
                .map(Some)
                .or_else(|_| Err(StatusCode::BAD_REQUEST))?;
        }

        if let Some(val) = get_first_header_value(
            &req,
            HeaderName::from_static("x-qr-min-height"),
        ) {
            gen.min_height = val
                .parse()
                .map(Some)
                .or_else(|_| Err(StatusCode::BAD_REQUEST))?;
        }

        if let Some(val) = get_first_header_value(
            &req,
            HeaderName::from_static("x-qr-dark-color"),
        ) {
            gen.dark_color = Some(format!("#{}", val));
        }

        if let Some(val) = get_first_header_value(
            &req,
            HeaderName::from_static("x-qr-light-color"),
        ) {
            gen.light_color = Some(format!("#{}", val));
        }

        if let Some(val) = get_first_header_value(
            &req,
            HeaderName::from_static("x-qr-version-type"),
        ) {
            gen.version_type = val.as_str().into();
        }

        if let Some(val) = get_first_header_value(
            &req,
            HeaderName::from_static("x-qr-version-number"),
        ) {
            gen.version_number = val
                .parse()
                .map(Some)
                .or_else(|_| Err(StatusCode::BAD_REQUEST))?;
        }

        if let Some(val) = get_first_header_value(
            &req,
            HeaderName::from_static("x-qr-ec-level"),
        ) {
            gen.error_correction_level = match val.as_str() {
                "L" => Ok(Some(EcLevel::L)),
                "M" => Ok(Some(EcLevel::M)),
                "Q" => Ok(Some(EcLevel::Q)),
                "H" => Ok(Some(EcLevel::H)),
                _ => Err(StatusCode::BAD_REQUEST),
            }?;
        }

        Ok(QRGenerator(gen))
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

fn generate(bytes: &[u8], gen: &Generator) -> Result<QRResponse, StatusCode> {
    let image = gen
        .generate(&bytes)
        .or_else(|_| Err(StatusCode::BAD_REQUEST))?;

    match gen.format {
        Format::Svg => Ok(QRResponse::Svg(image.into())),

        Format::Html => {
            let html = TEMPLATE
                .replace("{{ content }}", &image)
                .replace("{{ help }}", &HTML_HELP);
            Ok(QRResponse::Html(html))
        }

        Format::Unicode => Ok(QRResponse::Unicode(format!("{}\n", image))),
    }
}

async fn post_handler(
    OriginalUri(uri): OriginalUri,
    QRGenerator(gen): QRGenerator,
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
        Err(StatusCode::BAD_REQUEST)
    } else {
        generate(&bytes, &gen)
    }
}

async fn get_handler(
    OriginalUri(uri): OriginalUri,
    QRGenerator(gen): QRGenerator,
) -> Result<QRResponse, StatusCode> {
    let (_, path) = uri.path().split_once('/').unwrap_or_default();

    if path.is_empty() {
        match gen.format {
            Format::Html => {
                let html = TEMPLATE
                    .replace("{{ content }}", "")
                    .replace("{{ help }}", &HTML_HELP);
                Ok(QRResponse::Html(html))
            }
            Format::Unicode => Ok(QRResponse::Plain(HELP.to_string())),
            Format::Svg => Err(StatusCode::BAD_REQUEST),
        }
    } else {
        let input = uri
            .query()
            .map(|q| format!("{}?{}", path, q))
            .unwrap_or_else(|| path.to_string());

        let bytes = input.as_bytes();
        generate(&bytes, &gen)
    }
}
