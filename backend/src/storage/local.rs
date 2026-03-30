use anyhow::{anyhow, Result};
use std::fs;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

/// 本地文件存储
#[derive(Clone)]
pub struct LocalStorage {
    base_path: PathBuf,
}

impl LocalStorage {
    pub fn new(base_path: &str) -> Result<Self> {
        let path = PathBuf::from(base_path);

        // 确保存储目录存在
        fs::create_dir_all(&path)?;

        // 确保技能目录存在
        fs::create_dir_all(path.join("skills"))?;

        Ok(Self { base_path: path })
    }

    /// 上传技能内容
    pub async fn upload_skill_content(
        &self,
        skill_id: Uuid,
        version: &str,
        content: &[u8],
    ) -> Result<String> {
        let dir = self.base_path.join("skills").join(skill_id.to_string());
        fs::create_dir_all(&dir)?;

        let file_path = dir.join(format!("{}.tar.gz", version));

        // 异步写入文件
        let path = file_path.clone();
        let content = content.to_vec();
        tokio::task::spawn_blocking(move || {
            fs::write(&path, &content)
        }).await??;

        let storage_path = format!("local://{}", file_path.display());
        Ok(storage_path)
    }

    /// 下载技能内容
    pub async fn download_skill_content(
        &self,
        skill_id: Uuid,
        version: &str,
    ) -> Result<Option<Vec<u8>>> {
        let file_path = self
            .base_path
            .join("skills")
            .join(skill_id.to_string())
            .join(format!("{}.tar.gz", version));

        if !file_path.exists() {
            return Ok(None);
        }

        let path = file_path.clone();
        let content = tokio::task::spawn_blocking(move || {
            fs::read(&path)
        }).await??;

        Ok(Some(content))
    }

    /// 删除技能特定版本
    pub async fn delete_skill_content(&self, skill_id: Uuid, version: &str) -> Result<()> {
        let file_path = self
            .base_path
            .join("skills")
            .join(skill_id.to_string())
            .join(format!("{}.tar.gz", version));

        if file_path.exists() {
            let path = file_path.clone();
            tokio::task::spawn_blocking(move || {
                fs::remove_file(&path)
            }).await??;
        }

        Ok(())
    }

    /// 删除技能所有版本
    pub async fn delete_skill(&self, skill_id: Uuid) -> Result<()> {
        let dir = self.base_path.join("skills").join(skill_id.to_string());

        if dir.exists() {
            let path = dir.clone();
            tokio::task::spawn_blocking(move || {
                fs::remove_dir_all(&path)
            }).await??;
        }

        Ok(())
    }

    /// 列出技能所有版本
    pub fn list_versions(&self, skill_id: Uuid) -> Result<Vec<String>> {
        let dir = self.base_path.join("skills").join(skill_id.to_string());

        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut versions = Vec::new();
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(version) = name.strip_suffix(".tar.gz") {
                versions.push(version.to_string());
            }
        }

        Ok(versions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_upload_and_download() {
        let dir = tempdir().unwrap();
        let storage = LocalStorage::new(dir.path().to_str().unwrap()).unwrap();

        let skill_id = Uuid::new_v4();
        let content = b"test content";

        // 上传
        let path = storage.upload_skill_content(skill_id, "v1.0.0", content).await.unwrap();
        assert!(path.contains(&skill_id.to_string()));

        // 下载
        let downloaded = storage.download_skill_content(skill_id, "v1.0.0").await.unwrap();
        assert!(downloaded.is_some());
        assert_eq!(downloaded.unwrap(), content);

        // 列出版本
        let versions = storage.list_versions(skill_id).unwrap();
        assert!(versions.contains(&"v1.0.0".to_string()));
    }

    #[tokio::test]
    async fn test_delete_version() {
        let dir = tempdir().unwrap();
        let storage = LocalStorage::new(dir.path().to_str().unwrap()).unwrap();

        let skill_id = Uuid::new_v4();
        storage.upload_skill_content(skill_id, "v1.0.0", b"content").await.unwrap();

        // 删除
        storage.delete_skill_content(skill_id, "v1.0.0").await.unwrap();

        // 确认已删除
        let downloaded = storage.download_skill_content(skill_id, "v1.0.0").await.unwrap();
        assert!(downloaded.is_none());
    }

    #[tokio::test]
    async fn test_delete_skill() {
        let dir = tempdir().unwrap();
        let storage = LocalStorage::new(dir.path().to_str().unwrap()).unwrap();

        let skill_id = Uuid::new_v4();
        storage.upload_skill_content(skill_id, "v1.0.0", b"content1").await.unwrap();
        storage.upload_skill_content(skill_id, "v1.0.1", b"content2").await.unwrap();

        // 删除所有版本
        storage.delete_skill(skill_id).await.unwrap();

        // 确认目录已删除
        let versions = storage.list_versions(skill_id).unwrap();
        assert!(versions.is_empty());
    }
}