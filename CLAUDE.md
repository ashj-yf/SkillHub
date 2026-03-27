# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

Skills Intelligence Hub 是一个企业级 AI 技能管理平台，核心理念是"让技能主动找到用户"。支持 Claude Code、Cursor、GitHub Copilot 等多种 AI 编程工具的技能模板管理、分发和智能推荐。

**目标用户**：中型到大型团队（20-100+ 人），多部门/多业务线组织。

需求文档和规划位于 `docs/requirements/` 目录（软连接到本地独立仓库）。

## 技术栈

| 组件 | 技术 | 说明 |
|------|------|------|
| 后端 | Rust + Axum | 高性能、类型安全、CLI 代码复用 |
| 前端 | Vue 3 + Vite | 快速开发、构建迅速 |
| CLI | Rust | 与后端共享代码 |
| 关系数据库 | PostgreSQL | 含全文搜索 (tsvector + pg_trgm) |
| 对象存储 | MinIO | S3 兼容 API、轻量级 |
| 缓存 | Redis | 会话、热点数据缓存 |
| 部署 | Docker Compose | 中小规模部署 |

## 架构分层

```
┌─────────────────────────────────────────────────────┐
│                    分发层 (Delivery)                  │
│   CLI 工具  │  Web 市场  │  API/SDK  │  IDE 插件    │
└─────────────────────────────────────────────────────┘
                         │
┌─────────────────────────────────────────────────────┐
│                    服务层 (Service)                   │
│   认证授权  │  版本管理  │  搜索推荐  │  权限控制    │
└─────────────────────────────────────────────────────┘
                         │
┌─────────────────────────────────────────────────────┐
│                    存储层 (Storage)                   │
│   PostgreSQL  │  MinIO  │  Redis                    │
└─────────────────────────────────────────────────────┘
```

## 项目结构

```
backend/           # Rust + Axum 后端 API
├── src/
│   ├── api/       # API 路由 (auth.rs, skills.rs, users.rs)
│   ├── models/    # 数据模型
│   ├── services/  # 业务逻辑
│   ├── repos/     # 数据访问
│   ├── middleware/# 认证中间件
│   └── utils/     # JWT、错误处理
└── migrations/    # 数据库迁移

cli/               # Rust CLI 工具
├── src/
│   ├── commands/  # init, list, pull, search, show, tag
│   └── config.rs
└── Cargo.toml

web/               # Vue 3 + Vite 前端
├── src/
│   ├── views/     # Market, SkillDetail, Admin, Login, Register
│   ├── components/
│   └── stores/
└── vite.config.ts

docs/requirements/ # 需求文档（软连接，不推送）
```

## 搜索方案

采用 PostgreSQL 全文搜索，无需向量化：

| 能力 | 实现方式 |
|------|----------|
| 关键词搜索 | tsvector + tsquery |
| 模糊匹配 | pg_trgm (trigram) |
| 搜索排序 | ts_rank |
| 多字段搜索 | setweight 权重控制 |

## 推荐方案

基于规则推荐，无需向量化：

| 策略 | 权重 | 说明 |
|------|------|------|
| 标签匹配 | 40% | 技能标签与项目技术栈匹配 |
| 热度排序 | 30% | 下载量、评分 |
| 使用频率 | 20% | 团队使用记录 |
| 时效性 | 10% | 最近更新时间 |

## 认证方案

- 内置用户系统 + JWT Token
- 支持扩展：LDAP、OAuth 2.0、SAML
- 支持 API Key

## 技能格式

每个技能包含：
- `skill.yaml` - 元数据（支持 extends 继承、composes 组合）
- `claude-skill.md` - Claude Code 格式
- `cursor-rule.md` - Cursor 格式
- `copilot-instruct.md` - Copilot 格式

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

## 版本管理

采用 Docker Tag 模式：
- `skill-slug:latest` - 最新版本
- `skill-slug:v1.0.0` - 指定版本
- `skill-slug:v1` - 自动匹配 v1.x.x 最新

## 项目版本号规范

采用语义化版本 (SemVer)：`vMAJOR.MINOR.PATCH`

| 类型 | 说明 | 示例 |
|------|------|------|
| MAJOR | 不兼容的 API 变更 | v1.0.0 → v2.0.0 |
| MINOR | 向后兼容的功能新增 | v0.1.0 → v0.2.0 |
| PATCH | 向后兼容的问题修复 | v0.1.0 → v0.1.1 |

### 发布流程

1. **确认当前版本**：`git tag -l | sort -V | tail -1`
2. **确定新版本号**：根据变更类型递增
3. **创建并推送 tag**：`git tag v0.x.x && git push origin v0.x.x`
4. **自动构建**：GitHub Action 自动构建并推送镜像

### 注意事项

- **禁止重复发布**：同一提交不应创建多个 tag
- **版本号递增**：新版本必须大于当前最大版本号
- **tag 格式**：必须以 `v` 开头，如 `v0.1.0`
- **禁止测试创建 tag**：测试场景下禁止创建 tag，除非明确要求

## 需求文档目录

`docs/requirements/` 是软连接到本地独立仓库，不推送到 GitHub。

```bash
# 软连接位置
docs/requirements -> /Users/yf/Desktop/skills-hub-project

# 更新需求文档
cd /Users/yf/Desktop/skills-hub-project && git pull

# .gitignore 已排除 docs/ 目录
```

## 功能需求清单

### Phase 1 (MVP)
<!-- PM 标注：已实现 (2026-03-20 by PM) -->
- [x] 用户认证（注册/登录/JWT）

<!-- PM 标注：已实现 (2026-03-21 by PM) -->
- [x] 技能 CRUD

<!-- PM 标注：已实现 (2026-03-22 by PM) -->
- [x] 技能版本管理（Docker Tag 模式）

<!-- PM 标注：已实现 (2026-03-22 by PM) -->
- [x] CLI 基础命令（init/list/pull/search/show/tag）

<!-- PM 标注：已实现 (2026-03-23 by PM) -->
- [x] Web 技能市场

<!-- PM 标注：已实现 (2026-03-25 by PM) -->
- [x] 权限控制（RBAC）- 完整实现，所有 API 已集成权限检查

<!-- PM 标注：已实现 (2026-03-25 by PM) -->
- [x] 部门管理 - 后端 API 已完成

### Phase 2
- [ ] 项目上下文感知
- [ ] 智能推荐
- [ ] 多格式支持（Cursor/Copilot）
- [ ] 技能继承/组合

### Phase 3
- [ ] IDE 插件
- [ ] 评分系统
- [ ] API/SDK

## 团队协作

此项目使用 `/team` 命令触发团队协作模式。

| 角色 | 职责 |
|------|------|
| 架构师 | 技术决策、架构设计、Code Review |
| 后端开发 | API、CLI 开发 (Rust + Axum) |
| 前端开发 | Web UI 开发 (Vue 3 + Vite) |
| 测试工程师 | 测试用例设计、质量保障 |

### 使用方式

```
/team 实现 user-auth 功能    # 完整开发流程
/team review src/backend/    # 代码审查
/team test user-auth 模块    # 测试验证
```

详细配置见 `.claude/team.yaml`