use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub readme: Option<String>,
    pub version: String,
    pub tags: Vec<String>,
    pub download_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillVersion {
    pub id: String,
    pub skill_id: String,
    pub version: String,
    pub content: Option<String>,
    pub changelog: Option<String>,
    pub digest: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillVersionResponse {
    #[serde(flatten)]
    pub skill: Skill,
    pub content: Option<String>,
    pub version_info: SkillVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTag {
    pub id: String,
    pub skill_id: String,
    pub tag: String,
    pub version_id: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct SearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
}

pub struct ApiClient {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: &str, token: Option<String>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url: base_url.to_string(),
            token,
        })
    }

    fn build_request(&self, url: &str) -> reqwest::RequestBuilder {
        let mut builder = self.client.get(url);

        if let Some(ref token) = self.token {
            builder = builder.bearer_auth(token);
        }

        builder
    }

    async fn handle_response<T: for<'de> Deserialize<'de>>(response: reqwest::Response) -> Result<T> {
        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();

            if let Ok(api_error) = serde_json::from_str::<ApiErrorResponse>(&error_text) {
                return Err(anyhow!("API Error ({}): {}", api_error.error.code, api_error.error.message));
            }

            return Err(anyhow!("HTTP Error {}: {}", status, error_text));
        }

        response.json::<T>().await.context("Failed to parse response JSON")
    }

    pub async fn list_skills(&self, tags: Option<String>) -> Result<Vec<Skill>> {
        let mut url = format!("{}/skills", self.base_url);

        if let Some(ref t) = tags {
            url.push_str(&format!("?tags={}", urlencoding::encode(t)));
        }

        let response = self.build_request(&url).send().await
            .context("Failed to connect to API server. Is the server running?")?;

        Self::handle_response(response).await
    }

    pub async fn search_skills(&self, query: &str) -> Result<Vec<Skill>> {
        if query.trim().is_empty() {
            return Err(anyhow!("搜索关键词不能为空"));
        }

        let url = format!("{}/skills?q={}", self.base_url, urlencoding::encode(query));

        let response = self.build_request(&url).send().await
            .context("Failed to connect to API server. Is the server running?")?;

        Self::handle_response(response).await
    }

    pub async fn get_skill(&self, slug: &str) -> Result<Skill> {
        if slug.trim().is_empty() {
            return Err(anyhow!("技能 slug 不能为空"));
        }

        let url = format!("{}/skills/{}", self.base_url, urlencoding::encode(slug));

        let response = self.build_request(&url).send().await
            .context("Failed to connect to API server. Is the server running?")?;

        Self::handle_response(response).await
    }

    /// 获取技能的指定版本（支持 tag）
    pub async fn get_skill_version(&self, slug: &str, tag: &str) -> Result<SkillVersionResponse> {
        if slug.trim().is_empty() {
            return Err(anyhow!("技能 slug 不能为空"));
        }

        let url = format!("{}/skills/{}/{}", self.base_url,
            urlencoding::encode(slug),
            urlencoding::encode(tag));

        let response = self.build_request(&url).send().await
            .context("Failed to connect to API server. Is the server running?")?;

        Self::handle_response(response).await
    }

    /// 列出技能的所有标签
    pub async fn list_tags(&self, slug: &str) -> Result<Vec<SkillTag>> {
        if slug.trim().is_empty() {
            return Err(anyhow!("技能 slug 不能为空"));
        }

        let url = format!("{}/skills/{}/tags", self.base_url, urlencoding::encode(slug));

        let response = self.build_request(&url).send().await
            .context("Failed to connect to API server. Is the server running?")?;

        Self::handle_response(response).await
    }

    /// 创建标签
    pub async fn create_tag(&self, slug: &str, version: &str, tag: &str) -> Result<SkillTag> {
        if slug.trim().is_empty() {
            return Err(anyhow!("技能 slug 不能为空"));
        }

        let url = format!("{}/skills/{}/tags", self.base_url, urlencoding::encode(slug));

        let body = serde_json::json!({
            "version": version,
            "tag": tag
        });

        let response = self.client
            .post(&url)
            .bearer_auth(self.token.as_ref().unwrap_or(&String::new()))
            .json(&body)
            .send().await
            .context("Failed to connect to API server. Is the server running?")?;

        Self::handle_response(response).await
    }

    /// 删除标签
    pub async fn delete_tag(&self, slug: &str, tag: &str) -> Result<()> {
        if slug.trim().is_empty() {
            return Err(anyhow!("技能 slug 不能为空"));
        }

        let url = format!("{}/skills/{}/tags/{}", self.base_url,
            urlencoding::encode(slug),
            urlencoding::encode(tag));

        let response = self.client
            .delete(&url)
            .bearer_auth(self.token.as_ref().unwrap_or(&String::new()))
            .send().await
            .context("Failed to connect to API server. Is the server running?")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            if let Ok(api_error) = serde_json::from_str::<ApiErrorResponse>(&error_text) {
                return Err(anyhow!("API Error ({}): {}", api_error.error.code, api_error.error.message));
            }
            return Err(anyhow!("HTTP Error {}: {}", status, error_text));
        }

        Ok(())
    }

    /// 用户登录
    pub async fn login(&self, email: &str, password: &str) -> Result<String> {
        let url = format!("{}/auth/login", self.base_url);

        let body = serde_json::json!({
            "email": email,
            "password": password
        });

        let response = self.client
            .post(&url)
            .json(&body)
            .send().await
            .context("Failed to connect to API server. Is the server running?")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            if let Ok(api_error) = serde_json::from_str::<ApiErrorResponse>(&error_text) {
                return Err(anyhow!("{}: {}", api_error.error.code, api_error.error.message));
            }
            return Err(anyhow!("登录失败 (HTTP {}): {}", status, error_text));
        }

        #[derive(Debug, Deserialize)]
        struct LoginResponse {
            token: String,
        }

        let login_response: LoginResponse = response.json().await
            .context("Failed to parse login response")?;

        Ok(login_response.token)
    }

    /// 获取技能版本列表
    pub async fn list_versions(&self, slug: &str) -> Result<Vec<SkillVersion>> {
        if slug.trim().is_empty() {
            return Err(anyhow!("技能 slug 不能为空"));
        }

        let url = format!("{}/skills/{}/versions", self.base_url, urlencoding::encode(slug));

        let response = self.build_request(&url).send().await
            .context("Failed to connect to API server. Is the server running?")?;

        Self::handle_response(response).await
    }

    /// 检查 CLI 版本更新
    pub async fn check_cli_version(&self) -> Result<CliVersionInfo> {
        let url = format!("{}/cli/version", self.base_url);

        let response = self.client
            .get(&url)
            .send().await
            .context("Failed to check CLI version")?;

        let status = response.status();
        if !status.is_success() {
            return Err(anyhow!("版本检查失败 (HTTP {})", status));
        }

        response.json().await
            .context("Failed to parse version response")
    }
}

/// CLI 版本信息
#[derive(Debug, Clone, Deserialize)]
pub struct CliVersionInfo {
    pub version: String,
    pub release_date: String,
    pub changelog: String,
    pub downloads: CliDownloads,
    pub min_version: String,
    pub force_update: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CliDownloads {
    pub linux_x86_64: String,
    pub linux_arm64: String,
    pub macos_x86_64: String,
    pub macos_arm64: String,
    pub windows_x86_64: String,
}