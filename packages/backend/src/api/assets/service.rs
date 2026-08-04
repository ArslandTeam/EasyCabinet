use crate::{BackendError, api::storage_manager::StorageService};
use imagesize::ImageType;

pub struct AssetsService;

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

impl AssetsService {
    pub async fn upload_image(
        storage: &StorageService,
        asset_type: AssetType,
        image: &[u8],
    ) -> Result<String, BackendError> {
        if image.is_empty() {
            return Err(BackendError::BadRequest("Image data is empty".into()));
        }

        Self::verify_asset(asset_type, image)?;
        storage.save_file(image, asset_type.scope()).await
    }

    pub async fn delete_image(
        storage: &StorageService,
        asset_type: AssetType,
        hash: &str,
    ) -> Result<(), BackendError> {
        storage.remove_file(asset_type.scope(), hash).await
    }

    fn verify_asset(asset_type: AssetType, image: &[u8]) -> Result<(), BackendError> {
        let format = imagesize::image_type(image).map_err(|_| {
            BackendError::BadRequest(format!("Unknown or corrupted {asset_type:?} format"))
        })?;

        if format != ImageType::Png {
            return Err(BackendError::BadRequest(format!(
                "Invalid {asset_type:?} format: expected PNG, got {:?}",
                format
            )));
        }

        let size = imagesize::blob_size(image).map_err(|_| {
            BackendError::BadRequest(format!("Failed to parse {asset_type:?} dimensions"))
        })?;

        let (width, height) = (size.width as u32, size.height as u32);

        if !asset_type.valid_size(width, height) {
            return Err(BackendError::BadRequest(format!(
                "Invalid {asset_type:?} size: got {width}x{height}"
            )));
        }

        Ok(())
    }
}
