use anyhow::Result;
use colored::Colorize;
use tabled::{Table, Tabled};

use crate::api::ApiClient;
use crate::config::Config;

#[derive(Tabled)]
struct VersionRow {
    #[tabled(rename = "版本")]
    version: String,
    #[tabled(rename = "创建时间")]
    created_at: String,
    #[tabled(rename = "变更日志")]
    changelog: String,
}

/// 查看技能版本历史
pub async fn run(slug: &str) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config.api_url, config.token.clone())?;

    println!("{} 正在获取版本历史: {}", "→".yellow(), slug.cyan());
    println!();

    let versions = client.list_versions(slug).await?;

    if versions.is_empty() {
        println!("{} 该技能暂无版本记录", "!".yellow());
        return Ok(());
    }

    // 转换为表格行
    let rows: Vec<VersionRow> = versions
        .iter()
        .map(|v| VersionRow {
            version: v.version.clone(),
            created_at: v.created_at.split('T').next().unwrap_or(&v.created_at).to_string(),
            changelog: v.changelog.as_deref().unwrap_or("-").chars().take(30).collect::<String>(),
        })
        .collect();

    println!("{} 版本历史 ({} 个)", "→".yellow(), versions.len());
    println!();

    let table = Table::new(rows);
    println!("{table}");

    println!();
    println!("使用 {} 下载指定版本", "skillhub pull <skill>:<version>".cyan());

    Ok(())
}