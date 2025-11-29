use crate::{BackendError, generate_config::CONFIG};
use aws_sdk_s3::primitives::ByteStream;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

#[derive(Clone)]
enum StorageType {
    Local,
    S3 {
        client: aws_sdk_s3::Client,
        bucket: String,
    },
}

#[derive(Clone)]
pub struct StorageService {
    storage: StorageType,
}

impl StorageService {
    pub async fn storage_init() -> Self {
        match &*CONFIG.storage_textures_type {
            "local" => StorageService {
                storage: StorageType::Local,
            },
            "s3" => {
                let config = aws_config::load_from_env().await;
                let clinet = aws_sdk_s3::Client::new(&config);
                StorageService {
                    storage: StorageType::S3 {
                        client: clinet,
                        bucket: CONFIG.bucket_name.clone(),
                    },
                }
            }
            _ => panic!("STORAGE_TEXTURES_TYPE not correct set"),
        }
    }

    // INFO Обработка через map_err избатачна но возможно понадобится в будущем
    pub async fn get_file(&self, scope: &str, hash: &str) -> Result<Box<[u8]>, BackendError> {
        let path = Self::format_path(scope, hash);
        match &self.storage {
            StorageType::Local => {
                let file_path = PathBuf::from("uploads").join(path);
                let bytes = tokio::fs::read(file_path)
                    .await
                    .map_err(|_| BackendError::BadRequest("Not found".into()))?;

                Ok(bytes.into_boxed_slice())
            }
            StorageType::S3 { client, bucket } => {
                let response = client
                    .get_object()
                    .bucket(bucket)
                    .key(path)
                    .send()
                    .await
                    .map_err(|_| BackendError::BadRequest("Not found".into()))?;

                let bytes = response
                    .body
                    .collect()
                    .await
                    .map_err(|_| BackendError::InternalError)?
                    .into_bytes()
                    .to_vec();

                Ok(bytes.into_boxed_slice())
            }
        }
    }

    pub async fn save_file(&self, file: &[u8], scope: &str) -> Result<String, BackendError> {
        let hash = Self::generate_hash(file);
        let path = Self::format_path(scope, &hash);
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
                        .map_err(|_| panic!("No permission create dir"))?;
                }

                tokio::fs::write(file_path, file)
                    .await
                    .map_err(|_| panic!("No permission write file"))
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

    fn format_path(scope: &str, hash: &str) -> String {
        let prefix = &hash[..2.min(hash.len())];
        format!("{scope}/{prefix}/{hash}")
    }

    fn generate_hash(buffer: &[u8]) -> String {
        let hash = Sha256::digest(buffer);
        format!("{:x}", hash)
    }
}
