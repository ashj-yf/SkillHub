# 需求文档与规划目录

此目录通过 git subtree 挂载自 [skills-hub-project](https://github.com/JokerYF/skills-hub-project.git) 仓库。

## 目录用途

本目录包含 skills-hub 项目的需求文档和开发规划，包括：

- 功能需求说明
- 技术架构设计
- 开发路线图
- 迭代计划

## 更新方式

从上游仓库拉取最新更新：

```bash
git subtree pull --prefix=docs/requirements https://github.com/JokerYF/skills-hub-project.git main --squash
```

向上游仓库推送更改：

```bash
git subtree push --prefix=docs/requirements https://github.com/JokerYF/skills-hub-project.git main
```