// TODO переписать
use crate::{BackendError, api::files::service::files_service};
use image::{ImageFormat, ImageReader};
use std::io::Cursor;

#[derive(Clone, Copy, Debug)]
pub enum AssetType {
    Skin,
    Cape,
}

pub async fn format_url(asset_type: AssetType, hash: &str) -> Option<String> {
    if hash.is_empty() {
        return None;
    }
    let scope = match asset_type {
        AssetType::Skin => "skin",
        AssetType::Cape => "cape",
    };

    Some(files_service().await.format_url(scope, hash))
}

pub async fn upload_image(asset_type: AssetType, image: &[u8]) -> Result<String, BackendError> {
    if image.is_empty() {
        return Err(BackendError::BadRequest("Invalid image".into()));
    }

    verify_asset(asset_type, image).await?;

    let scope = match asset_type {
        AssetType::Skin => "skin",
        AssetType::Cape => "cape",
    };

    files_service()
        .await
        .save_file(image, scope)
        .await
        .map_err(|_| BackendError::InternalError)
}

// FIX вот это тем более переписать
async fn verify_asset(asset_type: AssetType, image: &[u8]) -> Result<(), BackendError> {
    let reader = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .map_err(|_| BackendError::BadRequest(format!("Failed to read {asset_type:?}")))?;

    let format = reader.format().ok_or_else(|| {
        BackendError::BadRequest(format!("Unknown {asset_type:?} format (could not guess)"))
    })?;

    match format {
        ImageFormat::Png | ImageFormat::Jpeg => {}
        _ => {
            return Err(BackendError::BadRequest(format!(
                "Invalid {asset_type:?} format: expected PNG or JPEG, got {:?}",
                format
            )));
        }
    }

    let img = reader
        .decode()
        .map_err(|_| BackendError::BadRequest(format!("Invalid {asset_type:?} data")))?;

    use image::GenericImageView;
    let (width, height) = img.dimensions();

    let valid_sizes: &[(u32, u32)] = match asset_type {
        AssetType::Skin => &[(64, 32), (64, 64)],
        AssetType::Cape => &[(64, 32)],
    };

    let valid = valid_sizes.iter().any(|&(w, h)| w == width && h == height);

    if !valid {
        return Err(BackendError::BadRequest(format!(
            "Invalid {asset_type:?} size: got {width}x{height}"
        )));
    }

    Ok(())
}
