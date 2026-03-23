# 后端日志系统说明

## 概述

Skills Hub 后端使用 `tracing` + `tracing-subscriber` 作为日志系统，支持结构化日志和灵活的配置。

## 技术选型

| 组件 | 说明 |
|------|------|
| `tracing` | Rust 生态最流行的日志/追踪库 |
| `tracing-subscriber` | 日志订阅器和格式化 |
| `EnvFilter` | 通过环境变量控制日志级别 |
| `fmt::Layer` | 支持多种输出格式 |

## 环境变量配置

### RUST_LOG

控制日志级别过滤器，支持精确到模块级别的控制。

```bash
# 全局日志级别
RUST_LOG=info

# 调试级别
RUST_LOG=debug

# 特定模块控制
RUST_LOG=skillhub_backend=trace,info

# 多个模块
RUST_LOG=skillhub_backend::api=debug,sqlx=warn,info

# 生产环境推荐
RUST_LOG=warn,skillhub_backend=info
```

### LOG_FORMAT

控制日志输出格式：

| 值 | 说明 | 使用场景 |
|----|------|----------|
| `pretty` | 人类可读的彩色格式 | 开发环境 |
| `json` | JSON 结构化输出 | 生产环境、日志收集 |
| `compact` | 紧凑的单行格式 | 高性能场景 |

```bash
# 开发环境
LOG_FORMAT=pretty

# 生产环境
LOG_FORMAT=json
```

### LOG_FILE

可选，将日志写入文件：

```bash
LOG_FILE=/var/log/skillhub/app.log
```

## 初始化代码

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, fmt};

fn init_logging() {
    let format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "pretty".into());
    let filter = EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into())
    );

    let registry = tracing_subscriber::registry().with(filter);

    match format.as_str() {
        "json" => {
            registry.with(fmt::layer().json()).init();
        }
        "compact" => {
            registry.with(fmt::layer().compact()).init();
        }
        _ => {
            registry.with(fmt::layer().pretty()).init();
        }
    }
}
```

## 日志级别使用规范

### error!

系统错误、异常情况，需要立即关注和处理。

```rust
// 数据库错误
error!(error = ?e, "Database connection failed");

// 外部服务错误
error!(service = "rustfs", error = ?e, "Object storage unavailable");

// 认证失败（可疑行为）
error!(user_id = %user_id, ip = %client_ip, "Authentication failed for invalid token");
```

### warn!

潜在问题、可恢复的异常，需要关注但不紧急。

```rust
// 配置缺失使用默认值
warn!("JWT_SECRET not set, using default (not recommended for production)");

// 重试操作
warn!(attempt = %retry, max_retries = %max, "Operation failed, retrying");

// 弃用警告
warn!(user_id = %user_id, "Using deprecated API endpoint");
```

### info!

重要业务事件，用于追踪系统运行状态。

```rust
// 服务生命周期
info!(host = %host, port = %port, "Server starting");
info!(duration_ms = %elapsed, "Server ready");

// 用户操作
info!(user_id = %user.id, username = %user.username, "User registered");
info!(user_id = %user_id, "User logged in");

// 资源操作
info!(skill_id = %skill.id, name = %skill.name, "Skill created");
info!(skill_id = %skill_id, version = %version, "Skill version published");

// 连接状态
info!("Connecting to database...");
info!(max_connections = %max, "Database connected");
info!("Connecting to object storage...");
info!(bucket = %bucket, "Object storage connected");
```

### debug!

调试信息、详细流程，开发阶段使用。

```rust
// 请求处理
debug!(method = %method, path = %path, "Processing request");
debug!(user_id = %user_id, "Request authenticated");

// 业务逻辑
debug!(skill_id = %skill_id, "Checking skill ownership");
debug!(query = %sql, "Executing database query");

// 条件分支
debug!(is_public = %is_public, "Determining skill visibility");
```

### trace!

最详细的追踪信息，性能分析或深度调试使用。

```rust
// 函数边界
trace!("entering process_skill_creation");
trace!("exiting process_skill_creation");

// 变量值
trace!(skill = ?skill, "Skill struct contents");

// 循环迭代
trace!(iteration = %i, total = %total, "Processing batch item");
```

## 结构化日志最佳实践

### 使用字段而非字符串拼接

```rust
// ✅ 推荐：结构化字段
info!(user_id = %user.id, skill_id = %skill.id, "Skill created");

// ❌ 不推荐：字符串拼接
info!("Skill {} created by user {}", skill.id, user.id);
```

### 字段命名规范

| 字段类型 | 命名示例 | 说明 |
|----------|----------|------|
| ID | `user_id`, `skill_id` | 使用 `_id` 后缀 |
| 名称 | `username`, `skill_name` | 语义化名称 |
| 操作 | `action`, `operation` | 动作描述 |
| 错误 | `error` | 使用 `?` 格式化 |
| 耗时 | `duration_ms`, `elapsed_ms` | 带单位 |
| 数量 | `count`, `total` | 数字统计 |

### 错误日志格式

```rust
// 使用 ? 格式化错误
error!(error = ?e, user_id = %user_id, "Failed to create skill");

// 包含更多上下文
error!(
    error = ?e,
    user_id = %user_id,
    skill_name = %name,
    "Skill creation failed"
);
```

## Docker 环境配置

### docker-compose.yml

```yaml
services:
  backend:
    build: ./backend
    environment:
      - RUST_LOG=${RUST_LOG:-info}
      - LOG_FORMAT=${LOG_FORMAT:-pretty}
      # - LOG_FILE=/var/log/skillhub/app.log
    volumes:
      # 可选：日志文件持久化
      - ./logs:/var/log/skillhub
```

### .env

```env
# 日志配置
RUST_LOG=info,skillhub_backend=debug
LOG_FORMAT=pretty
```

### .env.production

```env
# 生产环境日志配置
RUST_LOG=warn,skillhub_backend=info
LOG_FORMAT=json
LOG_FILE=/var/log/skillhub/app.log
```

## 日志收集

### JSON 格式输出

生产环境使用 JSON 格式，便于 ELK、Grafana Loki 等日志系统收集：

```bash
LOG_FORMAT=json RUST_LOG=info
```

输出示例：

```json
{"timestamp":"2026-03-23T08:00:00.000000Z","level":"INFO","message":"User registered","user_id":"abc-123","username":"testuser"}
```

### 字段索引

建议为以下字段建立索引：

- `level` - 日志级别
- `timestamp` - 时间戳
- `user_id` - 用户ID
- `skill_id` - 技能ID
- `error` - 错误信息

## 性能考虑

1. **日志级别**：生产环境使用 `info` 或 `warn`
2. **异步写入**：`tracing` 默认异步，不影响性能
3. **字段过滤**：避免在热路径记录过多字段
4. **条件编译**：可在 release 中移除 `trace` 和 `debug` 日志

```rust
// 仅在 debug 模式编译
#[cfg(debug_assertions)]
trace!(data = ?large_data, "Debug trace");
```

## 常见问题

### Q: 日志没有输出？

检查 `RUST_LOG` 环境变量是否设置正确。

### Q: 日志格式混乱？

检查 `LOG_FORMAT` 是否为 `pretty`（开发环境）。

### Q: 日志太多性能下降？

调高日志级别：`RUST_LOG=warn`

### Q: 如何只查看特定模块日志？

```bash
RUST_LOG=skillhub_backend::api::skills=debug,info
```

## 参考资料

- [tracing 官方文档](https://docs.rs/tracing/)
- [tracing-subscriber 文档](https://docs.rs/tracing-subscriber/)
- [EnvFilter 配置语法](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html)