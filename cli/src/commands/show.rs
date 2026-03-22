use anyhow::{anyhow, Result};
use colored::Colorize;

use crate::api::ApiClient;
use crate::config::Config;

pub async fn run(slug: &str) -> Result<()> {
    // 验证 slug
    let slug = slug.trim();
    if slug.is_empty() {
        return Err(anyhow!("技能 slug 不能为空"));
    }

    let config = Config::load()?;
    let client = ApiClient::new(&config.api_url, config.token.clone())?;

    println!("{} 正在获取技能详情: {}", "→".yellow(), slug.cyan());

    let skill = client.get_skill(slug).await?;

    println!();
    println!("{}", skill.name.green().bold());
    println!("{} {}", "版本:".dimmed(), skill.version.yellow());
    println!();

    if let Some(desc) = &skill.description {
        if !desc.is_empty() {
            println!("{}", desc);
            println!();
        }
    }

    if !skill.tags.is_empty() {
        println!("{} {}", "标签:".dimmed(), skill.tags.join(", "));
        println!();
    }

    println!(
        "{} {} 次下载",
        "下载量:".dimmed(),
        skill.download_count.to_string().cyan()
    );

    // 获取可用标签
    println!();
    println!("{} 正在获取可用标签...", "→".yellow());

    match client.list_tags(slug).await {
        Ok(tags) if !tags.is_empty() => {
            let tag_names: Vec<&str> = tags.iter().map(|t| t.tag.as_str()).collect();
            println!("{} {}", "可用标签:".dimmed(), tag_names.join(", "));
        }
        _ => {
            println!("{} 暂无其他标签", "!".yellow());
        }
    }

    if let Some(readme) = &skill.readme {
        if !readme.is_empty() {
            println!();
            println!("{}", "─".repeat(50).dimmed());
            println!();
            println!("{}", readme);
        }
    }

    println!();
    println!("使用 {} 下载此技能", format!("skillhub pull {}", slug).cyan());

    Ok(())
}