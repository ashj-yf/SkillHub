---
name: team
description: 触发团队协作模式，根据任务类型派发对应角色的 agent 完成 development 和 testing 任务
---

<SUBAGENT-STOP>
如果作为 subagent 被派发执行特定任务，跳过此 skill。
</SUBAGENT-STOP>

## 触发方式

- **手动触发：** `/team <任务描述>`
- **自动加载：** 在 CLAUDE.md 中配置自动加载

## 使用示例

```
/team 实现 user-auth 功能
/team review src/backend/auth.rs
/team test user-auth 模块
/team 设计 API 接口
```

## 团队成员

团队配置定义在 `.claude/team.yaml`，包含以下角色：

| 角色 | ID | 职责 |
|------|-----|------|
| 架构师 | architect | 技术决策、架构设计、Code Review |
| 后端开发 | backend | API、CLI 开发 (Rust + Axum) |
| 前端开发 | frontend | Web UI 开发 (Vue 3 + Vite) |
| 测试工程师 | tester | 测试用例设计、质量保障 |

## 工作流程

### 1. 任务解析

分析用户输入的任务描述，识别：
- 任务类型（开发/审查/测试/设计）
- 涉及的技术领域（后端/前端/全栈）
- 需要的角色组合

### 2. 模式选择

根据任务类型选择执行模式：

| 任务类型 | 执行模式 | 参与角色顺序 |
|----------|----------|--------------|
| 新功能开发 | 混合 | 架构师 → 后端/前端并行 → 测试 |
| 代码审查 | 顺序 | 架构师 |
| Bug 修复 | 顺序 | 后端/前端 → 测试 |
| 测试用例 | 顺序 | 测试 |
| 架构设计 | 顺序 | 架构师 |

**顺序模式**（有依赖的任务）：
```
架构设计 → 后端开发 → 前端开发 → 测试验证
```

**并行模式**（独立任务）：
```
┌────────────┐
│   后端     │  ──→  同时进行
├────────────┤
│   前端     │  ──→  同时进行
└────────────┘
         ↓
      测试验证
```

### 3. 执行步骤

对于完整的开发任务，按以下步骤执行：

**Phase 1: 架构设计**
- 派发 architect agent
- 输出：技术方案文档、接口定义
- 等待用户确认后继续

**Phase 2: 并行开发**
- 同时派发 backend 和 frontend agent
- 后端：实现 API、数据库、CLI
- 前端：实现 UI 组件、页面
- 各自独立完成后汇报

**Phase 3: 测试验证**
- 派发 tester agent
- 输出：测试用例、测试报告
- 确认功能正常

### 4. Agent 派发

使用 Agent 工具派发 subagent：

```
Agent(
  subagent_type: "general-purpose",
  prompt: "<角色上下文> + <任务描述>",
  description: "<角色名称> - <任务简述>"
)
```

角色上下文注入：
1. 读取 `.claude/team.yaml` 获取角色配置
2. 加载角色的 expertise 和 prompts
3. 注入 tech_stack（如适用）

### 5. 结果汇总

汇总各 agent 输出，生成结构化报告：
- 完成的工作项
- 生成的代码文件
- 需要关注的问题
- 后续建议

## 任务关键词识别

| 关键词 | 识别为 | 触发角色 |
|--------|--------|----------|
| 实现、开发、添加 | 开发任务 | architect → backend/frontend → tester |
| 审查、review、检查 | 代码审查 | architect |
| 测试、test、验证 | 测试任务 | tester |
| 设计、架构、方案 | 架构设计 | architect |
| 修复、fix、bug | Bug修复 | backend/frontend → tester |

## 配置文件路径

- 团队配置：`.claude/team.yaml`
- 设计文档：`docs/superpowers/specs/2026-03-21-agent-teams-design.md`