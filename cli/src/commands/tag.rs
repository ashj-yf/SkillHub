use anyhow::{anyhow, Result};
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

use crate::api::ApiClient;
use crate::config::Config;
use crate::TagAction;

#[derive(Tabled)]
struct TagRow {
    #[tabled(rename = "标签")]
    tag: String,
    #[tabled(rename = "版本")]
    version: String,
    #[tabled(rename = "更新时间")]
    updated_at: String,
}

pub async fn run(slug: &str, action: TagAction) -> Result<()> {
    let slug = slug.trim();
    if slug.is_empty() {
        return Err(anyhow!("技能 slug 不能为空"));
    }

    let config = Config::load()?;
    let client = ApiClient::new(&config.api_url, config.token.clone())?;

    match action {
        TagAction::List => list_tags(&client, slug).await,
        TagAction::Add { version, tag } => add_tag(&config, &client, slug, &version, &tag).await,
        TagAction::Rm { tag } => remove_tag(&config, &client, slug, &tag).await,
    }
}

async fn list_tags(client: &ApiClient, slug: &str) -> Result<()> {
    println!("{} 正在获取技能标签: {}", "→".yellow(), slug.cyan());

    let tags = client.list_tags(slug).await?;

    if tags.is_empty() {
        println!("{} 暂无标签", "!".yellow());
        return Ok(());
    }

    let rows: Vec<TagRow> = tags
        .iter()
        .map(|t| TagRow {
            tag: t.tag.clone(),
            version: t.version_id.clone(),
            updated_at: t.updated_at.clone(),
        })
        .collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();

    println!();
    println!("{}", table);
    println!();
    println!("共 {} 个标签", tags.len().to_string().green());

    Ok(())
}

async fn add_tag(config: &Config, client: &ApiClient, slug: &str, version: &str, tag: &str) -> Result<()> {
    println!("{} 正在添加标签: {} -> {}", "→".yellow(), tag.cyan(), version.yellow());

    if config.token.is_none() {
        return Err(anyhow!("需要登录才能管理标签。请使用 'skillhub login' 登录。"));
    }

    client.create_tag(slug, version, tag).await?;

    println!("{} 标签 {} 已添加", "✓".green(), tag.cyan());

    Ok(())
}

async fn remove_tag(config: &Config, client: &ApiClient, slug: &str, tag: &str) -> Result<()> {
    if tag == "latest" {
        return Err(anyhow!("不能删除 latest 标签"));
    }

    println!("{} 正在删除标签: {}", "→".yellow(), tag.cyan());

    if config.token.is_none() {
        return Err(anyhow!("需要登录才能管理标签。请使用 'skillhub login' 登录。"));
    }

    client.delete_tag(slug, tag).await?;

    println!("{} 标签 {} 已删除", "✓".green(), tag.cyan());

    Ok(())
}