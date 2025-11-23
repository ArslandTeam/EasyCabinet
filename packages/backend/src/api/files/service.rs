// TODO переписать
use crate::{BackendError, generate_config::CONFIG};
use aws_sdk_s3::primitives::ByteStream;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tokio::sync::OnceCell;

pub static FILES_SERVICE: OnceCell<FilesService> = OnceCell::const_new(); // FIX использовать другую реализацию DI (через async trait)

pub async fn files_service() -> &'static FilesService {
    FILES_SERVICE
        .get_or_init(|| async { FilesService::storage_init().await })
        .await
}

#[derive(Clone)]
enum StorageType {
    Local,
    S3 {
        client: aws_sdk_s3::Client,
        bucket: String,
    },
}

#[derive(Clone)]
pub struct FilesService {
    storage: StorageType,
}

impl FilesService {
    pub async fn storage_init() -> Self {
        match &*CONFIG.storage_textures_type {
            "local" => FilesService {
                storage: StorageType::Local,
            },
            "s3" => {
                let config = aws_config::load_from_env().await;
                let clinet = aws_sdk_s3::Client::new(&config);
                FilesService {
                    storage: StorageType::S3 {
                        client: clinet,
                        bucket: CONFIG.bucket_name.clone(),
                    },
                }
            }
            _ => panic!("STORAGE_TEXTURES_TYPE not correct set"),
        }
    }

    pub fn format_url(&self, scope: &str, hash: &str) -> String {
        match &self.storage {
            StorageType::Local => {
                format!(
                    "{}/uploads/{}",
                    CONFIG.backend_url,
                    format_path(scope, hash)
                )
            }
            StorageType::S3 { .. } => {
                format!("{}/{}", CONFIG.s3_aws_global_url, format_path(scope, hash)) // FIX переписать принципи логику убрать format_url и возращать клиенту сразу же файл а не ссылку
            }
        }
    }

    pub async fn save_file(&self, file: &[u8], scope: &str) -> Result<String, BackendError> {
        let hash = generate_hash(file);
        let path = format_path(scope, &hash);
        self.save_image_to_disk(file, &path).await?;
        Ok(hash)
    }

    async fn save_image_to_disk(&self, file: &[u8], path: &str) -> Result<(), BackendError> {
        match &self.storage {
            StorageType::Local => {
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
            StorageType::S3 { client, bucket } => client
                .put_object()
                .bucket(bucket)
                .key(path)
                .body(ByteStream::from(file.to_vec()))
                .send()
                .await
                .map(|_| ())
                .map_err(|_| BackendError::InternalError),
        }
    }
}

fn format_path(scope: &str, hash: &str) -> String {
    let prefix = &hash[..2.min(hash.len())];
    format!("{scope}/{prefix}/{hash}")
}

fn generate_hash(buffer: &[u8]) -> String {
    let hash = Sha256::digest(buffer);
    format!("{:x}", hash)
}
