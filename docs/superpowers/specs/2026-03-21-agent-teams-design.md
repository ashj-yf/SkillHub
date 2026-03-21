# Agent Teams 设计文档

> 创建时间：2026-03-21
> 状态：已批准

---

## 一、概述

为 Skills Intelligence Hub 项目创建一个 agent teams 系统，通过配置文件定义团队成员，配合工作流 Skill 驱动不同角色的 subagent 协作完成开发测试任务。

### 目标

- 定义项目开发所需的团队成员角色
- 支持顺序和并行两种协作模式
- 提供手动触发和自动加载两种触发方式

---

## 二、文件结构

```
.claude/
├── team.yaml              # 团队成员配置
└── skills/
    └── team.md            # /team 工作流 Skill
```

---

## 三、team.yaml 配置

### 3.1 完整配置

```yaml
name: skills-hub-team
description: Skills Intelligence Hub 开发团队

members:
  - id: architect
    name: 架构师
    role: Architect
    description: 负责技术决策、架构设计、Code Review
    expertise:
      - 系统架构设计
      - 技术选型
      - 代码审查
    prompts:
      review: "作为架构师，审查以下代码/设计..."
      design: "作为架构师，为以下需求设计技术方案..."

  - id: backend
    name: 后端开发
    role: Backend Developer
    description: 负责 API、CLI 开发，使用 Rust + Axum
    expertise:
      - Rust 编程
      - API 设计
      - CLI 工具开发
    tech_stack:
      - Rust
      - Axum
      - PostgreSQL
    prompts:
      implement: "作为后端开发，实现以下功能..."

  - id: frontend
    name: 前端开发
    role: Frontend Developer
    description: 负责 Web UI 开发，使用 Vue 3 + Vite
    expertise:
      - Vue 3 开发
      - 组件设计
      - 前端工程化
    tech_stack:
      - Vue 3
      - Vite
      - TypeScript
    prompts:
      implement: "作为前端开发，实现以下功能..."

  - id: tester
    name: 测试工程师
    role: QA Engineer
    description: 负责测试用例设计、质量保障
    expertise:
      - 测试用例设计
      - 自动化测试
      - 质量保障流程
    prompts:
      test: "作为测试工程师，为以下功能设计测试用例..."

workflows:
  sequential:
    - architect
    - backend
    - frontend
    - tester

  parallel_groups:
    - [backend, frontend]
```

### 3.2 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | string | 成员唯一标识，用于工作流引用 |
| `name` | string | 显示名称 |
| `role` | string | 角色类型 |
| `description` | string | 角色职责描述 |
| `expertise` | string[] | 专业领域 |
| `tech_stack` | string[] | 技术栈（可选） |
| `prompts` | object | 预定义提示词模板 |

---

## 四、team.md Skill

### 4.1 Skill 内容

```markdown
---
name: team
description: 触发团队协作模式，根据任务类型派发对应角色的 agent
---

## 触发方式

- 手动：`/team <任务描述>`
- 自动：CLAUDE.md 中配置自动加载

## 工作流程

### 1. 解析任务
分析任务类型，确定需要哪些角色参与。

### 2. 选择执行模式

**顺序模式**（适用于有依赖的任务）：
架构设计 → 后端开发 → 前端开发 → 测试

**并行模式**（适用于独立任务）：
- 后端和前端可同时工作
- 测试在开发完成后执行

### 3. 派发 Agent

使用 Agent 工具派发对应角色的 subagent：
- 读取 `.claude/team.yaml` 获取角色配置
- 注入角色的 expertise 和 prompts
- 执行任务并收集结果

### 4. 结果汇总

汇总各 agent 的输出，生成工作报告。
```

---

## 五、工作流执行逻辑

### 5.1 执行流程图

```
用户输入: /team 实现 user-auth 功能

     ┌─────────────────────────────────────┐
     │           任务解析                   │
     │  识别: 需要架构设计 + 后端 + 测试     │
     └────────────────┬────────────────────┘
                      │
                      ▼
     ┌─────────────────────────────────────┐
     │        Phase 1: 架构设计              │
     │     派发 architect agent             │
     │     输出: 技术方案文档                │
     └────────────────┬────────────────────┘
                      │
                      ▼
     ┌─────────────────────────────────────┐
     │        Phase 2: 并行开发              │
     │  ┌────────────┐  ┌────────────┐     │
     │  │  backend   │  │  frontend  │     │
     │  │   agent    │  │   agent    │     │
     │  └────────────┘  └────────────┘     │
     └────────────────┬────────────────────┘
                      │
                      ▼
     ┌─────────────────────────────────────┐
     │        Phase 3: 测试验证              │
     │       派发 tester agent              │
     │       输出: 测试报告                  │
     └─────────────────────────────────────┘
```

### 5.2 模式选择规则

| 任务类型 | 执行模式 | 参与角色 |
|----------|----------|----------|
| 新功能开发 | 混合 | 架构师 → 后端/前端并行 → 测试 |
| 代码审查 | 顺序 | 架构师 |
| Bug 修复 | 顺序 | 后端/前端 → 测试 |
| 测试用例 | 顺序 | 测试 |
| 架构设计 | 顺序 | 架构师 |

---

## 六、触发机制

### 6.1 手动触发

```
/team 实现 user-auth 功能
/team review src/backend/auth.rs
/team test user-auth 模块
```

### 6.2 自动加载

在 `CLAUDE.md` 中添加配置：

```markdown
## 团队协作

此项目使用 team skill 进行团队协作开发。
- 开发任务自动触发团队工作流
- 代码审查由架构师 agent 负责
```

---

## 七、团队成员角色来源

根据 `docs/requirements/docs/04-开发计划.md` 中的人员配置：

| 角色 | 人数 | 职责 |
|------|------|------|
| 后端开发 | 1-2 | API、CLI 开发 |
| 前端开发 | 1 | Web UI 开发 |
| 架构师 | 0.5 | 技术决策、Code Review |
| 测试 | 0.5 | 测试用例、质量保障 |

---

## 八、实现要点

1. **配置读取**：Skill 启动时读取 `team.yaml`，解析成员配置
2. **任务分析**：根据任务关键词判断需要的角色和执行模式
3. **Agent 派发**：使用 Agent 工具，注入角色上下文
4. **结果汇总**：收集各 agent 输出，生成结构化报告

---

*设计文档由 Claude Code 生成*