use std::process::Command;

fn main() {
    // 从 Cargo.toml 获取版本
    let version = env!("CARGO_PKG_VERSION").to_string();

    // 获取 git commit hash
    let commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // 获取构建时间
    let build_date = chrono_lite();

    // 注入环境变量
    println!("cargo:rustc-env=VERSION={}", version);
    println!("cargo:rustc-env=GIT_COMMIT={}", commit);
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);

    // 重新构建时重新运行
    println!("cargo:rerun-if-changed=Cargo.toml");
}

fn chrono_lite() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();

    let secs = duration.as_secs();
    let days = secs / 86400;
    let year = 1970 + (days * 400 + 800) / 146097;
    let month = ((secs % 31536000) / 2592000 + 1).min(12);
    let day = ((secs % 2592000) / 86400 + 1).min(31);

    format!("{}-{:02}-{:02}", year, month, day)
}