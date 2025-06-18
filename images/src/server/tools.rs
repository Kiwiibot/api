use std::{collections::HashMap, path::PathBuf, sync::LazyLock, time::Duration};

use axum::{
    Json,
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
    task::{spawn, spawn_blocking},
    time::sleep,
};

use crate::{
    core::{self, error::Error, types::OptionValue},
    server::server::{ImageData, ServerError, handle_error, handle_server_error},
    tools,
};

static REQWEST_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageRequest {
    pub(crate) images: Vec<ImageData>,
    pub(crate) texts: Vec<String>,
    pub(crate) options: HashMap<String, OptionValue>,
}

async fn download_url(
    url: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<Vec<u8>, ServerError> {
    let headers = headers.unwrap_or_default();
    let request = REQWEST_CLIENT.get(url);
    let request = headers
        .iter()
        .fold(request, |request, (key, value)| request.header(key, value));
    let response = request.send().await?;
    let data = response.bytes().await?;
    Ok(data.to_vec())
}

pub(crate) async fn process_images(
    imgs: Vec<ImageData>,
) -> Result<Vec<core::types::Image>, ServerError> {
    let mut images: Vec<core::types::Image> = Vec::new();
    for image in imgs {
        let (name, data) = match image {
            ImageData::Url { url, headers, name } => match download_url(&url, headers).await {
                Ok(data) => (name, data),
                Err(err) => return Err(err.into()),
            },
            ImageData::Path { path, name } => match std::fs::read(&path) {
                Ok(data) => (name, data),
                Err(err) => return Err(err.into()),
            },
            ImageData::Data { name, data } => (name, data),
        };

        images.push(core::types::Image { name, data });
    }

    Ok(images)
}

async fn preamble_tool(
    images: Vec<ImageData>,
    index: Option<Path<String>>,
) -> Result<core::types::Image, ServerError> {
    let imgs = match process_images(images).await {
        Ok(imgs) => imgs,
        Err(err) => return Err(err),
    };

    let index = match index {
        Some(id) => match id.parse::<usize>() {
            Ok(idx) => idx,
            Err(err) => return Err(err.into()),
        },
        None => 0,
    };

    Ok(imgs[index].clone())
}

pub async fn handle_inspect(
    param: Option<Path<String>>,
    Json(payload): Json<ImageRequest>,
) -> Response {
    let img = match preamble_tool(payload.images, param).await {
        Ok(img) => img,
        Err(err) => return handle_server_error(err).into_response(),
    };

    match spawn_blocking(move || tools::images::inspect::inspect(img.data))
        .await
        .unwrap()
    {
        Ok(result) => Json(result).into_response(),
        Err(err) => handle_error(err).into_response(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ImagesResponse {
    images_ids: Vec<String>,
}

pub(crate) async fn create_temp_file(data: Vec<u8>) -> Result<String, ServerError> {
    let id = format!("{:x}", md5::compute(&data));
    let path = PathBuf::from("/tmp").join(&id);
    if path.exists() {
        return Ok(id);
    }
    let mut file = File::create(&path).await?;
    file.write_all(&data).await?;
    let path_for_task = path.clone();
    spawn(async move {
        sleep(Duration::from_secs(30 * 60)).await;

        if let Err(e) = fs::remove_file(&path_for_task).await {
            eprintln!("temp cleanup: failed to remove {:?}: {}", path_for_task, e);
        }
    });
    Ok(id)
}

pub(crate) async fn get_temp_file(id: &str) -> Result<Vec<u8>, ServerError> {
    let path = PathBuf::from("/tmp").join(id);
    let mut file = File::open(&path).await?;
    let mut data = Vec::new();
    file.read_to_end(&mut data).await?;
    Ok(data)
}

pub(crate) fn handle_image(result: Result<Vec<u8>, Error>) -> Response {
    match result {
        Ok(data) => {
            let kind = infer::get(&data).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", kind.mime_type())
                .body(Body::from(data))
                .unwrap()
        }
        Err(error) => handle_error(error).into_response(),
    }
}

pub(crate) async fn handle_images_result(result: Result<Vec<Vec<u8>>, Error>) -> Response {
    let mut images_ids = vec![];
    match result {
        Ok(data) => {
            for d in data {
                match create_temp_file(d).await {
                    Ok(id) => images_ids.push(id),
                    Err(err) => return handle_server_error(err).into_response(),
                };
            }
        }
        Err(error) => return handle_error(error).into_response(),
    }
    let response = ImagesResponse { images_ids };
    Json(response).into_response()
}

pub(crate) async fn handle_images(Path(image_id): Path<String>) -> Response {
    match get_temp_file(&image_id).await {
        Ok(data) => {
            let kind = infer::get(&data).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", kind.mime_type())
                .body(Body::from(data))
                .unwrap()
        }
        Err(err) => handle_server_error(err).into_response(),
    }
}

pub(crate) async fn gif_split(
    param: Option<Path<String>>,
    Json(payload): Json<ImageRequest>,
) -> Response {
    let img = match preamble_tool(payload.images, param).await {
        Ok(img) => img,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let result = spawn_blocking(move || tools::images::modification::gif_split(img.data))
        .await
        .unwrap();

    handle_images_result(result).await
}

pub(crate) async fn gif_reverse(
    param: Option<Path<String>>,
    Json(payload): Json<ImageRequest>,
) -> Response {
    let img = match preamble_tool(payload.images, param).await {
        Ok(img) => img,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let result = spawn_blocking(move || tools::images::modification::gif_reverse(img.data))
        .await
        .unwrap();

    handle_image(result)
}

pub(crate) async fn gif(
    param: Option<Path<String>>,
    Json(payload): Json<ImageRequest>,
) -> Response {
    let img = match preamble_tool(payload.images, param).await {
        Ok(img) => img,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let result = spawn_blocking(move || tools::images::modification::gif(img.data))
        .await
        .unwrap();

    handle_image(result)
}
