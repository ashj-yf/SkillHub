// Storage module - uses LocalStorage as backend
// aws-sdk-s3 requires Rust 1.91+, current stable is 1.86

mod local;

use anyhow::Result;
pub use local::LocalStorage;
use uuid::Uuid;

/// 存储后端
#[derive(Clone)]
pub enum StorageBackend {
    Local(LocalStorage),
}

impl StorageBackend {
    pub async fn upload_skill_content(
        &self,
        skill_id: Uuid,
        version: &str,
        content: &[u8],
    ) -> Result<String> {
        match self {
            StorageBackend::Local(local) => {
                local.upload_skill_content(skill_id, version, content).await
            }
        }
    }

    pub async fn download_skill_content(
        &self,
        skill_id: Uuid,
        version: &str,
    ) -> Result<Option<Vec<u8>>> {
        match self {
            StorageBackend::Local(local) => {
                local.download_skill_content(skill_id, version).await
            }
        }
    }

    pub async fn delete_skill_content(&self, skill_id: Uuid, version: &str) -> Result<()> {
        match self {
            StorageBackend::Local(local) => {
                local.delete_skill_content(skill_id, version).await
            }
        }
    }

    pub async fn delete_skill(&self, skill_id: Uuid) -> Result<()> {
        match self {
            StorageBackend::Local(local) => local.delete_skill(skill_id).await,
        }
    }
}

/// 默认存储位置
pub const DEFAULT_STORAGE_PATH: &str = "./data/storage";

impl StorageBackend {
    /// 创建默认本地存储
    pub fn local() -> Result<Self> {
        let storage = LocalStorage::new(DEFAULT_STORAGE_PATH)?;
        Ok(StorageBackend::Local(storage))
    }

    /// 创建指定路径的本地存储
    pub fn local_with_path(path: &str) -> Result<Self> {
        let storage = LocalStorage::new(path)?;
        Ok(StorageBackend::Local(storage))
    }
}