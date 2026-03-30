use anyhow::{anyhow, Result};
use colored::Colorize;

use crate::api::ApiClient;
use crate::commands::local::LocalSkill;
use crate::config::Config;

/// 更新已安装的技能
pub async fn run(slug: Option<String>, update_all: bool) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config.api_url, config.token.clone())?;

    if update_all {
        // 更新所有技能
        let skills = scan_local_skills(&config.skills_dir)?;

        if skills.is_empty() {
            println!("{} 暂无已安装的技能", "!".yellow());
            return Ok(());
        }

        println!("{} 检查所有技能更新...", "→".yellow());
        println!();

        for skill in &skills {
            if let Err(e) = update_skill(&client, &config, &skill.slug).await {
                println!("{} 更新 {} 失败: {}", "✗".red(), skill.slug.cyan(), e);
            }
        }

        println!();
        println!("{} 更新完成", "✓".green());
    } else if let Some(slug) = slug {
        update_skill(&client, &config, &slug).await?;
    } else {
        return Err(anyhow!("请指定技能 slug 或使用 --all"));
    }

    Ok(())
}

async fn update_skill(client: &ApiClient, config: &Config, slug: &str) -> Result<()> {
    // 获取本地版本
    let local = get_local_skill_version(&config.skills_dir, slug)?;

    // 获取远程最新版本
    let remote = client.get_skill(slug).await?;

    if local.version == remote.version {
        println!("{} {} 已是最新版本 ({})", "✓".green(), slug.cyan(), local.version.yellow());
        return Ok(());
    }

    println!("{} 更新 {} ({} -> {})", "→".yellow(), slug.cyan(), local.version.yellow(), remote.version.yellow());

    // TODO: 实际下载更新（复用 pull 逻辑）
    // 当前版本：显示提示
    println!("  请使用 {} 重新下载", format!("skillhub pull {}:latest", slug).cyan());

    Ok(())
}

fn scan_local_skills(skills_dir: &std::path::Path) -> Result<Vec<LocalSkill>> {
    let mut skills = Vec::new();

    if !skills_dir.exists() {
        return Ok(skills);
    }

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

    Ok(skills)
}

fn get_local_skill_version(skills_dir: &std::path::Path, slug: &str) -> Result<LocalSkill> {
    let skill_yaml = skills_dir.join(slug).join("skill.yaml");

    if !skill_yaml.exists() {
        return Err(anyhow!("技能未安装: {}", slug));
    }

    let content = std::fs::read_to_string(&skill_yaml)?;
    let skill: LocalSkill = serde_yaml::from_str(&content)?;

    Ok(skill)
}