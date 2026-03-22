use anyhow::{anyhow, Result};
use colored::Colorize;

use crate::api::ApiClient;
use crate::config::Config;

/// 解析技能引用，返回 (slug, tag)
/// "python-security" -> ("python-security", "latest")
/// "python-security:v1.0.0" -> ("python-security", "v1.0.0")
fn parse_reference(reference: &str) -> (&str, &str) {
    match reference.split_once(':') {
        Some((slug, tag)) => (slug, tag),
        None => (reference, "latest"),
    }
}

pub async fn run(reference: &str, target_dir: Option<String>) -> Result<()> {
    // 验证引用
    let reference = reference.trim();
    if reference.is_empty() {
        return Err(anyhow!("技能引用不能为空"));
    }

    // 解析引用
    let (slug, tag) = parse_reference(reference);

    // 验证 slug
    let slug = slug.trim();
    if slug.is_empty() {
        return Err(anyhow!("技能 slug 不能为空"));
    }

    let config = Config::load()?;
    let client = ApiClient::new(&config.api_url, config.token.clone())?;

    // 获取技能版本
    println!("{} 正在获取技能: {}:{}", "→".yellow(), slug.cyan(), tag.yellow());

    let response = client.get_skill_version(slug, tag).await?;
    let skill = response.skill;
    let version_info = response.version_info;

    // 确定目标目录
    let skill_dir = if let Some(dir) = target_dir {
        std::path::PathBuf::from(dir).join(&skill.slug)
    } else {
        config.skills_dir.join(&skill.slug)
    };

    // 检查目录是否已存在
    if skill_dir.exists() {
        println!("{} 技能目录已存在: {}", "!".yellow(), skill_dir.display());
        println!("如需更新，请先删除现有目录");
        return Ok(());
    }

    // 创建目录
    std::fs::create_dir_all(&skill_dir)?;

    // 写入 skill.yaml
    let skill_yaml = serde_yaml::to_string(&serde_yaml::Value::Mapping({
        let mut map = serde_yaml::Mapping::new();
        map.insert("name".into(), skill.name.clone().into());
        map.insert("slug".into(), skill.slug.clone().into());
        map.insert("version".into(), version_info.version.clone().into());
        map.insert(
            "description".into(),
            skill.description.clone().unwrap_or_default().into(),
        );
        map.insert("tags".into(), skill.tags.clone().into());
        map.insert("download_tag".into(), tag.into());
        map.insert("download_version".into(), version_info.version.clone().into());
        map
    }))?;
    std::fs::write(skill_dir.join("skill.yaml"), skill_yaml)?;

    // 写入 claude-skill.md（使用版本内容或 readme）
    let content = response.content
        .or(skill.readme)
        .unwrap_or_default();

    if !content.is_empty() {
        std::fs::write(skill_dir.join("claude-skill.md"), content)?;
    }

    println!();
    println!(
        "{} 技能 {}:{} ({}) 已安装到:",
        "✓".green(),
        skill.slug.cyan(),
        tag.yellow(),
        format!("v{}", version_info.version).yellow()
    );
    println!("  {}", skill_dir.display());

    if let Some(digest) = &version_info.digest {
        println!("  Digest: {}", digest.dimmed());
    }

    println!();
    println!("使用方式:");
    println!("  在 Claude Code 中: 将技能目录添加到项目配置");

    Ok(())
}