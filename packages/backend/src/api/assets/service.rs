use crate::api::files;

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
