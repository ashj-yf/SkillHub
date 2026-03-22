use anyhow::Result;
use colored::Colorize;

use crate::config::Config;

pub async fn run() -> Result<()> {
    let config_path = Config::config_path();

    if config_path.exists() {
        println!("{} 配置文件已存在: {}", "✓".green(), config_path.display());
        println!();
        println!("如需重置配置，请先删除配置文件后重新运行 init");
        return Ok(());
    }

    let config = Config::default();
    config.save()?;

    println!("{} 配置文件已创建: {}", "✓".green(), config_path.display());
    println!();
    println!("配置详情:");
    println!("  API URL: {}", config.api_url);
    println!("  技能目录: {}", config.skills_dir.display());
    println!();
    println!("现在可以使用以下命令:");
    println!("  {}          列出可用技能", "skillhub list".cyan());
    println!("  {}  搜索技能", "skillhub search <关键词>".cyan());
    println!("  {}     下载技能", "skillhub pull <slug>".cyan());

    Ok(())
}