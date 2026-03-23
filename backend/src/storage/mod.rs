use anyhow::{Context, Result};
use aws_config::BehaviorVersion;
use aws_sdk_s3::primitives::ByteStream;
use tracing::{error, info};
use uuid::Uuid;

/// S3-compatible storage client (RustFS/MinIO/AWS S3)
#[derive(Clone)]
pub struct Storage {
    client: aws_sdk_s3::Client,
    bucket: String,
}

impl Storage {
    /// Create a new storage client from configuration
    pub async fn new(
        endpoint: &str,
        access_key: &str,
        secret_key: &str,
        bucket: &str,
    ) -> Result<Self> {
        // Create AWS SDK config for S3-compatible storage (RustFS/MinIO/AWS S3)
        let credentials = aws_sdk_s3::config::Credentials::new(
            access_key,
            secret_key,
            None,
            None,
            "storage-service",
        );

        let config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(aws_sdk_s3::config::Region::new("us-east-1"))
            .endpoint_url(endpoint)
            .credentials_provider(credentials)
            .force_path_style(true) // Required for S3-compatible storage
            .build();

        let client = aws_sdk_s3::Client::from_conf(config);

        // Ensure bucket exists
        let storage = Self {
            client,
            bucket: bucket.to_string(),
        };
        storage.ensure_bucket().await?;

        Ok(storage)
    }

    /// Ensure the bucket exists, create if not
    async fn ensure_bucket(&self) -> Result<()> {
        let bucket_exists = self
            .client
            .head_bucket()
            .bucket(&self.bucket)
            .send()
            .await;

        if bucket_exists.is_err() {
            info!(bucket = %self.bucket, "Bucket not found, creating...");
            self.client
                .create_bucket()
                .bucket(&self.bucket)
                .send()
                .await
                .context("Failed to create bucket")?;
            info!(bucket = %self.bucket, "Bucket created successfully");
        }

        Ok(())
    }

    /// Generate storage path for skill version content
    fn get_skill_path(skill_id: Uuid, version: &str) -> String {
        format!("skills/{}/versions/{}/content", skill_id, version)
    }

    /// Upload skill content to object storage
    pub async fn upload_skill_content(
        &self,
        skill_id: Uuid,
        version: &str,
        content: &str,
    ) -> Result<String> {
        let key = Self::get_skill_path(skill_id, version);

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(ByteStream::from(content.as_bytes().to_vec()))
            .content_type("text/plain")
            .send()
            .await
            .map_err(|e| {
                error!(skill_id = %skill_id, version = %version, bucket = %self.bucket, error = ?e, "Failed to upload skill content");
                e
            })
            .context("Failed to upload skill content to storage")?;

        Ok(key)
    }

    /// Download skill content from object storage
    pub async fn download_skill_content(
        &self,
        skill_id: Uuid,
        version: &str,
    ) -> Result<Option<String>> {
        let key = Self::get_skill_path(skill_id, version);

        let result = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await;

        match result {
            Ok(output) => {
                let bytes = output
                    .body
                    .collect()
                    .await
                    .context("Failed to read skill content from storage")?;
                let content = String::from_utf8(bytes.into_bytes().to_vec())
                    .context("Failed to parse skill content as UTF-8")?;
                Ok(Some(content))
            }
            Err(e) => {
                // Check if it's a "not found" error
                if let Some(service_error) = e.as_service_error() {
                    if service_error.is_no_such_key() {
                        return Ok(None);
                    }
                }
                error!(skill_id = %skill_id, version = %version, bucket = %self.bucket, error = ?e, "Failed to download skill content");
                Err(e).context("Failed to download skill content from storage")
            }
        }
    }

    /// Delete skill content from object storage
    pub async fn delete_skill_content(&self, skill_id: Uuid, version: &str) -> Result<()> {
        let key = Self::get_skill_path(skill_id, version);

        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await
            .context("Failed to delete skill content from storage")?;

        Ok(())
    }

    /// Delete all versions of a skill from object storage
    /// Note: This is a best-effort cleanup. For complete cleanup,
    /// use bucket lifecycle policies or list and delete all objects.
    pub async fn delete_skill(&self, _skill_id: Uuid) -> Result<()> {
        // For a complete implementation, we would need to:
        // 1. List all objects with prefix "skills/{skill_id}/"
        // 2. Delete each object
        // For now, we'll rely on the bucket lifecycle policy or manual cleanup
        Ok(())
    }
}