use anyhow::Result;
use colored::Colorize;
use dialoguer::Confirm;

use crate::config::Config;

/// 删除已安装的技能
pub async fn run(slug: &str, force: bool) -> Result<()> {
    let config = Config::load()?;
    let skill_dir = config.skills_dir.join(slug);

    if !skill_dir.exists() {
        println!("{} 技能未安装: {}", "!".yellow(), slug.cyan());
        return Ok(());
    }

    // 确认删除
    if !force {
        let confirm = Confirm::new()
            .with_prompt(format!("确定删除技能 '{}'?", slug.cyan()))
            .default(false)
            .interact()?;

        if !confirm {
            println!("已取消");
            return Ok(());
        }
    }

    // 删除目录
    std::fs::remove_dir_all(&skill_dir)?;

    println!("{} 已删除技能: {}", "✓".green(), slug.cyan());

    Ok(())
}