use anyhow::Result;
use colored::Colorize;

use crate::api::ApiClient;
use crate::config::Config;

/// 版本信息（编译时注入）
const VERSION: &str = env!("VERSION");
const GIT_COMMIT: &str = env!("GIT_COMMIT");
const BUILD_DATE: &str = env!("BUILD_DATE");

/// 显示版本信息
pub async fn run() -> Result<()> {
    println!("Skills Hub CLI v{}", VERSION.cyan());
    println!();
    println!("  Commit:  {}", GIT_COMMIT.dimmed());
    println!("  Build:   {}", BUILD_DATE.dimmed());
    println!();

    // 检查更新
    check_update().await?;

    Ok(())
}

/// 检查 CLI 更新
async fn check_update() -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config.api_url, None)?;

    match client.check_cli_version().await {
        Ok(info) => {
            if info.version != VERSION {
                println!("{} 新版本可用: v{} (当前: v{})", "!".yellow(), info.version.cyan(), VERSION);
                println!();
                println!("  更新日志:");
                for line in info.changelog.lines().take(5) {
                    println!("    {}", line);
                }
                println!();
                println!("  访问 GitHub Releases 下载最新版本");
            } else {
                println!("{} 已是最新版本", "✓".green());
            }
        }
        Err(e) => {
            println!("{} 无法检查更新: {}", "!".yellow(), e);
        }
    }

    Ok(())
}