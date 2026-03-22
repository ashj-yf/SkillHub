# SkillHub

<p align="center">
  <strong>企业级 AI 技能管理平台</strong>
</p>

<p align="center">
  <em>让技能主动找到用户</em>
</p>

<p align="center">
  <a href="#功能特性">功能特性</a> •
  <a href="#技术栈">技术栈</a> •
  <a href="#快速开始">快速开始</a> •
  <a href="#cli-使用">CLI 使用</a> •
  <a href="#api-文档">API 文档</a>
</p>

---

## 简介

SkillHub 是一个企业级 AI 技能管理平台，用于集中管理、分发和共享 AI 编程工具的技能模板。支持 Claude Code、Cursor、GitHub Copilot 等多种 AI 编程工具。

**核心理念**：从"存储分发"升级为"智能技能引擎"，让技能主动找到用户。

## 功能特性

- 用户认证（注册/登录/JWT）
- 技能 CRUD 操作
- 技能版本管理（Docker Tag 模式）
- CLI 工具（init/list/pull/search/show/tag）
- Web 技能市场界面
- PostgreSQL 全文搜索
- MinIO 对象存储

### 版本管理

采用 Docker Tag 模式管理技能版本：

```bash
skillhub pull python-security           # 默认 latest
skillhub pull python-security:v1.0.0    # 指定版本
skillhub pull python-security:v1        # 自动匹配 v1.x.x 最新
```

### 多工具格式支持

一个技能，多端适配：

```
skill/
├── skill.yaml          # 元数据
├── claude-skill.md     # Claude Code 格式
├── cursor-rule.md      # Cursor 格式
└── copilot-instruct.md # Copilot 格式
```

## 技术栈

| 组件 | 技术 | 说明 |
|------|------|------|
| 后端 | Rust + Axum | 高性能、类型安全 |
| 前端 | Vue 3 + Vite | 快速开发体验 |
| CLI | Rust | 与后端共享代码 |
| 数据库 | PostgreSQL | 含全文搜索 |
| 对象存储 | MinIO | S3 兼容 API |
| 缓存 | Redis | 会话、热点数据 |
| 部署 | Docker Compose | 一键部署 |

## 项目结构

```
skillhub/
├── backend/                 # Rust 后端 API
│   ├── src/
│   │   ├── api/            # API 路由
│   │   ├── models/         # 数据模型
│   │   ├── services/       # 业务逻辑
│   │   ├── repos/          # 数据访问
│   │   ├── middleware/     # 认证中间件
│   │   ├── storage/        # MinIO 存储
│   │   └── utils/          # 工具函数
│   └── migrations/         # 数据库迁移
│
├── cli/                    # Rust CLI 工具
│   └── src/
│       └── commands/       # 命令实现
│
├── web/                    # Vue 3 前端
│   └── src/
│       ├── views/          # 页面组件
│       ├── components/     # UI 组件
│       ├── api/            # API 客户端
│       └── stores/         # 状态管理
│
└── docker-compose.yml      # 容器编排
```

## 快速开始

### 环境要求

- Docker & Docker Compose
- Rust 1.75+ (开发)
- Node.js 18+ (开发)

### 一键部署

```bash
# 克隆项目
git clone https://github.com/JokerYF/SkillHub.git
cd SkillHub

# 启动所有服务
docker-compose up -d

# 查看服务状态
docker-compose ps
```

服务地址：

| 服务 | 地址 | 说明 |
|------|------|------|
| Web 界面 | http://localhost:5173 | 技能市场 |
| API 服务 | http://localhost:3000 | RESTful API |
| MinIO 控制台 | http://localhost:9001 | 对象存储管理 |
| PostgreSQL | localhost:5432 | 数据库 |
| Redis | localhost:6379 | 缓存 |

### 本地开发

```bash
# 后端开发
cd backend
cp .env.example .env
cargo run

# 前端开发
cd web
npm install
npm run dev

# CLI 开发
cd cli
cargo run -- --help
```

## CLI 使用

### 安装

```bash
# 从源码编译
cd cli
cargo install --path .

# 或下载预编译二进制
# 发布后提供下载链接
```

### 命令

```bash
# 初始化配置
skillhub init

# 列出可用技能
skillhub list

# 搜索技能
skillhub search python

# 下载技能（支持 Docker Tag 语法）
skillhub pull python-security           # latest 版本
skillhub pull python-security:v1.0.0    # 指定版本
skillhub pull python-security:v1        # 前缀匹配

# 查看技能详情
skillhub show python-security

# 管理标签
skillhub tag python-security list       # 列出标签
skillhub tag python-security add v1.0.0 stable  # 添加标签
skillhub tag python-security rm old     # 删除标签
```

### 配置文件

配置文件位于 `~/.skillhub/config.yaml`：

```yaml
api_url: http://localhost:3000/api
skills_dir: ~/.skillhub/skills
token: your-jwt-token
```

## API 文档

### 认证

```http
POST /api/auth/register
Content-Type: application/json

{
  "username": "testuser",
  "email": "test@example.com",
  "password": "password123"
}
```

```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "test@example.com",
  "password": "password123"
}

# 响应
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### 技能管理

```http
# 获取技能列表
GET /api/skills?q=python&tags=security&page=1&sort=downloads

# 获取技能详情
GET /api/skills/:slug

# 创建技能（需认证）
POST /api/skills
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "Python Security",
  "slug": "python-security",
  "description": "Python 安全编码规范",
  "tags": ["python", "security"]
}

# 更新技能（需认证）
PUT /api/skills/:slug

# 删除技能（需认证）
DELETE /api/skills/:slug
```

### 版本管理

```http
# 创建版本（需认证）
POST /api/skills/:slug/versions
Authorization: Bearer <token>
Content-Type: application/json

{
  "version": "v1.0.0",
  "content": "# Skill content...",
  "changelog": "Initial release"
}

# 获取版本列表
GET /api/skills/:slug/versions

# 获取指定版本
GET /api/skills/:slug/versions/:version

# 下载版本内容
GET /api/skills/:slug/versions/:version/download
```

### 标签管理

```http
# 获取标签列表
GET /api/skills/:slug/tags

# 创建标签（需认证）
POST /api/skills/:slug/tags
Authorization: Bearer <token>
Content-Type: application/json

{
  "tag": "stable",
  "version": "v1.0.0"
}

# 删除标签（需认证）
DELETE /api/skills/:slug/tags/:tag
```

### 获取清单

```http
# 获取技能完整 manifest
GET /api/skills/:slug/manifest

# 响应
{
  "skill_id": "uuid",
  "name": "Python Security",
  "tags": {
    "latest": "v1.2.0",
    "stable": "v1.0.0"
  },
  "versions": ["v1.2.0", "v1.1.0", "v1.0.0"],
  "updated_at": "2026-03-22T10:00:00Z"
}
```

## 开发指南

### 数据库迁移

```bash
# 迁移文件位于 backend/migrations/
# 自动在容器启动时执行

# 手动执行
psql -U skillhub -d skillhub -f migrations/001_init.sql
psql -U skillhub -d skillhub -f migrations/002_skills.sql
psql -U skillhub -d skillhub -f migrations/003_fulltext_search.sql
```

### 环境变量

后端 `.env` 配置：

```env
# 服务器
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# 数据库
DATABASE_URL=postgres://skillhub:skillhub123@localhost:5432/skillhub

# JWT
JWT_SECRET=your-secret-key-change-in-production

# MinIO
MINIO_ENDPOINT=http://localhost:9000
MINIO_ACCESS_KEY=skillhub
MINIO_SECRET_KEY=skillhub123
MINIO_BUCKET=skills
```

### 代码规范

- **Rust**: 遵循 `rustfmt` 和 `clippy` 规范
- **TypeScript/Vue**: 使用 ESLint + Prettier
- **提交**: 采用 Conventional Commits 格式

## 技能格式

### skill.yaml 示例

```yaml
id: python-security
name: Python 安全编码规范
version: 2.1.0
description: 基于 OWASP 和实战经验的安全编码技能
author: security-team
tags:
  - python
  - security
  - owasp

extends: security-base          # 继承
composes:                       # 组合
  - bandit-rules
  - owasp-top10

visibility: company             # public / company / department / private
```

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License

---

<p align="center">
  Made with ❤️ by SkillHub Team
</p>