use crate::{BackendError, api::storage::service::StorageService};
use image::{ImageFormat, ImageReader};
use std::io::Cursor;

#[derive(Clone, Copy, Debug)]
pub enum AssetType {
    Skin,
    Cape,
}

impl AssetType {
    pub const fn scope(self) -> &'static str {
        match self {
            Self::Skin => "skin",
            Self::Cape => "cape",
        }
    }

    pub const fn valid_size(self, width: u32, height: u32) -> bool {
        match self {
            Self::Skin => matches!((width, height), (64, 32) | (64, 64)),
            Self::Cape => matches!((width, height), (64, 32)),
        }
    }
}

pub async fn upload_image(
    storage: &StorageService,
    asset_type: AssetType,
    image: &[u8],
) -> Result<String, BackendError> {
    if image.is_empty() {
        return Err(BackendError::BadRequest("Image data is empty".to_string()));
    }

    verify_asset(asset_type, image)?;
    storage.save_file(image, asset_type.scope()).await
}

fn verify_asset(asset_type: AssetType, image: &[u8]) -> Result<(), BackendError> {
    let reader = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .map_err(|_| BackendError::BadRequest(format!("Failed to read {asset_type:?}")))?;

    let format_image = reader.format().ok_or_else(|| {
        BackendError::BadRequest(format!("Unknown {asset_type:?} format (could not guess)"))
    })?;

    if !matches!(format_image, ImageFormat::Png | ImageFormat::Jpeg) {
        return Err(BackendError::BadRequest(format!(
            "Invalid {asset_type:?} format: expected PNG or JPEG, got {format_image:?}"
        )));
    }

    let (width, height) = reader.into_dimensions().map_err(|_| {
        BackendError::BadRequest(format!("Invalid {asset_type:?} metadata or corrupted data"))
    })?;

    if !asset_type.valid_size(width, height) {
        return Err(BackendError::BadRequest(format!(
            "Invalid {asset_type:?} size: got {width}x{height}"
        )));
    }

    Ok(())
}
