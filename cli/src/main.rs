use clap::{Parser, Subcommand};

mod api;
mod commands;
mod config;

#[derive(Parser)]
#[command(name = "skillhub")]
#[command(about = "Skills Intelligence Hub CLI", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 初始化配置
    Init,

    /// 列出可用技能
    List {
        /// 过滤标签
        #[arg(short, long)]
        tags: Option<String>,
    },

    /// 下载技能（支持 Docker Tag 语法）
    Pull {
        /// 技能引用，格式: skill-slug 或 skill-slug:tag
        /// 示例: python-security, python-security:latest, python-security:v1.0.0
        reference: String,

        /// 目标目录
        #[arg(short, long)]
        dir: Option<String>,
    },

    /// 搜索技能
    Search {
        /// 搜索关键词
        query: String,
    },

    /// 显示技能详情
    Show {
        /// 技能 slug
        slug: String,
    },

    /// 管理技能标签
    Tag {
        /// 技能 slug
        slug: String,

        #[command(subcommand)]
        action: TagAction,
    },
}

#[derive(Subcommand)]
enum TagAction {
    /// 列出所有标签
    List,

    /// 添加标签
    Add {
        /// 版本号
        version: String,
        /// 标签名
        tag: String,
    },

    /// 删除标签
    Rm {
        /// 标签名
        tag: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run().await?,
        Commands::List { tags } => commands::list::run(tags).await?,
        Commands::Pull { reference, dir } => commands::pull::run(&reference, dir).await?,
        Commands::Search { query } => commands::search::run(&query).await?,
        Commands::Show { slug } => commands::show::run(&slug).await?,
        Commands::Tag { slug, action } => commands::tag::run(&slug, action).await?,
    }

    Ok(())
}