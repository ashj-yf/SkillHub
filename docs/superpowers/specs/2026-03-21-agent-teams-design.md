# Agent Teams Skill 设计文档

> 创建时间：2026-03-21
> 状态：设计中

---

## 一、概述

为 Skills Intelligence Hub 项目创建 agent teams skill，实现多角色 agent 协作开发。

### 目标

- 输入 `/team <任务>` 自动分析任务类型
- 根据任务类型智能选择参与角色
- 支持顺序和并行两种执行模式
- 各角色定义独立维护，易于扩展

---

## 二、架构设计

### 2.1 文件结构

```
.claude/
├── team.yaml              # 成员定义 + 工作流规则
└── skills/
    └── team/
        ├── SKILL.md       # 主控制器：任务分析、派发逻辑
        ├── architect.md   # 架构师角色定义
        ├── backend.md     # 后端开发角色定义
        ├── frontend.md    # 前端开发角色定义
        └── tester.md      # 测试工程师角色定义
```

### 2.2 工作原理

```
用户输入: /team 实现 user-auth 功能
         ↓
    ┌────────────────┐
    │   SKILL.md     │  1. 读取 team.yaml
    │   (主控制器)    │  2. 分析任务类型
    └────────┬───────┘  3. 查询工作流规则
             │          4. 派发 agent
             ▼
    ┌────────────────┐
    │   team.yaml    │  成员定义 + 任务规则
    │   (配置中心)    │
    └────────┬───────┘
             │
    ┌────────┴────────┐
    ▼                 ▼
┌─────────┐      ┌─────────┐
│architect│      │ backend │  ... 角色定义
│  .md    │      │  .md    │
└─────────┘      └─────────┘
```

### 2.3 核心流程

1. **解析任务** - 根据关键词匹配任务类型
2. **确定工作流** - 查询 workflow 配置
3. **派发 Agent** - 按顺序或并行执行
4. **注入上下文** - 读取角色 skill 文件
5. **汇总结果** - 生成结构化报告

---

## 三、配置格式

### 3.1 team.yaml

```yaml
name: skills-hub-team
description: Skills Intelligence Hub 开发团队

# 任务类型 → 角色映射规则
task_rules:
  - keywords: [实现, 开发, 添加, 新建]
    type: feature
    workflow: [architect, parallel: [backend, frontend], tester]

  - keywords: [审查, review, 检查]
    type: review
    workflow: [architect]

  - keywords: [测试, test, 验证]
    type: test
    workflow: [tester]

  - keywords: [修复, fix, bug]
    type: bugfix
    workflow: [backend, tester]

  - keywords: [设计, 架构, 方案]
    type: design
    workflow: [architect]

# 成员定义
members:
  - id: architect
    name: 架构师
    skill: .claude/skills/team/architect.md

  - id: backend
    name: 后端开发
    skill: .claude/skills/team/backend.md

  - id: frontend
    name: 前端开发
    skill: .claude/skills/team/frontend.md

  - id: tester
    name: 测试工程师
    skill: .claude/skills/team/tester.md
```

### 3.2 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `task_rules` | array | 任务类型匹配规则 |
| `keywords` | array | 触发关键词 |
| `workflow` | array | 执行顺序，支持 `parallel: [...]` |
| `members` | array | 团队成员定义 |
| `skill` | string | 角色定义文件路径 |

---

## 四、角色定义文件

### 4.1 architect.md

```markdown
---
role: architect
name: 架构师
---

## 角色定位

你是 Skills Intelligence Hub 项目的架构师，负责技术决策、架构设计和代码审查。

## 专业领域

- 系统架构设计
- 技术选型
- API 设计评审
- 代码审查

## 技术栈

- 后端：Rust + Axum + PostgreSQL
- 前端：Vue 3 + Vite + TypeScript
- 基础设施：Docker Compose, Gitea, Qdrant

## 工作原则

1. 保持简单，避免过度设计
2. 优先考虑可维护性
3. 关注安全性和性能
4. 遵循项目现有架构模式

## 输出规范

设计文档应包含：
- 架构图（使用 ASCII 或 Mermaid）
- 接口定义
- 数据流说明
- 关键决策及理由
```

### 4.2 backend.md

```markdown
---
role: backend
name: 后端开发
---

## 角色定位

你是后端开发工程师，负责 API 和 CLI 工具开发。

## 技术栈

- 语言：Rust
- 框架：Axum
- 数据库：PostgreSQL + SQLx
- 异步运行时：Tokio

## 项目结构

backend/
├── src/
│   ├── api/       # API 路由
│   ├── models/    # 数据模型
│   ├── services/  # 业务逻辑
│   └── repos/     # 数据访问
└── migrations/    # 数据库迁移

## 工作原则

1. 遵循 Rust 最佳实践
2. 完善的错误处理
3. 编写单元测试
4. 添加必要的日志
```

### 4.3 frontend.md

```markdown
---
role: frontend
name: 前端开发
---

## 角色定位

你是前端开发工程师，负责 Web UI 开发。

## 技术栈

- 框架：Vue 3
- 构建：Vite
- 语言：TypeScript
- 状态管理：Pinia

## 项目结构

web/
├── src/
│   ├── views/     # 页面组件
│   ├── components/
│   ├── api/       # API 调用
│   └── stores/    # 状态管理
└── vite.config.ts

## 工作原则

1. 使用 Composition API
2. TypeScript 类型定义
3. 组件化开发
4. 响应式设计
```

### 4.4 tester.md

```markdown
---
role: tester
name: 测试工程师
---

## 角色定位

你是测试工程师，负责测试用例设计和质量保障。

## 专业领域

- 单元测试
- 集成测试
- API 测试
- 边界条件覆盖

## 工作原则

1. 测试覆盖核心逻辑
2. 包含正常和异常场景
3. 边界条件必须测试
4. 输出清晰的测试报告
```

---

## 五、主控制器 SKILL.md

```markdown
---
name: team
description: 触发团队协作模式，根据任务类型自动派发对应角色的 agent
---

## 工作流程

### 1. 解析任务
读取 `.claude/team.yaml`，根据关键词匹配任务类型。

### 2. 确定工作流
根据匹配的任务类型，获取对应的 workflow 配置。

### 3. 派发 Agent
按照 workflow 顺序派发 agent：
- **顺序执行**：等待前一个完成后再派发下一个
- **并行执行**：同时派发多个 agent（`parallel: [...]`）

### 4. 注入角色上下文
派发时读取对应的角色 skill 文件，注入：
- 角色身份
- 专业领域
- 技术栈
- 预定义提示词

### 5. 汇总结果
收集各 agent 输出，生成结构化报告。

## 使用示例

/team 实现 user-auth 功能
/team review src/backend/auth.rs
/team 修复登录验证 bug
/team 设计 API 接口
```

---

## 六、团队成员来源

根据 `docs/requirements/docs/04-开发计划.md` 中的人员配置：

| 角色 | 人数 | 职责 |
|------|------|------|
| 后端开发 | 1-2 | API、CLI 开发 |
| 前端开发 | 1 | Web UI 开发 |
| 架构师 | 0.5 | 技术决策、Code Review |
| 测试 | 0.5 | 测试用例、质量保障 |

---

*设计文档由 Claude Code 生成*