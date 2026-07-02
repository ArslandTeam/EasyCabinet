use crate::{BackendError, generate_config::CONFIG};

enum StorageType {
    Local,
    S3 { client: aws_sdk_s3::Client },
}

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
                let client = aws_sdk_s3::Client::new(&config);
                StorageService {
                    storage: StorageType::S3 { client },
                }
            }
            _ => panic!("STORAGE_TEXTURES_TYPE not correct set"),
        }
    }

    pub fn format_url(&self, scope: &str, hash: &str) -> String {
        let path = Self::format_path(scope, hash);
        match &self.storage {
            StorageType::Local => {
                format!("{}/uploads/{path}", CONFIG.backend_url)
            }
            StorageType::S3 { .. } => {
                format!("{}/{}/{path}", CONFIG.aws_public_url, CONFIG.bucket_name)
            }
        }
    }

    pub async fn save_file(&self, file: &[u8], scope: &str) -> Result<String, BackendError> {
        let hash = Self::generate_hash(file);
        let path = Self::format_path(scope, &hash);
        self.save_image_to_disk(file, &path).await?;
        Ok(hash)
    }

    pub async fn remove_file(&self, scope: &str, hash: &str) -> Result<(), BackendError> {
        let path = Self::format_path(scope, hash);
        self.remove_image_to_disk(&path).await?;
        Ok(())
    }

    async fn save_image_to_disk(&self, file: &[u8], path: &str) -> Result<(), BackendError> {
        match &self.storage {
            StorageType::Local => {
                let file_path = std::path::PathBuf::from("uploads").join(path);

                if let Some(parent) = file_path.parent() {
                    tokio::fs::create_dir_all(parent).await.map_err(|e| {
                        tracing::error!("Error create dir: {e}");
                        BackendError::InternalError
                    })?;
                }

                tokio::fs::write(file_path, file).await.map_err(|e| {
                    tracing::error!("Error write file: {e}");
                    BackendError::InternalError
                })
            }
            StorageType::S3 { client, .. } => client
                .put_object()
                .bucket(&CONFIG.bucket_name)
                .key(path)
                .body(aws_sdk_s3::primitives::ByteStream::from(file.to_vec()))
                .send()
                .await
                .map(|_| ())
                .map_err(|e| {
                    tracing::error!("Error S3: {e}");
                    BackendError::InternalError
                }),
        }
    }

    async fn remove_image_to_disk(&self, path: &str) -> Result<(), BackendError> {
        match &self.storage {
            StorageType::Local => {
                let file_path = std::path::PathBuf::from("uploads").join(path);

                tokio::fs::remove_file(file_path).await.map_err(|e| {
                    tracing::error!("{e}");
                    BackendError::InternalError
                })
            }
            StorageType::S3 { client, .. } => client
                .delete_object()
                .bucket(&CONFIG.bucket_name)
                .key(path)
                .send()
                .await
                .map(|_| ())
                .map_err(|e| {
                    tracing::error!("Error S3: {e}");
                    BackendError::InternalError
                }),
        }
    }

    fn format_path(scope: &str, hash: &str) -> String {
        let prefix = &hash[..2];
        format!("{scope}/{prefix}/{hash}")
    }

    fn generate_hash(buffer: &[u8]) -> String {
        use sha2::Digest;
        let hash = sha2::Sha256::digest(buffer);
        format!("{:x}", hash)
    }
}
