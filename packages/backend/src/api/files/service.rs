// TODO переписать
use crate::BackendError;
use sha2::{Digest, Sha256};
use std::{env, path::PathBuf};

pub fn format_url(scope: &str, hash: &str) -> String {
    let backend_url = env::var("BACKEND_URL").expect("BACKEND_URL key not set in .env");
    format!("{}/uploads/{}", backend_url, format_path(scope, hash))
}

fn format_path(scope: &str, hash: &str) -> String {
    let prefix = &hash[..2.min(hash.len())];
    format!("{scope}/{prefix}/{hash}")
}

pub async fn save_file(file: &[u8], scope: &str) -> Result<String, BackendError> {
    let hash = generate_hash(file);
    let path = format_path(scope, &hash);
    save_image_to_disk(file, &path).await?;
    Ok(hash)
}

async fn save_image_to_disk(file: &[u8], path: &str) -> Result<(), BackendError> {
    let file_path = PathBuf::from("uploads").join(path);

    if let Some(parent) = file_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|_| BackendError::InternalError)?;
    }

    tokio::fs::write(file_path, file)
        .await
        .map_err(|_| BackendError::InternalError)
}

fn generate_hash(buffer: &[u8]) -> String {
    let hash = Sha256::digest(buffer);
    format!("{:x}", &hash)
}
