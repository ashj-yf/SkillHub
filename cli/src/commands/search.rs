use anyhow::{anyhow, Result};
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

use crate::api::ApiClient;
use crate::config::Config;

#[derive(Tabled)]
struct SearchResultRow {
    #[tabled(rename = "名称")]
    name: String,
    #[tabled(rename = "Slug")]
    slug: String,
    #[tabled(rename = "描述")]
    description: String,
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        // 安全地截断，避免在多字节字符中间截断
        let chars: Vec<char> = text.chars().collect();
        if chars.len() <= max_len {
            text.to_string()
        } else {
            chars[..max_len - 3].iter().collect::<String>() + "..."
        }
    }
}

pub async fn run(query: &str) -> Result<()> {
    // 验证查询参数
    let query = query.trim();
    if query.is_empty() {
        return Err(anyhow!("搜索关键词不能为空"));
    }

    let config = Config::load()?;
    let client = ApiClient::new(&config.api_url, config.token.clone())?;

    println!("{} 正在搜索: {}", "→".yellow(), query.cyan());

    let skills = client.search_skills(query).await?;

    if skills.is_empty() {
        println!("{} 未找到匹配的技能", "!".yellow());
        return Ok(());
    }

    let rows: Vec<SearchResultRow> = skills
        .iter()
        .map(|s| SearchResultRow {
            name: s.name.clone(),
            slug: s.slug.clone(),
            description: truncate_text(
                s.description.as_deref().unwrap_or(""),
                50
            ),
        })
        .collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();

    println!();
    println!("{}", table);
    println!();
    println!(
        "找到 {} 个匹配技能",
        skills.len().to_string().green()
    );
    println!("使用 {} 下载技能", "skillhub pull <slug>".cyan());

    Ok(())
}