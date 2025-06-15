use std::{
    collections::HashMap,
    error, fmt,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    num::ParseIntError,
    path::PathBuf,
};

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};

use base64_serde::base64_serde_type;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::{net::TcpListener, task::spawn_blocking};

use tower_http::trace::{self, TraceLayer};
use tracing::{Level, info};

use crate::{
    core::{error::Error, registry::get_image, types::OptionValue},
    server::tools::{
        ImageRequest, gif_reverse, gif_split, handle_image, handle_images, handle_inspect,
        process_images,
    },
};

base64_serde_type!(Base64Standard, base64::engine::general_purpose::STANDARD);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum ImageData {
    Url {
        url: String,
        headers: Option<HashMap<String, String>>,
        name: String,
    },
    Path {
        path: PathBuf,
        name: String,
    },
    Data {
        name: String,
        #[serde(with = "Base64Standard")]
        data: Vec<u8>,
    },
}

#[derive(Debug)]
pub(crate) enum ServerError {
    RequestError(reqwest::Error),
    IOError(std::io::Error),
    GeneratorError(Error),
    ParseIntError(ParseIntError),
}

impl From<reqwest::Error> for ServerError {
    fn from(err: reqwest::Error) -> Self {
        ServerError::RequestError(err)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::IOError(err)
    }
}

impl From<Error> for ServerError {
    fn from(err: Error) -> Self {
        ServerError::GeneratorError(err)
    }
}

impl From<ParseIntError> for ServerError {
    fn from(err: ParseIntError) -> Self {
        ServerError::ParseIntError(err)
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::RequestError(err) => write!(f, "Request error: {err}"),
            ServerError::IOError(err) => write!(f, "IO error: {err}"),
            ServerError::GeneratorError(err) => write!(f, "{err}"),
            ServerError::ParseIntError(err) => write!(f, "{err}"),
        }
    }
}

impl error::Error for ServerError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ErrorResponse {
    code: u16,
    message: String,
    data: Value,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

pub(crate) fn handle_server_error(error: ServerError) -> ErrorResponse {
    let message = format!("{error}");
    match error {
        ServerError::RequestError(err) => ErrorResponse {
            code: 410,
            message,
            data: json!({ "error": format!("{err}") }),
        },
        ServerError::IOError(err) => ErrorResponse {
            code: 420,
            message,
            data: json!({ "error": format!("{err}") }),
        },
        ServerError::ParseIntError(err) => ErrorResponse {
            code: 430,
            message,
            data: json!({"error": format!("{err}")}),
        },
        ServerError::GeneratorError(err) => handle_error(err),
    }
}

pub(crate) fn handle_error(error: Error) -> ErrorResponse {
    let message = format!("{error}");
    match error {
        Error::ImageDecodeError(err) => ErrorResponse {
            code: 510,
            message,
            data: json!({ "error": err }),
        },
        Error::ImageEncodeError(err) => ErrorResponse {
            code: 520,
            message,
            data: json!({ "error": err }),
        },
        Error::ImageAssetMissing(path) => ErrorResponse {
            code: 530,
            message,
            data: json!({ "path": path }),
        },
        Error::DeserializeError(err) => ErrorResponse {
            code: 540,
            message,
            data: json!({ "error": err }),
        },
        Error::ImageNumberMismatch(min, max, actual) => ErrorResponse {
            code: 550,
            message,
            data: json!({ "min": min, "max": max, "actual": actual }),
        },
        Error::TextNumberMismatch(min, max, actual) => ErrorResponse {
            code: 551,
            message,
            data: json!({ "min": min, "max": max, "actual": actual }),
        },
        Error::InvalidChoice(name, choices, given) => ErrorResponse {
            code: 552,
            message: message,
            data: json!({"name": name, "choices": choices, "given": given}),
        },
        Error::TextOverLength(text) => ErrorResponse {
            code: 560,
            message,
            data: json!({ "text": text }),
        },
        Error::Generic(feedback) => ErrorResponse {
            code: 570,
            message,
            data: json!({ "feedback": feedback }),
        },
    }
}

async fn image_generate(Path(key): Path<String>, Json(payload): Json<ImageRequest>) -> Response {
    let image = match get_image(&key) {
        Some(image) => image,
        None => return (StatusCode::NOT_FOUND, "Image not found").into_response(),
    };

    let images = match process_images(payload.images).await {
        Ok(imgs) => imgs,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let texts = payload.texts;
    let options = payload.options;

    let result = spawn_blocking(move || image.generate(images, texts, options))
        .await
        .unwrap();

    handle_image(result)
}

async fn image_preview(
    Path(key): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let image = match get_image(&key) {
        Some(image) => image,
        None => return (StatusCode::NOT_FOUND, "Image not found").into_response(),
    };

    let mut options: HashMap<String, OptionValue> = HashMap::new();

    for (k, v) in params {
        if let Ok(int_value) = v.parse::<i32>() {
            options.insert(k, OptionValue::Integer(int_value));
        } else if let Ok(float_value) = v.parse::<f32>() {
            options.insert(k, OptionValue::Float(float_value));
        } else if v.eq_ignore_ascii_case("true") {
            options.insert(k, OptionValue::Boolean(true));
        } else if v.eq_ignore_ascii_case("false") {
            options.insert(k, OptionValue::Boolean(false));
        } else {
            options.insert(k, OptionValue::String(v));
        }
    }

    let result = spawn_blocking(move || image.generate_preview(options))
        .await
        .unwrap();

    handle_image(result)
}

pub async fn run_server(host: Option<IpAddr>, port: Option<u16>) {
    let app = Router::new()
        .layer(DefaultBodyLimit::disable())
        .route("/image/{key}", post(image_generate))
        .route("/image/{key}/preview", get(image_preview))
        .route("/tools/inspect", post(handle_inspect))
        .route("/tools/inspect/{id}", post(handle_inspect))
        .route("/tools/gif_split", post(gif_split))
        .route("/tools/gif_split/{id}", post(gif_split))
        .route("/tools/gif_reverse", post(gif_reverse))
        .route("/tools/gif_reverse/{id}", post(gif_reverse))
        .route("/images/{image_id}", get(handle_images))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let host = host.unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
    let port = port.unwrap_or(5555);
    let addr = SocketAddr::new(host, port);
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Server running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
