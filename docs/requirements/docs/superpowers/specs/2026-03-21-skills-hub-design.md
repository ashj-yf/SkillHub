# Skills Intelligence Hub - 设计文档

> 创建时间：2026-03-21
> 状态：待审核

---

## 一、项目概述

### 1.1 目标

构建企业级 AI 技能管理平台，实现技能的集中管理、智能分发和多端访问。

### 1.2 核心理念

让技能主动找到用户，而非用户被动搜索。

### 1.3 开发阶段

| 阶段 | 目标 | 核心交付 |
|------|------|---------|
| Phase 1 | Web 技能管理平台 | 上传、下载、检索、Web UI |
| Phase 2 | CLI 命令行工具 | 本地下载、搜索、用户操作 |
| Phase 3 | MCP 远端加载 | 无需下载，实时获取技能 |
| Phase 4 | 权限管理系统 | 用户组、角色、细粒度权限 |

---

## 二、技术架构

### 2.1 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                      Skills Hub Server                       │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────────┐   │
│  │ Web UI  │  │REST API │  │MCP Server│  │  CLI SDK    │   │
│  │ (Vue)   │  │ (Rust)  │  │(fastmcp)│  │  (Rust)     │   │
│  └────┬────┘  └────┬────┘  └────┬────┘  └──────┬──────┘   │
│       │            │            │              │           │
│       └────────────┴────────────┴──────────────┘           │
│                          │                                   │
│              ┌───────────┴───────────┐                      │
│              │    Business Logic     │                      │
│              │  (技能管理、权限...)    │                      │
│              └───────────┬───────────┘                      │
│                          │                                   │
└──────────────────────────┼───────────────────────────────────┘
                           │
          ┌────────────────┼────────────────┐
          ▼                ▼                ▼
     ┌─────────┐     ┌─────────┐     ┌─────────┐
     │PostgreSQL│     │  MinIO  │     │  Redis  │
     │ (元数据) │     │ (文件)  │     │ (缓存)  │
     └─────────┘     └─────────┘     └─────────┘
```

### 2.2 技术栈

| 组件 | 技术选型 | 说明 |
|------|---------|------|
| 后端 | Rust + Axum | 高性能、类型安全 |
| 前端 | Vue 3 + Vite + Naive UI | 开发效率高 |
| CLI | Rust + clap | 与后端共享代码 |
| MCP Server | Python + fastmcp | 快速实现 |
| 关系数据库 | PostgreSQL 15 | 元数据、权限 |
| 对象存储 | MinIO | 技能文件存储 |
| 缓存 | Redis | 热点数据缓存 |

---

## 三、技能格式

### 3.1 目录结构

```
skill-name/
├── skill.yaml          # 元数据
├── skill.md            # 内容
└── assets/             # 可选：图片等资源
```

### 3.2 元数据格式（skill.yaml）

```yaml
id: python-security
name: Python 安全编码规范
version: 1.0.0
description: 基于 OWASP 的安全编码最佳实践
author: security-team
tags:
  - python
  - security
  - owasp
category: security
visibility: public      # public / internal / department / team / private
```

### 3.3 版本管理（Docker Tag 模式）

**技能引用格式：** `{skill-id}:{tag}`

```
python-security:latest     # 最新版本
python-security:v1.0.0     # 指定版本
python-security:v1         # 版本前缀（自动匹配 v1.x.x 最新）
python-security:stable     # 稳定版标签
```

**Tag 规则：**

| Tag 类型 | 示例 | 说明 |
|----------|------|------|
| latest | `:latest` | 默认标签，指向最新发布版本 |
| 语义版本 | `:v1.0.0` | 完整版本号 |
| 主版本 | `:v1` | 指向 v1.x.x 最新版本 |
| 预发布 | `:v2.0.0-beta.1` | 预发布版本 |
| 自定义 | `:stable`, `:dev` | 用户定义的命名标签 |

### 3.4 存储结构（MinIO）

```
buckets/
└── skills/
    └── {skill-id}/
        ├── manifest.json           # 标签索引
        ├── v1.0.0/                 # 版本目录（内容寻址）
        │   ├── skill.yaml
        │   ├── skill.md
        │   └── assets/
        ├── v1.1.0/
        │   └── ...
        └── v2.0.0/
            └── ...
```

**manifest.json 示例：**

```json
{
  "skill_id": "python-security",
  "name": "Python 安全编码规范",
  "tags": {
    "latest": "v2.0.0",
    "stable": "v2.0.0",
    "v2": "v2.0.0",
    "v2.0": "v2.0.0",
    "v2.0.0": "v2.0.0",
    "v1": "v1.1.0",
    "v1.0": "v1.0.0",
    "v1.0.0": "v1.0.0",
    "dev": "v2.1.0-beta.1"
  },
  "versions": ["v2.0.0", "v1.1.0", "v1.0.0", "v2.1.0-beta.1"],
  "updated_at": "2026-03-21T10:00:00Z"
}
```

---

## 四、Phase 1：Web 技能管理平台

### 4.1 功能范围

| 用户角色 | 权限 |
|---------|------|
| 管理员 | 上传、编辑、删除技能 |
| 普通用户 | 浏览、搜索、下载技能 |
| 未登录用户 | 浏览、搜索（下载需登录） |

管理员账号通过配置文件或命令行创建。

### 4.2 Web 页面

| 页面 | 功能 |
|------|------|
| 首页 | 热门技能、最新技能、分类入口 |
| 技能市场 | 列表、搜索、筛选、排序 |
| 技能详情 | 基本信息、内容预览、版本历史、下载 |
| 管理后台 | 技能管理（管理员） |

### 4.3 版本管理

- 默认下载 `:latest` 标签版本
- 支持指定版本：`skill pull python-security:v1.0.0`
- 管理员可管理标签（创建、移动、删除）
- 支持 `v1` 短版本号，自动指向 `v1.x.x` 最新版本

### 4.4 搜索能力

- 按名称、标签、描述关键词搜索
- 分类浏览
- 热门/最新排序

### 4.5 数据模型

```sql
-- 技能表
CREATE TABLE skills (
  id UUID PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  author VARCHAR(100),
  category VARCHAR(50),
  visibility VARCHAR(20) DEFAULT 'public',
  status VARCHAR(20) DEFAULT 'published',
  downloads INT DEFAULT 0,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

-- 技能版本表（存储实际版本内容）
CREATE TABLE skill_versions (
  id UUID PRIMARY KEY,
  skill_id UUID REFERENCES skills(id),
  version VARCHAR(50) NOT NULL,       -- v1.0.0, v2.1.0-beta.1
  storage_path VARCHAR(500) NOT NULL,
  change_note TEXT,
  digest VARCHAR(64),                 -- 内容哈希，类似 Docker digest
  created_at TIMESTAMP DEFAULT NOW(),
  created_by VARCHAR(100),
  UNIQUE(skill_id, version)
);

-- 技能标签表（类似 Docker tag）
CREATE TABLE skill_tags (
  id UUID PRIMARY KEY,
  skill_id UUID REFERENCES skills(id),
  tag VARCHAR(50) NOT NULL,           -- latest, stable, v1, v1.0.0
  version_id UUID REFERENCES skill_versions(id),
  updated_at TIMESTAMP DEFAULT NOW(),
  updated_by VARCHAR(100),
  UNIQUE(skill_id, tag)
);

-- 创建索引
CREATE INDEX idx_skill_tags_skill ON skill_tags(skill_id);
CREATE INDEX idx_skill_versions_skill ON skill_versions(skill_id);

-- 标签表
CREATE TABLE tags (
  id UUID PRIMARY KEY,
  name VARCHAR(50) UNIQUE NOT NULL
);

-- 技能-标签关联
CREATE TABLE skill_tags (
  skill_id UUID REFERENCES skills(id),
  tag_id UUID REFERENCES tags(id),
  PRIMARY KEY (skill_id, tag_id)
);

-- 用户表（简单）
CREATE TABLE users (
  id UUID PRIMARY KEY,
  username VARCHAR(50) UNIQUE NOT NULL,
  email VARCHAR(100) UNIQUE NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  role VARCHAR(20) DEFAULT 'user',
  created_at TIMESTAMP DEFAULT NOW()
);
```

---

## 五、Phase 2：CLI 命令行工具

### 5.1 命令设计

```bash
# 初始化
skill init --server https://skills.company.com

# 搜索与浏览
skill search <keyword>
skill search --tag <tag>
skill list
skill list --category <category>
skill info <skill-id>

# 下载（Docker Tag 风格）
skill pull <skill-id>                  # 默认 :latest
skill pull <skill-id>:latest           # 最新版本
skill pull <skill-id>:v1.0.0           # 指定版本
skill pull <skill-id>:v1               # 主版本最新
skill pull <skill-id>:stable           # 自定义标签
skill pull <skill-id> --output ./path/

# 标签管理（管理员/创作者）
skill tag <skill-id>                   # 查看所有标签
skill tag <skill-id> add v1.0.0 stable # 添加标签
skill tag <skill-id> rm beta           # 删除标签

# 用户操作
skill login
skill logout
skill submit ./skill-dir/
skill submit ./skill-dir/ --update
skill my
skill my --status pending
```

### 5.2 本地配置

```
~/.skillhub/
├── config.yaml           # 服务器地址
├── cache/
│   └── skills.json       # 技能列表缓存
└── credentials           # 登录凭证（加密）
```

### 5.3 项目内技能

```
project/
└── .skills/
    └── python-security/
        ├── skill.yaml
        └── skill.md
```

### 5.4 认证

- JWT Token 认证
- Token 加密存储在本地
- 过期后提示重新登录

---

## 六、Phase 3：MCP 远端加载

### 6.1 架构

```
┌─────────────────┐         ┌─────────────────┐         ┌─────────────────┐
│   Claude Code   │         │  MCP Server     │         │  Skills Hub     │
│   (MCP Client)  │◄─MCP───▶│  (fastmcp)      │◄─REST──▶│  API (Rust)     │
└─────────────────┘         └─────────────────┘         └─────────────────┘
```

### 6.2 MCP Tools

| Tool | 描述 | 参数 |
|------|------|------|
| `search_skills` | 搜索技能 | keyword, tags, category, limit |
| `get_skill` | 获取技能内容 | skill_id, tag (default: latest) |
| `list_skills` | 列出技能 | category, sort |
| `list_tags` | 列出技能标签 | skill_id |

### 6.3 MCP Resources

| Resource | 描述 |
|----------|------|
| `skills://list` | 技能列表 |
| `skills://{skill-id}` | 技能详情（默认 latest） |
| `skills://{skill-id}:{tag}` | 指定标签版本 |
| `skills://{skill-id}/content` | 技能内容（latest） |
| `skills://{skill-id}/content:{tag}` | 指定版本内容 |

### 6.4 用户配置

```json
{
  "mcpServers": {
    "skills-hub": {
      "command": "skillhub-mcp",
      "args": ["--server", "https://skills.company.com"],
      "env": {
        "SKILLS_HUB_TOKEN": "user-api-token"
      }
    }
  }
}
```

### 6.5 MCP Server 实现（fastmcp）

```python
from fastmcp import FastMCP
import httpx

mcp = FastMCP("Skills Hub")
API_BASE = "https://skills.company.com/api/v1"

@mcp.tool()
async def search_skills(keyword: str, tags: list[str] = None, limit: int = 10):
    """搜索 Skills Hub 中的技能"""
    params = {"keyword": keyword, "limit": limit}
    if tags:
        params["tags"] = ",".join(tags)
    async with httpx.AsyncClient() as client:
        resp = await client.get(f"{API_BASE}/skills/search", params=params)
        return resp.json()

@mcp.tool()
async def get_skill(skill_id: str, tag: str = "latest"):
    """获取指定技能的完整内容，默认获取最新版本"""
    async with httpx.AsyncClient() as client:
        resp = await client.get(f"{API_BASE}/skills/{skill_id}/{tag}")
        return resp.json()

@mcp.tool()
async def list_tags(skill_id: str):
    """列出技能的所有可用标签"""
    async with httpx.AsyncClient() as client:
        resp = await client.get(f"{API_BASE}/skills/{skill_id}/tags")
        return resp.json()

@mcp.resource("skills://list")
async def list_skills():
    """列出所有公开技能"""
    async with httpx.AsyncClient() as client:
        resp = await client.get(f"{API_BASE}/skills")
        return resp.json()
```

---

## 七、Phase 4：权限管理系统

### 7.1 权限模型

RBAC + 资源级权限混合模型：

```
用户 ──属于──▶ 用户组 ──拥有──▶ 角色 ──包含──▶ 权限
  │                                          │
  └──────────直接授权─────────────────────────┘
```

### 7.2 角色体系

| 角色 | 描述 | 权限 |
|------|------|------|
| 超级管理员 | 系统管理 | 全部权限 |
| 部门管理员 | 管理本部门 | 部门用户管理、技能审核 |
| 技能创作者 | 提交技能 | 创建、编辑自己的技能 |
| 普通用户 | 使用技能 | 浏览、搜索、下载公开技能 |

### 7.3 技能可见性

| 可见性 | 访问范围 |
|--------|---------|
| public | 所有用户 |
| internal | 登录用户 |
| department | 指定部门成员 |
| team | 指定团队成员 |
| private | 仅创建者和授权用户 |

### 7.4 操作权限

| 权限 | 描述 |
|------|------|
| `skill:read` | 查看技能 |
| `skill:download` | 下载技能 |
| `skill:create` | 创建技能 |
| `skill:update` | 更新技能 |
| `skill:delete` | 删除技能 |
| `skill:publish` | 发布技能 |
| `skill:approve` | 审核技能 |
| `user:read` | 查看用户 |
| `user:manage` | 管理用户 |
| `group:manage` | 管理用户组 |

### 7.5 数据模型扩展

```sql
-- 用户组表
CREATE TABLE groups (
  id UUID PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  description TEXT,
  type VARCHAR(20),        -- department / team / custom
  parent_id UUID REFERENCES groups(id),
  created_at TIMESTAMP DEFAULT NOW()
);

-- 用户-用户组关联
CREATE TABLE user_groups (
  user_id UUID REFERENCES users(id),
  group_id UUID REFERENCES groups(id),
  role VARCHAR(20) DEFAULT 'member',  -- member / admin
  joined_at TIMESTAMP DEFAULT NOW(),
  PRIMARY KEY (user_id, group_id)
);

-- 角色表
CREATE TABLE roles (
  id UUID PRIMARY KEY,
  name VARCHAR(50) UNIQUE NOT NULL,
  description TEXT,
  is_system BOOLEAN DEFAULT FALSE
);

-- 权限表
CREATE TABLE permissions (
  id UUID PRIMARY KEY,
  name VARCHAR(50) UNIQUE NOT NULL,
  description TEXT,
  resource VARCHAR(50),
  action VARCHAR(50)
);

-- 角色-权限关联
CREATE TABLE role_permissions (
  role_id UUID REFERENCES roles(id),
  permission_id UUID REFERENCES permissions(id),
  PRIMARY KEY (role_id, permission_id)
);

-- 用户-角色关联
CREATE TABLE user_roles (
  user_id UUID REFERENCES users(id),
  role_id UUID REFERENCES roles(id),
  PRIMARY KEY (user_id, role_id)
);

-- 技能访问控制
CREATE TABLE skill_acls (
  skill_id UUID REFERENCES skills(id),
  principal_type VARCHAR(20),  -- user / group / role
  principal_id UUID,
  permission VARCHAR(50),
  granted_by UUID REFERENCES users(id),
  granted_at TIMESTAMP DEFAULT NOW(),
  PRIMARY KEY (skill_id, principal_type, principal_id, permission)
);

-- 审计日志
CREATE TABLE audit_logs (
  id UUID PRIMARY KEY,
  user_id UUID,
  action VARCHAR(100),
  resource_type VARCHAR(50),
  resource_id UUID,
  details JSONB,
  ip_address VARCHAR(50),
  created_at TIMESTAMP DEFAULT NOW()
);
```

### 7.6 Web 管理页面

| 页面 | 功能 |
|------|------|
| 用户管理 | 列表、创建、禁用、重置密码 |
| 用户组管理 | 创建部门/团队、添加成员 |
| 角色管理 | 查看、自定义角色 |
| 技能权限 | 设置可见性、授权访问 |
| 审计日志 | 操作记录查看 |

### 7.7 CLI 管理命令

```bash
# 用户管理
skill admin users list
skill admin users create --username <name> --email <email>
skill admin users disable <username>

# 用户组管理
skill admin groups create --name <name> --type team
skill admin groups add-member <group> <user>

# 角色管理
skill admin roles list
skill admin roles grant <user> <role>

# 技能权限
skill permission set <skill> --visibility department --department <dept>
skill permission grant <skill> --user <user> --permission read
```

---

## 八、API 设计概要

### 8.1 REST API 端点

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | `/api/v1/skills` | 技能列表 |
| GET | `/api/v1/skills/search` | 搜索技能 |
| GET | `/api/v1/skills/{id}` | 技能详情（默认 latest） |
| GET | `/api/v1/skills/{id}/{tag}` | 指定标签版本 |
| GET | `/api/v1/skills/{id}/tags` | 标签列表 |
| POST | `/api/v1/skills/{id}/tags` | 创建标签 |
| DELETE | `/api/v1/skills/{id}/tags/{tag}` | 删除标签 |
| GET | `/api/v1/skills/{id}/{tag}/download` | 下载技能 |
| POST | `/api/v1/skills` | 创建技能 |
| PUT | `/api/v1/skills/{id}` | 更新技能 |
| DELETE | `/api/v1/skills/{id}` | 删除技能 |
| POST | `/api/v1/auth/login` | 登录 |
| POST | `/api/v1/auth/logout` | 登出 |
| GET | `/api/v1/users/me` | 当前用户信息 |
| GET | `/api/v1/users/me/skills` | 我的技能 |

### 8.2 技能引用解析

```
python-security           → skill_id=python-security, tag=latest
python-security:latest    → skill_id=python-security, tag=latest
python-security:v1.0.0    → skill_id=python-security, tag=v1.0.0
python-security:v1        → skill_id=python-security, tag=v1
```

### 8.3 API 示例

```bash
# 获取最新版本
GET /api/v1/skills/python-security/latest

# 获取指定版本
GET /api/v1/skills/python-security/v1.0.0

# 获取主版本最新
GET /api/v1/skills/python-security/v1
# 返回 v1.x.x 中最新的版本内容

# 列出所有标签
GET /api/v1/skills/python-security/tags
# 返回: {"tags": ["latest", "stable", "v1", "v1.0.0", "v1.1.0", "v2", "v2.0.0"]}

# 创建新标签
POST /api/v1/skills/python-security/tags
{"tag": "beta", "version": "v2.1.0-beta.1"}
```

---

## 九、部署架构

### 9.1 Docker Compose

```yaml
version: '3.8'
services:
  api:
    build: ./backend
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=postgres://user:pass@postgres:5432/skillshub
      - MINIO_ENDPOINT=minio:9000
      - REDIS_URL=redis://redis:6379
    depends_on:
      - postgres
      - minio
      - redis

  web:
    build: ./web
    ports:
      - "80:80"
    depends_on:
      - api

  mcp:
    build: ./mcp
    ports:
      - "3001:3001"
    environment:
      - API_BASE=http://api:3000
    depends_on:
      - api

  postgres:
    image: postgres:15
    volumes:
      - postgres_data:/var/lib/postgresql/data
    environment:
      - POSTGRES_DB=skillshub
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=pass

  minio:
    image: minio/minio
    command: server /data --console-address ":9001"
    volumes:
      - minio_data:/data
    ports:
      - "9000:9000"
      - "9001:9001"

  redis:
    image: redis:7
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  minio_data:
  redis_data:
```

---

## 十、待确认事项

- [ ] 前端 UI 设计稿
- [ ] 具体部署环境
- [ ] 与现有系统的集成需求

---

*文档创建于 2026-03-21*