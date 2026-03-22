use anyhow::Result;
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

use crate::api::ApiClient;
use crate::config::Config;

#[derive(Tabled)]
struct SkillRow {
    #[tabled(rename = "名称")]
    name: String,
    #[tabled(rename = "Slug")]
    slug: String,
    #[tabled(rename = "版本")]
    version: String,
    #[tabled(rename = "标签")]
    tags: String,
    #[tabled(rename = "下载")]
    downloads: String,
}

pub async fn run(tags: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config.api_url, config.token.clone())?;

    println!("{} 正在获取技能列表...", "→".yellow());

    let skills = client.list_skills(tags).await?;

    if skills.is_empty() {
        println!("{} 暂无技能", "!".yellow());
        return Ok(());
    }

    let rows: Vec<SkillRow> = skills
        .iter()
        .map(|s| SkillRow {
            name: s.name.clone(),
            slug: s.slug.clone(),
            version: s.version.clone(),
            tags: s.tags.join(", "),
            downloads: s.download_count.to_string(),
        })
        .collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();

    println!();
    println!("{}", table);
    println!();
    println!("共 {} 个技能", skills.len().to_string().green());

    Ok(())
}