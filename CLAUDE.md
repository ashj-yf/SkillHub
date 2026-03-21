# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

Skills Intelligence Hub 是一个企业级 AI 技能管理平台，核心理念是"让技能主动找到用户"。支持 Claude Code、Cursor、GitHub Copilot 等多种 AI 编程工具的技能模板管理、分发和智能推荐。

需求文档和规划位于 `docs/requirements/` 目录（git subtree 挂载）。

## 技术栈

| 组件 | 技术 |
|------|------|
| 后端 | Rust + Axum |
| 前端 | Vue 3 + Vite |
| CLI | Rust |
| 关系数据库 | PostgreSQL |
| 向量数据库 | Qdrant |
| 缓存 | Redis |
| Git 服务 | Gitea |
| 部署 | Docker Compose |

## 项目结构（规划）

```
backend/           # Rust + Axum 后端 API
├── src/
│   ├── api/       # API 路由 (auth.rs, skills.rs, users.rs)
│   ├── models/    # 数据模型
│   ├── services/  # 业务逻辑
│   └── repos/     # 数据访问
└── migrations/    # 数据库迁移

cli/               # Rust CLI 工具
├── src/
│   ├── commands/  # init, list, pull, search, sense
│   └── config.rs
└── Cargo.toml

web/               # Vue 3 + Vite 前端
├── src/
│   ├── views/     # Market, SkillDetail, Admin
│   ├── components/
│   └── stores/
└── vite.config.ts

docs/requirements/ # 需求文档（git subtree）
```

## 架构分层

1. **感知层 (Context)** - 项目检测、意图识别、上下文聚合
2. **智能层 (Intelligence)** - 技能图谱、向量引擎、推荐引擎
3. **存储层 (Storage)** - Git 仓库、向量数据库、关系数据库
4. **分发层 (Delivery)** - CLI、Web 市场、IDE 插件、API/SDK

## 技能格式

每个技能包含：
- `skill.yaml` - 元数据（支持 extends 继承、composes 组合）
- `claude-skill.md` - Claude Code 格式
- `cursor-rule.md` - Cursor 格式
- `copilot-instruct.md` - Copilot 格式

## Subtree 操作

需求文档目录通过 git subtree 管理：

```bash
# 拉取上游更新
git subtree pull --prefix=docs/requirements https://github.com/JokerYF/skills-hub-project.git main --squash

# 推送到上游
git subtree push --prefix=docs/requirements https://github.com/JokerYF/skills-hub-project.git main
```