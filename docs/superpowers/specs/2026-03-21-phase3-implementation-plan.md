# Skills Intelligence Hub - Phase 3 实现计划

> 创建时间：2026-03-21
> 前置条件：Phase 2 智能化完成
> 预计周期：持续迭代

---

## 一、Phase 3 目标概览

```
Phase 2 智能化  ──────▶  Phase 3 生态完善
      │                         │
      │                         ├── IDE 插件生态
      │                         ├── 评分与反馈系统
      │                         ├── 分析统计平台
      │                         └── API/SDK 开放
      │
      └── 智能推荐、多格式可用
```

**核心价值：** 构建开放的技能生态系统，连接更多工具和用户

---

## 二、功能模块详细设计

### 模块 1: IDE 插件生态 (Week 1-4)

#### 1.1 VS Code 插件

**目标：** 在 VS Code 中直接浏览、搜索、安装技能

**插件架构：**

```
vscode-extension/
├── src/
│   ├── extension.ts          # 插件入口
│   ├── views/
│   │   ├── SkillExplorer.ts  # 技能浏览器视图
│   │   ├── SkillDetail.ts    # 技能详情面板
│   │   └── Recommendations.ts # 推荐视图
│   ├── commands/
│   │   ├── install.ts        # 安装命令
│   │   ├── search.ts         # 搜索命令
│   │   └── update.ts         # 更新命令
│   ├── providers/
│   │   └── SkillProvider.ts  # 数据提供者
│   └── utils/
│       └── api.ts            # API 客户端
├── package.json              # 插件配置
├── tsconfig.json
└── README.md
```

**核心功能：**

| 功能 | 说明 |
|------|------|
| 技能浏览器 | 侧边栏展示技能列表、分类 |
| 搜索面板 | 快速搜索技能 |
| 一键安装 | 点击即可安装技能 |
| 推荐提示 | 根据当前项目推荐技能 |
| 状态显示 | 已安装技能状态 |

**package.json 配置：**

```json
{
  "name": "skillhub-vscode",
  "displayName": "Skills Intelligence Hub",
  "description": "Browse, search and install AI skills",
  "version": "1.0.0",
  "engines": { "vscode": "^1.85.0" },
  "categories": ["Other"],
  "activationEvents": ["onStartupFinished"],
  "main": "./out/extension.js",
  "contributes": {
    "viewsContainers": {
      "activitybar": [{
        "id": "skillhub",
        "title": "Skills Hub",
        "icon": "resources/icon.svg"
      }]
    },
    "views": {
      "skillhub": [
        { "id": "skillhub.explorer", "name": "Skills" },
        { "id": "skillhub.recommendations", "name": "Recommended" }
      ]
    },
    "commands": [
      {
        "command": "skillhub.search",
        "title": "Skills Hub: Search Skills"
      },
      {
        "command": "skillhub.install",
        "title": "Skills Hub: Install Skill"
      },
      {
        "command": "skillhub.update",
        "title": "Skills Hub: Update Skill"
      }
    ],
    "configuration": {
      "title": "Skills Hub",
      "properties": {
        "skillhub.apiUrl": {
          "type": "string",
          "default": "https://api.skillhub.io",
          "description": "Skills Hub API URL"
        },
        "skillhub.autoRecommend": {
          "type": "boolean",
          "default": true,
          "description": "Show skill recommendations"
        }
      }
    }
  }
}
```

**验收标准：**
- [ ] 插件安装成功
- [ ] 技能列表正确显示
- [ ] 搜索功能正常
- [ ] 一键安装功能正常
- [ ] 推荐功能正常

---

#### 1.2 Cursor 插件

**目标：** 兼容 Cursor IDE（基于 VS Code API）

**特殊功能：**

| 功能 | 说明 |
|------|------|
| Cursor Rules 集成 | 自动生成 `.cursorrules` |
| 实时推荐 | 根据 Chat 上下文推荐技能 |
| 快捷命令 | 斜杠命令调用技能 |

**Cursor 特有集成：**

```typescript
// Cursor Rules 生成
async function generateCursorRules(skills: Skill[]): Promise<string> {
  let rules = "# Project Rules\n\n";

  for (const skill of skills) {
    rules += `## ${skill.name}\n`;
    rules += `${skill.description}\n\n`;

    if (skill.content.principles) {
      rules += "### Guidelines\n";
      for (const p of skill.content.principles) {
        rules += `- ${p}\n`;
      }
      rules += "\n";
    }
  }

  return rules;
}

// 写入 .cursorrules
async function writeCursorRules(rules: string): Promise<void> {
  const workspaceFolders = vscode.workspace.workspaceFolders;
  if (!workspaceFolders) return;

  const cursorRulesPath = path.join(workspaceFolders[0].uri.fsPath, '.cursorrules');
  await vscode.workspace.fs.writeFile(
    vscode.Uri.file(cursorRulesPath),
    Buffer.from(rules)
  );
}
```

---

### 模块 2: 评分与反馈系统 (Week 2-3)

#### 2.1 技能评分模型

**目标：** 建立多维度的技能评分体系

**评分维度：**

| 维度 | 权重 | 数据来源 |
|------|------|----------|
| 功能性 | 30% | 用户评分 |
| 文档质量 | 20% | 用户评分 + 自动检测 |
| 稳定性 | 20% | 错误率、更新频率 |
| 社区活跃度 | 15% | 贡献者、讨论数 |
| 使用热度 | 15% | 下载量、使用频率 |

**综合评分公式：**

```
Score = Σ(Dimension_i × Weight_i) × Trend_Bonus

Trend_Bonus:
  - 周环比增长 > 20%: ×1.1
  - 周环比下降 > 20%: ×0.9
  - 其他: ×1.0
```

**数据库 Schema：**

```sql
-- 技能评分
CREATE TABLE skill_ratings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    skill_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    functionality_score INT CHECK (functionality_score BETWEEN 1 AND 5),
    documentation_score INT CHECK (documentation_score BETWEEN 1 AND 5),
    stability_score INT CHECK (stability_score BETWEEN 1 AND 5),
    comment TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(skill_id, user_id)
);

-- 评分聚合
CREATE TABLE skill_rating_stats (
    skill_id UUID PRIMARY KEY REFERENCES skills(id) ON DELETE CASCADE,
    avg_functionality DECIMAL(3,2),
    avg_documentation DECIMAL(3,2),
    avg_stability DECIMAL(3,2),
    total_score DECIMAL(3,2),
    rating_count INT,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

#### 2.2 使用评价系统

**目标：** 支持用户提交技能使用体验

**评价类型：**

| 类型 | 说明 | 展示位置 |
|------|------|----------|
| 评分 | 1-5 星多维度评分 | 技能详情页 |
| 评论 | 文字评价 | 技能详情页 |
| 标签 | 快捷标签（好用/有问题等） | 技能卡片 |
| 反馈 | 问题反馈 | 后台管理 |

**评价 API：**

```
POST /api/skills/{slug}/ratings
{
  "functionality_score": 5,
  "documentation_score": 4,
  "stability_score": 5,
  "comment": "非常好用的技能，文档清晰",
  "tags": ["推荐", "好用"]
}

GET /api/skills/{slug}/ratings
{
  "stats": {
    "avg_functionality": 4.5,
    "avg_documentation": 4.2,
    "avg_stability": 4.8,
    "total_score": 4.5,
    "rating_count": 128
  },
  "distribution": {
    "5": 80,
    "4": 30,
    "3": 12,
    "2": 4,
    "1": 2
  },
  "recent_reviews": [...]
}
```

---

#### 2.3 热度排行

**目标：** 展示技能热度趋势

**排行榜类型：**

| 排行榜 | 时间范围 | 更新频率 |
|--------|----------|----------|
| 日榜 | 当天 | 每小时 |
| 周榜 | 近7天 | 每天 |
| 月榜 | 近30天 | 每天 |
| 总榜 | 全时间 | 每天 |

**热度算法：**

```
HotScore = (downloads × 1.0) +
           (ratings × 5.0) +
           (favorites × 3.0) +
           (comments × 2.0) +
           (time_decay)

time_decay = e^(-λ × days_since_created)
λ = 0.1 (衰减系数)
```

**排行榜 API：**

```
GET /api/rankings?type=weekly&limit=10

Response:
{
  "rankings": [
    {
      "rank": 1,
      "skill": { "slug": "writing-plans", "name": "Writing Plans" },
      "score": 1523.5,
      "change": +3
    },
    ...
  ],
  "updated_at": "2026-03-21T10:00:00Z"
}
```

---

### 模块 3: 分析统计平台 (Week 3-5)

#### 3.1 使用日志系统

**目标：** 记录技能使用行为，支持分析

**日志类型：**

| 事件类型 | 说明 | 采集点 |
|----------|------|--------|
| skill_view | 技能详情页浏览 | Web 前端 |
| skill_download | 技能下载 | CLI、Web |
| skill_install | 技能安装完成 | CLI |
| skill_use | 技能被调用 | CLI (可选) |
| skill_update | 技能更新 | CLI |
| skill_error | 技能使用错误 | CLI |

**日志 Schema：**

```sql
CREATE TABLE usage_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(50) NOT NULL,
    skill_id UUID REFERENCES skills(id),
    user_id UUID REFERENCES users(id),
    client_type VARCHAR(20),  -- cli, web, vscode
    client_version VARCHAR(20),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 分区表（按月）
CREATE INDEX idx_usage_logs_created_at ON usage_logs(created_at);
CREATE INDEX idx_usage_logs_skill ON usage_logs(skill_id);
CREATE INDEX idx_usage_logs_user ON usage_logs(user_id);
```

**日志采集架构：**

```
┌─────────┐     ┌─────────┐     ┌─────────┐
│   CLI   │────▶│   API   │────▶│  Kafka  │
└─────────┘     └─────────┘     └────┬────┘
                                     │
┌─────────┐     ┌─────────┐         │
│   Web   │────▶│   API   │─────────┘
└─────────┘     └─────────┘         │
                                     ▼
                              ┌─────────────┐
                              │   ClickHouse │
                              │   (分析存储)  │
                              └─────────────┘
```

---

#### 3.2 数据看板

**目标：** 可视化展示技能使用数据

**看板模块：**

| 模块 | 内容 | 刷新频率 |
|------|------|----------|
| 总览 | 总技能数、总用户、总下载量 | 实时 |
| 趋势图 | 下载趋势、使用趋势 | 分钟级 |
| 排行榜 | 热门技能、活跃用户 | 小时级 |
| 分布图 | 技能分类分布、用户分布 | 天级 |

**看板设计：**

```
┌────────────────────────────────────────────────────────────┐
│  Skills Hub Dashboard                         2026-03-21  │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │ 总技能数 │  │ 总用户数 │  │ 总下载量 │  │ 今日活跃 │  │
│  │   1,234  │  │   5,678  │  │  89,012  │  │   234    │  │
│  │  +12% ↑  │  │  +8% ↑   │  │  +15% ↑  │  │  +5% ↑   │  │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │              下载趋势 (近7天)                          │ │
│  │  │                    ╱╲                              │ │
│  │  │          ╱╲       ╱  ╲    ╱╲                       │ │
│  │  │     ╱╲  ╱  ╲  ╱╲ ╱    ╲  ╱  ╲                      │ │
│  │  └───────────────────────────────────────────────────│ │
│  │      Mon  Tue  Wed  Thu  Fri  Sat  Sun               │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ┌─────────────────────┐  ┌─────────────────────────────┐ │
│  │   热门技能 Top 5     │  │     技能分类分布            │ │
│  │  1. writing-plans   │  │  ┌─────────────────────┐    │ │
│  │  2. test-driven-dev │  │  │ ████████ 编程 40%  │    │ │
│  │  3. debugging       │  │  │ ██████ 测试 30%    │    │ │
│  │  4. code-review     │  │  │ ████ 部署 20%      │    │ │
│  │  5. git-workflow    │  │  │ ██ 其他 10%        │    │ │
│  └─────────────────────┘  └─────────────────────────────┘ │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

**看板 API：**

```
GET /api/admin/dashboard

Response:
{
  "summary": {
    "total_skills": 1234,
    "total_users": 5678,
    "total_downloads": 89012,
    "daily_active": 234
  },
  "trends": {
    "downloads": [
      { "date": "2026-03-15", "count": 1234 },
      ...
    ]
  },
  "rankings": {
    "skills": [...],
    "users": [...]
  },
  "distribution": {
    "categories": [...],
    "languages": [...]
  }
}
```

---

#### 3.3 行为分析

**目标：** 分析用户行为，优化产品设计

**分析维度：**

| 分析类型 | 说明 | 应用场景 |
|----------|------|----------|
| 漏斗分析 | 从浏览到安装的转化率 | 优化安装流程 |
| 留存分析 | 用户留存率 | 产品健康度 |
| 路径分析 | 用户行为路径 | 发现问题 |
| 归因分析 | 影响因素分析 | 优化推荐 |

**分析报告示例：**

```
用户转化漏斗 (2026-03-01 ~ 2026-03-21)

浏览列表    10,000  (100%)
    │
    ▼
查看详情     5,000  (50%)
    │
    ▼
点击下载     2,000  (20%)
    │
    ▼
安装成功     1,600  (16%)

转化率分析:
- 列表→详情: 50% (正常)
- 详情→下载: 40% (偏低，建议优化详情页)
- 下载→安装: 80% (良好)
```

---

### 模块 4: API/SDK 开放 (Week 5-6)

#### 4.1 RESTful API 文档

**目标：** 提供完整的 API 文档，支持第三方集成

**文档规范：** OpenAPI 3.0

**API 分类：**

| 分类 | 接口数 | 说明 |
|------|--------|------|
| 认证 | 4 | 登录、注册、刷新、登出 |
| 用户 | 6 | 用户 CRUD、个人设置 |
| 技能 | 10 | 技能 CRUD、搜索、版本 |
| 评分 | 4 | 评分 CRUD |
| 管理 | 8 | 后台管理接口 |

**文档生成：**

```yaml
# openapi.yaml (示例)
openapi: 3.0.0
info:
  title: Skills Intelligence Hub API
  version: 1.0.0

paths:
  /api/skills:
    get:
      summary: List skills
      parameters:
        - name: q
          in: query
          schema:
            type: string
        - name: tags
          in: query
          schema:
            type: string
        - name: page
          in: query
          schema:
            type: integer
            default: 1
      responses:
        '200':
          description: Success
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/Skill'
```

**文档站点：** 使用 Swagger UI 或 Redoc

---

#### 4.2 Python SDK

**目标：** 提供 Python 客户端 SDK

**SDK 架构：**

```
skillhub-python/
├── skillhub/
│   ├── __init__.py
│   ├── client.py         # 客户端主类
│   ├── models/
│   │   ├── __init__.py
│   │   ├── skill.py
│   │   └── user.py
│   ├── api/
│   │   ├── __init__.py
│   │   ├── skills.py
│   │   └── auth.py
│   └── errors.py
├── tests/
├── setup.py
├── pyproject.toml
└── README.md
```

**使用示例：**

```python
from skillhub import SkillHubClient

# 初始化客户端
client = SkillHubClient(api_url="https://api.skillhub.io")

# 认证
client.login("user@example.com", "password")

# 列出技能
skills = client.skills.list(q="testing", tags=["python"])
for skill in skills:
    print(f"{skill.name}: {skill.description}")

# 搜索技能
results = client.skills.search_semantic("帮我写单元测试")

# 下载技能
skill = client.skills.get("test-driven-development")
skill.download("/path/to/project/.claude/skills")

# 上下文检测 + 推荐
context = client.context.detect("/path/to/project")
recommendations = client.recommend(context)
```

**发布：** PyPI (`pip install skillhub`)

---

#### 4.3 Node.js SDK

**目标：** 提供 Node.js 客户端 SDK

**SDK 架构：**

```
skillhub-node/
├── src/
│   ├── index.ts
│   ├── client.ts
│   ├── models/
│   │   ├── skill.ts
│   │   └── user.ts
│   ├── api/
│   │   ├── skills.ts
│   │   └── auth.ts
│   └── errors.ts
├── tests/
├── package.json
├── tsconfig.json
└── README.md
```

**使用示例：**

```typescript
import { SkillHubClient } from '@skillhub/sdk';

const client = new SkillHubClient({
  apiUrl: 'https://api.skillhub.io'
});

// 认证
await client.login('user@example.com', 'password');

// 列出技能
const skills = await client.skills.list({ q: 'testing' });

// 搜索技能
const results = await client.skills.searchSemantic('帮我写单元测试');

// 下载技能
const skill = await client.skills.get('test-driven-development');
await skill.download('/path/to/project/.claude/skills');

// 上下文检测 + 推荐
const context = await client.context.detect('/path/to/project');
const recommendations = await client.recommend(context);
```

**发布：** npm (`npm install @skillhub/sdk`)

---

#### 4.4 Webhook 支持

**目标：** 支持事件推送，方便第三方集成

**支持的 Webhook 事件：**

| 事件 | 触发时机 | Payload |
|------|----------|---------|
| `skill.created` | 技能创建 | 技能信息 |
| `skill.updated` | 技能更新 | 技能信息、变更内容 |
| `skill.deleted` | 技能删除 | 技能 ID |
| `rating.created` | 收到评价 | 评价信息 |
| `user.registered` | 用户注册 | 用户信息 |

**Webhook 配置：**

```
POST /api/webhooks
{
  "url": "https://your-server.com/webhook",
  "events": ["skill.created", "skill.updated"],
  "secret": "your-webhook-secret"
}
```

**Webhook Payload：**

```json
{
  "id": "evt_abc123",
  "event": "skill.updated",
  "data": {
    "skill": {
      "id": "...",
      "slug": "writing-plans",
      "version": "1.1.0"
    },
    "changes": ["version", "readme"]
  },
  "timestamp": "2026-03-21T10:00:00Z",
  "signature": "sha256=..."
}
```

**签名验证：**

```python
import hmac
import hashlib

def verify_webhook(payload: bytes, signature: str, secret: str) -> bool:
    expected = hmac.new(
        secret.encode(),
        payload,
        hashlib.sha256
    ).hexdigest()
    return hmac.compare_digest(f"sha256={expected}", signature)
```

---

## 三、基础设施需求

### 新增服务

| 服务 | 用途 | 配置建议 |
|------|------|----------|
| ClickHouse | 日志分析 | 4C8G |
| Kafka | 消息队列 | 3节点集群 |
| Redis | 缓存/排行榜 | 2C4G |

### 数据库扩展

```sql
-- 新增索引
CREATE INDEX idx_skill_ratings_skill ON skill_ratings(skill_id);
CREATE INDEX idx_usage_logs_event_type ON usage_logs(event_type);

-- 物化视图（排行榜）
CREATE MATERIALIZED VIEW skill_rankings AS
SELECT
    s.id,
    s.slug,
    s.name,
    COUNT(DISTINCT ul.user_id) as unique_users,
    COUNT(ul.id) as total_events,
    AVG(sr.total_score) as avg_rating
FROM skills s
LEFT JOIN usage_logs ul ON s.id = ul.skill_id
LEFT JOIN skill_rating_stats sr ON s.id = sr.skill_id
GROUP BY s.id, s.slug, s.name
ORDER BY unique_users DESC;
```

---

## 四、API 新增汇总

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | /api/rankings | 排行榜 |
| POST | /api/skills/{slug}/ratings | 提交评分 |
| GET | /api/skills/{slug}/ratings | 获取评分统计 |
| GET | /api/admin/dashboard | 数据看板 |
| GET | /api/admin/analytics | 行为分析 |
| POST | /api/webhooks | 配置 Webhook |
| GET | /api/docs | API 文档 |

---

## 五、SDK 发布计划

| SDK | 版本 | 发布时间 | 平台 |
|-----|------|----------|------|
| Python SDK | v1.0.0 | Week 5 | PyPI |
| Node.js SDK | v1.0.0 | Week 6 | npm |

---

## 六、检查点

| 检查点 | 时间 | 验收项 |
|--------|------|--------|
| CP1 | Week 2 | VS Code 插件可用 |
| CP2 | Week 3 | 评分系统完成 |
| CP3 | Week 4 | 看板功能完成 |
| CP4 | Week 5 | Python SDK 发布 |
| CP5 | Week 6 | Node.js SDK 发布 |

---

## 七、持续迭代方向

Phase 3 完成后，后续可考虑：

1. **更多 IDE 插件** - JetBrains 系列、Zed
2. **企业版功能** - 私有部署、SSO、审计日志
3. **AI 增强** - 自动技能生成、智能问答
4. **社区功能** - 技能讨论区、贡献者体系
5. **移动端** - iOS/Android App

---

*文档由 Claude Code 生成*