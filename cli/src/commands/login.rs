use anyhow::Result;
use colored::Colorize;
use dialoguer::{Input, Password};

use crate::api::ApiClient;
use crate::config::Config;

/// 用户登录
pub async fn run(token: Option<String>) -> Result<()> {
    let config = Config::load()?;

    let token = match token {
        Some(t) => {
            println!("{} 使用提供的 Token 登录...", "→".yellow());
            t
        }
        None => {
            // 交互式登录
            println!("{} Skills Hub CLI 登录", "→".yellow());
            println!();

            let email: String = Input::new()
                .with_prompt("邮箱")
                .interact()?;

            let password = Password::new()
                .with_prompt("密码")
                .interact()?;

            // 调用 API 登录
            let client = ApiClient::new(&config.api_url, None)?;

            println!();
            println!("{} 正在验证...", "→".yellow());

            match client.login(&email, &password).await {
                Ok(t) => t,
                Err(e) => {
                    println!("{} 登录失败: {}", "✗".red(), e);
                    return Err(e);
                }
            }
        }
    };

    // 保存 Token 到配置
    let mut config = config;
    config.token = Some(token);
    config.save()?;

    println!();
    println!("{} 登录成功！", "✓".green());
    println!("  Token 已保存到: {}", Config::config_path().display());

    Ok(())
}