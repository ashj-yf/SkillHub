use anyhow::Result;
use colored::Colorize;

use crate::config::Config;

/// 用户登出
pub async fn run() -> Result<()> {
    let mut config = Config::load()?;

    if config.token.is_none() {
        println!("{} 当前未登录", "!".yellow());
        return Ok(());
    }

    config.token = None;
    config.save()?;

    println!("{} 已登出", "✓".green());

    Ok(())
}