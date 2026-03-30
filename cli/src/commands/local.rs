use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;
use tabled::{Table, Tabled};

use crate::config::Config;

/// 本地技能元数据
#[derive(Debug, Deserialize)]
pub struct LocalSkill {
    pub name: String,
    pub slug: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub download_tag: String,
}

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
}

/// 列出本地已安装的技能
pub async fn run() -> Result<()> {
    let config = Config::load()?;
    let skills_dir = &config.skills_dir;

    if !skills_dir.exists() {
        println!("{} 暂无已安装的技能", "!".yellow());
        println!();
        println!("使用 {} 下载技能", "skillhub pull <skill>".cyan());
        return Ok(());
    }

    // 扫描技能目录
    let mut skills = Vec::new();
    for entry in std::fs::read_dir(skills_dir)? {
        let entry = entry?;
        let skill_yaml = entry.path().join("skill.yaml");

        if skill_yaml.exists() {
            if let Ok(content) = std::fs::read_to_string(&skill_yaml) {
                if let Ok(skill) = serde_yaml::from_str::<LocalSkill>(&content) {
                    skills.push(skill);
                }
            }
        }
    }

    if skills.is_empty() {
        println!("{} 暂无已安装的技能", "!".yellow());
        println!();
        println!("使用 {} 下载技能", "skillhub pull <skill>".cyan());
        return Ok(());
    }

    // 转换为表格行
    let rows: Vec<SkillRow> = skills
        .iter()
        .map(|s| SkillRow {
            name: s.name.clone(),
            slug: s.slug.clone(),
            version: s.version.clone(),
            tags: s.tags.join(", "),
        })
        .collect();

    println!("{} 已安装技能 ({} 个)", "→".yellow(), skills.len());
    println!();

    let table = Table::new(rows);
    println!("{table}");

    println!();
    println!("技能目录: {}", skills_dir.display().to_string().dimmed());

    Ok(())
}