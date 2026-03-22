# Skills Intelligence Hub - Phase 1 实现计划

> 创建时间：2026-03-21
> 状态：待执行

---

## 一、项目结构

```
skills-hub-project/
├── backend/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs            # 入口
│   │   ├── lib.rs             # 库入口
│   │   ├── api/               # API 路由
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs        # 认证接口
│   │   │   ├── skills.rs      # 技能接口
│   │   │   └── users.rs       # 用户接口
│   │   ├── models/            # 数据模型
│   │   │   ├── mod.rs
│   │   │   ├── user.rs
│   │   │   ├── skill.rs
│   │   │   └── department.rs
│   │   ├── services/          # 业务逻辑
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   └── skill.rs
│   │   ├── repos/             # 数据访问
│   │   │   ├── mod.rs
│   │   │   ├── user.rs
│   │   │   └── skill.rs
│   │   ├── utils/             # 工具函数
│   │   │   ├── mod.rs
│   │   │   ├── jwt.rs
│   │   │   └── error.rs
│   │   └── config.rs          # 配置
│   ├── migrations/            # 数据库迁移
│   │   ├── 001_init.sql
│   │   └── 002_skills.sql
│   ├── Cargo.toml
│   └── .env.example
├── cli/                       # Rust CLI
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── init.rs
│   │   │   ├── list.rs
│   │   │   ├── pull.rs
│   │   │   └── search.rs
│   │   ├── config.rs
│   │   └── api.rs             # API 客户端
│   └── Cargo.toml
├── web/                       # Vue 3 前端
│   ├── src/
│   │   ├── main.ts
│   │   ├── App.vue
│   │   ├── views/
│   │   │   ├── Login.vue
│   │   │   ├── Register.vue
│   │   │   ├── Market.vue
│   │   │   ├── SkillDetail.vue
│   │   │   └── Admin.vue
│   │   ├── components/
│   │   │   ├── SkillCard.vue
│   │   │   ├── SearchBar.vue
│   │   │   └── Navbar.vue
│   │   ├── api/
│   │   │   ├── index.ts
│   │   │   ├── auth.ts
│   │   │   └── skills.ts
│   │   ├── stores/
│   │   │   └── user.ts
│   │   └── router/
│   │       └── index.ts
│   ├── package.json
│   ├── vite.config.ts
│   └── tsconfig.json
├── docker/
│   ├── docker-compose.yml
│   ├── Dockerfile.backend
│   └── Dockerfile.web
└── docs/
    └── ...
```

---

## 二、实现任务清单

### 任务 1: 后端项目初始化 (Day 1-2)

**步骤：**
1. 创建 `backend/` 目录
2. 初始化 Cargo 项目
3. 添加依赖
4. 创建基础目录结构
5. 配置数据库连接

**验收标准：**
- [ ] `cargo build` 成功
- [ ] `cargo test` 通过
- [ ] 数据库连接测试通过

**依赖 (Cargo.toml)：**
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
jsonwebtoken = "9"
bcrypt = "0.15"
tower-http = { version = "0.5", features = ["cors"] }
anyhow = "1"
thiserror = "1"
dotenvy = "0.15"
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

### 任务 2: 数据库设计与迁移 (Day 2-3)

**步骤：**
1. 设计数据表结构
2. 创建迁移文件
3. 实现 models 层
4. 实现 repos 层

**数据库 Schema：**

```sql
-- migrations/001_init.sql

-- 用户表
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(20) DEFAULT 'user',
    department_id UUID REFERENCES departments(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 部门表
CREATE TABLE departments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    parent_id UUID REFERENCES departments(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 索引
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_department ON users(department_id);
```

```sql
-- migrations/002_skills.sql

-- 技能表
CREATE TABLE skills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    readme TEXT,
    author_id UUID REFERENCES users(id),
    tags TEXT[],
    is_public BOOLEAN DEFAULT true,
    download_count INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 技能版本表（Docker Tag 模式）
CREATE TABLE skill_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    skill_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    version VARCHAR(50) NOT NULL,       -- v1.0.0, v2.1.0-beta.1
    storage_path VARCHAR(500) NOT NULL,
    changelog TEXT,
    digest VARCHAR(64),                 -- 内容哈希
    created_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID REFERENCES users(id),
    UNIQUE(skill_id, version)
);

-- 技能标签表（类似 Docker Tag）
CREATE TABLE skill_tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    skill_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    tag VARCHAR(50) NOT NULL,           -- latest, stable, v1, v1.0.0
    version_id UUID REFERENCES skill_versions(id) ON DELETE CASCADE,
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    updated_by UUID REFERENCES users(id),
    UNIQUE(skill_id, tag)
);

-- 索引
CREATE INDEX idx_skills_slug ON skills(slug);
CREATE INDEX idx_skills_author ON skills(author_id);
CREATE INDEX idx_skills_tags ON skills USING GIN(tags);
CREATE INDEX idx_skill_tags_skill ON skill_tags(skill_id);
CREATE INDEX idx_skill_versions_skill ON skill_versions(skill_id);
```

**Model 示例 (models/user.rs)：**
```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: String,
    pub department_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
```

**验收标准：**
- [ ] 迁移执行成功
- [ ] Model 编译通过
- [ ] Repo 层 CRUD 测试通过

---

### 任务 3: 认证系统实现 (Day 3-4)

**步骤：**
1. 实现 JWT 工具函数
2. 实现密码哈希
3. 实现注册/登录 API
4. 实现认证中间件

**JWT 工具 (utils/jwt.rs)：**
```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // user id
    pub role: String,
    pub exp: usize,
}

pub fn create_token(user_id: &str, role: &str, secret: &str) -> Result<String> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        role: role.to_owned(),
        exp,
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}
```

**认证 API (api/auth.rs)：**
```rust
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use crate::{
    models::{CreateUser, LoginRequest},
    services::AuthService,
    utils::error::ApiError,
};

pub async fn register(
    State(service): State<AuthService>,
    Json(payload): Json<CreateUser>,
) -> Result<StatusCode, ApiError> {
    service.register(payload).await?;
    Ok(StatusCode::CREATED)
}

pub async fn login(
    State(service): State<AuthService>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let token = service.login(payload).await?;
    Ok(Json(serde_json::json!({ "token": token })))
}
```

**验收标准：**
- [ ] 注册接口返回 201
- [ ] 登录接口返回有效 JWT
- [ ] 受保护接口拒绝无 Token 请求

---

### 任务 4: 技能 API 实现 (Day 4-5)

**步骤：**
1. 实现技能 CRUD 接口
2. 实现搜索接口
3. 实现版本管理

**技能 API (api/skills.rs)：**
```rust
use axum::{
    extract::{Path, Query, State},
    Json,
};
use crate::{
    models::{Skill, CreateSkill, UpdateSkill},
    services::SkillService,
    utils::error::ApiError,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub tags: Option<String>,
    pub page: Option<u32>,
}

pub async fn list(
    State(service): State<SkillService>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<Skill>>, ApiError> {
    let skills = service.list(query.q, query.tags, query.page).await?;
    Ok(Json(skills))
}

pub async fn get(
    State(service): State<SkillService>,
    Path(slug): Path<String>,
) -> Result<Json<Skill>, ApiError> {
    let skill = service.get_by_slug(&slug).await?;
    Ok(Json(skill))
}

pub async fn create(
    State(service): State<SkillService>,
    Json(payload): Json<CreateSkill>,
) -> Result<Json<Skill>, ApiError> {
    let skill = service.create(payload).await?;
    Ok(Json(skill))
}
```

**验收标准：**
- [ ] GET /api/skills 返回技能列表
- [ ] GET /api/skills/:slug 返回技能详情
- [ ] POST /api/skills 创建技能成功
- [ ] 搜索功能正常

---

### 任务 5: CLI 工具开发 (Day 5-7)

**步骤：**
1. 初始化 CLI 项目
2. 实现 init 命令
3. 实现 list 命令
4. 实现 pull 命令
5. 实现 search 命令

**CLI 依赖 (Cargo.toml)：**
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
dirs = "5"
colored = "2"
```

**CLI 入口 (src/main.rs)：**
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "skillhub")]
#[command(about = "Skills Intelligence Hub CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 初始化配置
    Init,
    /// 列出可用技能
    List,
    /// 下载技能（支持 Docker Tag 语法）
    Pull {
        /// 技能引用，格式: skill-id 或 skill-id:tag
        /// 示例: python-security, python-security:latest, python-security:v1.0.0
        reference: String,
    },
    /// 搜索技能
    Search { query: String },
    /// 管理技能标签
    Tag {
        /// 技能 ID
        skill_id: String,
        #[command(subcommand)]
        action: TagAction,
    },
}

#[derive(Subcommand)]
enum TagAction {
    /// 列出所有标签
    List,
    /// 添加标签
    Add { version: String, tag: String },
    /// 删除标签
    Rm { tag: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run().await?,
        Commands::List => commands::list::run().await?,
        Commands::Pull { reference } => commands::pull::run(&reference).await?,
        Commands::Search { query } => commands::search::run(&query).await?,
        Commands::Tag { skill_id, action } => commands::tag::run(&skill_id, action).await?,
    }

    Ok(())
}
```

**Pull 命令 (commands/pull.rs)：**
```rust
use anyhow::Result;
use crate::config::Config;
use crate::api::Client;

/// 解析技能引用，返回 (skill_id, tag)
/// "python-security" -> ("python-security", "latest")
/// "python-security:v1.0.0" -> ("python-security", "v1.0.0")
fn parse_reference(reference: &str) -> (&str, &str) {
    match reference.split_once(':') {
        Some((id, tag)) => (id, tag),
        None => (reference, "latest"),
    }
}

pub async fn run(reference: &str) -> Result<()> {
    let config = Config::load()?;
    let client = Client::new(&config.api_url)?;

    let (skill_id, tag) = parse_reference(reference);
    println!("正在获取技能: {}:{}\n", skill_id, tag);

    let skill = client.get_skill(skill_id, tag).await?;

    // 写入本地
    let skill_dir = config.skills_dir.join(skill_id);
    std::fs::create_dir_all(&skill_dir)?;

    // 写入 skill.yaml
    let yaml_content = serde_yaml::to_string(&skill)?;
    std::fs::write(skill_dir.join("skill.yaml"), yaml_content)?;

    // 写入 claude-skill.md
    std::fs::write(
        skill_dir.join("claude-skill.md"),
        skill.readme,
    )?;

    println!("✓ 技能 {}:{} 已安装到 {}", skill_id, tag, skill_dir.display());

    Ok(())
}
```

**验收标准：**
- [ ] `skillhub init` 创建配置文件
- [ ] `skillhub list` 显示技能列表
- [ ] `skillhub pull <skill-id>` 下载最新版本
- [ ] `skillhub pull <skill-id>:v1.0.0` 下载指定版本
- [ ] `skillhub search <query>` 搜索技能
- [ ] `skillhub tag <skill-id> list` 列出标签

---

### 任务 6: Web UI 开发 (Day 7-10)

**步骤：**
1. 初始化 Vue 项目
2. 实现登录/注册页面
3. 实现技能市场页面
4. 实现技能详情页面

**Web 依赖 (package.json)：**
```json
{
  "dependencies": {
    "vue": "^3.4",
    "vue-router": "^4",
    "pinia": "^2",
    "axios": "^1",
    "@vueuse/core": "^10"
  },
  "devDependencies": {
    "vite": "^5",
    "typescript": "^5",
    "@vitejs/plugin-vue": "^5",
    "tailwindcss": "^3"
  }
}
```

**路由配置 (router/index.ts)：**
```typescript
import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  { path: '/login', component: () => import('@/views/Login.vue') },
  { path: '/register', component: () => import('@/views/Register.vue') },
  { path: '/', component: () => import('@/views/Market.vue') },
  { path: '/skill/:slug', component: () => import('@/views/SkillDetail.vue') },
  { path: '/admin', component: () => import('@/views/Admin.vue'), meta: { requiresAuth: true } },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})
```

**API 客户端 (api/skills.ts)：**
```typescript
import axios from 'axios'

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL || 'http://localhost:3000/api',
})

export interface Skill {
  id: string
  name: string
  slug: string
  description: string
  readme: string
  version: string
  tags: string[]
  download_count: number
}

export async function listSkills(params?: { q?: string; tags?: string }) {
  const { data } = await api.get<Skill[]>('/skills', { params })
  return data
}

export async function getSkill(slug: string) {
  const { data } = await api.get<Skill>(`/skills/${slug}`)
  return data
}
```

**验收标准：**
- [ ] 登录/注册功能正常
- [ ] 技能列表正确显示
- [ ] 技能详情页正常
- [ ] 搜索功能正常

---

### 任务 7: Docker 部署配置 (Day 10-11)

**docker-compose.yml：**
```yaml
version: '3.8'

services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_DB: skillhub
      POSTGRES_USER: skillhub
      POSTGRES_PASSWORD: skillhub123
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  backend:
    build:
      context: ./backend
      dockerfile: ../docker/Dockerfile.backend
    environment:
      DATABASE_URL: postgres://skillhub:skillhub123@postgres:5432/skillhub
      JWT_SECRET: your-secret-key
    ports:
      - "3000:3000"
    depends_on:
      - postgres

  web:
    build:
      context: ./web
      dockerfile: ../docker/Dockerfile.web
    ports:
      - "8080:80"
    depends_on:
      - backend

volumes:
  postgres_data:
```

**验收标准：**
- [ ] `docker-compose up` 成功启动所有服务
- [ ] 后端健康检查通过
- [ ] 前端可访问

---

### 任务 8: 集成测试 (Day 11-12)

**步骤：**
1. 编写后端集成测试
2. 编写 CLI 测试
3. 编写 E2E 测试

**后端测试示例：**
```rust
#[cfg(test)]
mod tests {
    use axum::test::TestServer;
    use super::*;

    #[tokio::test]
    async fn test_register_and_login() {
        let server = TestServer::new(create_test_app()).unwrap();

        // 注册
        let response = server
            .post("/api/auth/register")
            .json(&json!({
                "username": "testuser",
                "email": "test@example.com",
                "password": "password123"
            }))
            .await;
        assert_eq!(response.status_code(), StatusCode::CREATED);

        // 登录
        let response = server
            .post("/api/auth/login")
            .json(&json!({
                "email": "test@example.com",
                "password": "password123"
            }))
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
```

**验收标准：**
- [ ] 所有后端测试通过
- [ ] CLI 命令测试通过
- [ ] E2E 测试通过

---

## 三、执行检查点

| 检查点 | 时间 | 验收项 |
|--------|------|--------|
| CP1 | Day 3 | 后端编译通过、数据库连接成功 |
| CP2 | Day 5 | 认证系统完成、API 测试通过 |
| CP3 | Day 7 | CLI 基础命令可用 |
| CP4 | Day 10 | Web UI 功能完整 |
| CP5 | Day 12 | 全部测试通过、可部署 |

---

## 四、风险与应对

| 风险 | 应对措施 |
|------|----------|
| Rust 编译问题 | 使用稳定的依赖版本，参考官方文档 |
| 数据库迁移失败 | 提供回滚脚本 |
| 前端样式问题 | 使用 Tailwind CSS 组件库 |

---

## 五、启动命令

```bash
# 后端开发
cd backend && cargo run

# CLI 开发
cd cli && cargo run -- list

# Web 开发
cd web && npm run dev

# 完整部署
docker-compose up -d
```

---

*文档由 Claude Code 生成*