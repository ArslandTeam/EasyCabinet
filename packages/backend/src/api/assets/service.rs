// TODO переписать
use crate::{BackendError, api::files};
use image::{ImageFormat, ImageReader};
use std::io::Cursor;

#[derive(Clone, Copy, Debug)]
pub enum AssetType {
    Skin,
    Cape,
}

pub fn format_url(asset_type: AssetType, hash: &str) -> Option<String> {
    if hash.is_empty() {
        return None;
    }

    let scope = match asset_type {
        AssetType::Skin => "skin",
        AssetType::Cape => "cape",
    };

    Some(files::service::format_url(scope, hash))
}

pub async fn upload_image(asset_type: AssetType, image: &[u8]) -> Result<String, BackendError> {
    if image.is_empty() {
        return Err(BackendError::BadRequest("Invalid image".to_string()));
    }

    verify_asset(asset_type, image).await?;

    let scope = match asset_type {
        AssetType::Skin => "skin",
        AssetType::Cape => "cape",
    };

    files::service::save_file(image, scope)
        .await
        .map_err(|_| BackendError::BadRequest("".to_string()))
}

async fn verify_asset(asset_type: AssetType, image: &[u8]) -> Result<(), BackendError> {
    let image_reader = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .expect("Cursor io never fails")
        .format()
        .ok_or_else(|| BackendError::BadRequest(format!("Unknown {:#?} format", asset_type)))
        .unwrap();

    match image_reader {
        ImageFormat::Png | ImageFormat::Jpeg => Ok(()),
        _ => Err(BackendError::BadRequest(format!(
            "Invalid {:#?} format: expected PNG or JPEG",
            asset_type
        ))),
    }
}
