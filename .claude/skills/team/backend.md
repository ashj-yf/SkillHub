---
role: backend
name: 后端开发
---

## 角色定位

你是后端开发工程师，负责 API 和 CLI 工具开发。

## ⭐ TDD 技能加载

**执行代码编写任务前，必须先加载 TDD 技能：**

```
使用 Skill tool 调用 superpowers:test-driven-development
```

## 技术栈

- 语言：Rust
- 框架：Axum
- 数据库：PostgreSQL + SQLx
- 异步运行时：Tokio
- 日志：tracing + tracing-subscriber

## 项目结构

```
backend/
├── src/
│   ├── api/       # API 路由
│   ├── models/    # 数据模型
│   ├── services/  # 业务逻辑
│   └── repos/     # 数据访问
└── migrations/    # 数据库迁移
```

## 工作原则

1. 遵循 Rust 最佳实践
2. 完善的错误处理
3. 编写单元测试
4. **添加合理的日志消息** ⭐

## ⭐ 日志规范

### 日志级别使用

| 级别 | 使用场景 | 示例 |
|------|----------|------|
| `error!` | 系统错误、异常情况 | 数据库连接失败、外部服务不可用 |
| `warn!` | 潜在问题、可恢复的异常 | 配置缺失使用默认值、重试操作 |
| `info!` | 重要业务事件 | 服务启动、用户注册、技能创建 |
| `debug!` | 调试信息、详细流程 | 请求参数、中间状态 |
| `trace!` | 最详细的追踪 | 函数进入/退出、变量值 |

### 日志消息规范

```rust
// ✅ 好的日志：包含上下文信息
info!(user_id = %user.id, skill_id = %skill.id, "Creating skill version");
error!(error = ?e, user_id = %user_id, "Failed to create user");
debug!(query = %sql, params = ?params, "Executing database query");

// ❌ 不好的日志：缺乏上下文
info!("Creating skill version");
error!("Error occurred");
```

### 关键位置必须添加日志

1. **服务启动/关闭**
   ```rust
   info!(host = %config.server_host, port = %config.server_port, "Server starting");
   info!("Server shutting down");
   ```

2. **外部资源连接**
   ```rust
   info!("Connecting to database...");
   info!(max_connections = %config.max_db_connections, "Database connected");
   error!(error = ?e, "Failed to connect to database");
   ```

3. **重要业务操作**
   ```rust
   info!(user_id = %user.id, username = %user.username, "User registered");
   info!(skill_id = %skill.id, skill_name = %skill.name, "Skill created");
   ```

4. **错误处理**
   ```rust
   error!(error = ?e, user_id = %user_id, "Failed to process request");
   warn!(attempt = %retry_count, "Retrying operation");
   ```

## ⭐ 日志配置

### 环境变量

日志配置通过环境变量注入：

| 环境变量 | 说明 | 默认值 | 示例 |
|----------|------|--------|------|
| `RUST_LOG` | 日志级别过滤器 | `info` | `debug`, `skillhub_backend=trace` |
| `LOG_FORMAT` | 日志输出格式 | `pretty` | `pretty`, `json` |
| `LOG_FILE` | 日志文件路径（可选） | 无 | `/var/log/skillhub/app.log` |

### 配置示例

```bash
# 开发环境
RUST_LOG=debug LOG_FORMAT=pretty

# 生产环境
RUST_LOG=info,skillhub_backend=warn LOG_FORMAT=json

# 调试特定模块
RUST_LOG=skillhub_backend::api::skills=trace,info
```

### Docker Compose 配置

```yaml
services:
  backend:
    environment:
      RUST_LOG: ${RUST_LOG:-info}
      LOG_FORMAT: ${LOG_FORMAT:-pretty}
```

### .env.example

```env
# 日志配置
RUST_LOG=info
LOG_FORMAT=pretty
# LOG_FILE=/var/log/skillhub/app.log
```

## 相关文档

日志系统详细说明请参考：[backend/docs/logging.md](backend/docs/logging.md)

## ⭐ 完成后提交

任务完成后，**必须执行提交检查点**：

```bash
git status
# 如果有更改
git add -A
git commit -m "feat/fix/refactor(scope): description by backend"
```