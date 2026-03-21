# Skills Intelligence Hub - Phase 2 实现计划

> 创建时间：2026-03-21
> 前置条件：Phase 1 MVP 完成
> 预计周期：4-6 周

---

## 一、Phase 2 目标概览

```
Phase 1 MVP  ──────▶  Phase 2 智能化
   │                       │
   │                       ├── 智能推荐引擎
   │                       ├── 多格式支持
   │                       ├── 技能图谱
   │                       └── 增强功能
   │
   └── 基础功能可用
```

**核心价值：** 从"被动查找"升级为"主动推荐"，提升技能发现效率

---

## 二、功能模块详细设计

### 模块 1: 智能推荐引擎 (Week 1-2)

#### 1.1 项目上下文检测

**目标：** 自动识别项目技术栈、框架、语言

**检测维度：**

| 维度 | 检测方式 | 示例 |
|------|----------|------|
| 语言 | 文件扩展名统计 | `.ts` 占比 60% → TypeScript 项目 |
| 框架 | 配置文件特征 | `package.json` 含 `next` → Next.js |
| 数据库 | 连接配置、ORM | `prisma/schema.prisma` → PostgreSQL |
| 构建工具 | 锁文件、配置 | `bun.lockb` → Bun |
| 测试框架 | 测试文件特征 | `*.test.ts` + `vitest.config.ts` |

**实现设计：**

```
cli/src/context/
├── mod.rs              # 上下文检测入口
├── detectors/
│   ├── mod.rs
│   ├── language.rs     # 语言检测器
│   ├── framework.rs    # 框架检测器
│   ├── database.rs     # 数据库检测器
│   └── tooling.rs      # 工具链检测器
├── patterns.rs         # 特征模式库
└── result.rs           # 检测结果结构
```

**检测算法伪代码：**

```rust
// context/mod.rs
pub struct ProjectContext {
    pub languages: Vec<LanguageInfo>,
    pub frameworks: Vec<FrameworkInfo>,
    pub databases: Vec<DatabaseInfo>,
    pub tools: Vec<ToolInfo>,
    pub confidence: f32,
}

impl ProjectContext {
    pub fn detect(project_path: &Path) -> Result<Self> {
        let mut detectors: Vec<Box<dyn Detector>> = vec![
            Box::new(LanguageDetector::new()),
            Box::new(FrameworkDetector::new()),
            Box::new(DatabaseDetector::new()),
            Box::new(ToolingDetector::new()),
        ];

        let mut context = ProjectContext::default();

        for detector in &mut detectors {
            detector.detect(project_path, &mut context)?;
        }

        context.calculate_confidence();
        Ok(context)
    }
}
```

**验收标准：**
- [ ] 正确识别 10+ 主流语言
- [ ] 正确识别 15+ 主流框架
- [ ] 识别准确率 > 85%
- [ ] 检测耗时 < 2s

---

#### 1.2 语义搜索（向量索引）

**目标：** 支持自然语言搜索技能，而非仅关键词匹配

**技术选型：**

| 组件 | 选择 | 理由 |
|------|------|------|
| 向量数据库 | Qdrant | 开源、高性能、支持过滤 |
| Embedding 模型 | text-embedding-3-small | 性价比高、效果好 |
| 备选方案 | pgvector | 无需额外组件 |

**数据流程：**

```
技能内容 ──▶ Embedding API ──▶ 向量 ──▶ Qdrant 存储
                                        │
用户查询 ──▶ Embedding API ──▶ 向量 ──▶ 相似度搜索 ──▶ 结果
```

**向量 Schema 设计：**

```rust
// 向量存储结构
pub struct SkillVector {
    pub id: Uuid,                    // 技能 ID
    pub vector: Vec<f32>,            // 1536 维向量
    pub payload: SkillVectorPayload, // 元数据
}

pub struct SkillVectorPayload {
    pub name: String,
    pub slug: String,
    pub description: String,
    pub tags: Vec<String>,
    pub version: String,
    pub download_count: u32,
}
```

**API 设计：**

```
POST /api/skills/search/semantic
Request:
{
  "query": "帮我处理 JSON 数据解析和验证",
  "limit": 10,
  "filters": {
    "tags": ["rust", "data-processing"],
    "min_downloads": 100
  }
}

Response:
{
  "results": [
    {
      "skill": { ... },
      "score": 0.89,
      "highlights": ["JSON 解析", "数据验证"]
    }
  ]
}
```

**验收标准：**
- [ ] 向量索引创建成功
- [ ] 语义搜索返回相关结果
- [ ] 搜索延迟 < 500ms
- [ ] Top-10 命中率 > 80%

---

#### 1.3 技能推荐算法

**目标：** 基于项目上下文推荐合适的技能

**推荐策略：**

| 策略 | 权重 | 说明 |
|------|------|------|
| 上下文匹配 | 40% | 项目技术栈与技能标签匹配度 |
| 语义相似度 | 30% | 项目描述与技能描述相似度 |
| 热度排序 | 15% | 下载量、评分 |
| 团队使用 | 15% | 团队成员使用过的技能 |

**推荐流程：**

```
┌─────────────────┐
│ 项目上下文检测  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐     ┌─────────────────┐
│  标签匹配过滤   │────▶│  候选技能池     │
└─────────────────┘     └────────┬────────┘
                                 │
         ┌───────────────────────┼───────────────────────┐
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  语义相似度计算 │     │  热度分数计算   │     │  团队使用加分   │
└────────┬────────┘     └────────┬────────┘     └────────┬────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                                 ▼
                        ┌─────────────────┐
                        │  加权排序返回   │
                        └─────────────────┘
```

**CLI 命令设计：**

```bash
# 自动检测并推荐
skillhub sense

# 指定目录
skillhub sense --path /path/to/project

# 详细输出
skillhub sense --verbose

# 输出示例
🔍 检测到项目技术栈:
  • 语言: TypeScript (78%), Python (22%)
  • 框架: Next.js, FastAPI
  • 数据库: PostgreSQL
  • 工具: Docker, pnpm

💡 推荐技能 (Top 5):

  1. superpowers:writing-plans (评分: 0.92)
     用于编写详细的实现计划
     → skillhub pull writing-plans

  2. superpowers:test-driven-development (评分: 0.87)
     测试驱动开发流程
     → skillhub pull test-driven-development

  3. superpowers:systematic-debugging (评分: 0.84)
     系统化调试方法
     → skillhub pull systematic-debugging
```

**验收标准：**
- [ ] `skillhub sense` 命令可用
- [ ] 推荐结果与项目相关
- [ ] 推荐响应时间 < 3s
- [ ] 用户满意度 > 70%

---

### 模块 2: 多格式支持 (Week 2-3)

#### 2.1 格式转换架构

**目标：** 一份技能源文件，自动转换多种目标格式

**支持的格式：**

| 格式 | 用途 | 文件名 |
|------|------|--------|
| Claude Code | Claude CLI 技能 | `claude-skill.md` |
| Cursor Rules | Cursor IDE 规则 | `.cursorrules` |
| Copilot Instructions | GitHub Copilot | `.github/copilot-instructions.md` |
| Windsurf Rules | Windsurf IDE | `.windsurfrules` |

**转换引擎设计：**

```
backend/src/formats/
├── mod.rs              # 转换引擎入口
├── schema.rs           # 技能源格式定义
├── converters/
│   ├── mod.rs
│   ├── claude.rs       # Claude Code 格式
│   ├── cursor.rs       # Cursor 格式
│   ├── copilot.rs      # Copilot 格式
│   └── windsurf.rs     # Windsurf 格式
└── templates/          # 格式模板
    ├── claude.md.hbs
    ├── cursor.md.hbs
    └── copilot.md.hbs
```

**源格式 Schema：**

```yaml
# skill.yaml - 技能源格式
id: writing-plans
name: Writing Plans
description: 用于编写详细的实现计划
version: 1.0.0
author: superpowers

# 触发条件
triggers:
  - pattern: "write.*plan"
  - pattern: "implement.*step"
  - context: "planning"

# 核心内容
content:
  principles:
    - 原则1：明确目标
    - 原则2：分步实施
  workflow:
    - step: 1
      action: 分析需求
    - step: 2
      action: 设计方案
  examples:
    - title: 示例1
      code: |
        // 代码示例

# 格式特定配置
formats:
  claude:
    trigger: TRIGGER when: user asks for planning
  cursor:
    priority: high
  copilot:
    languages: ["typescript", "python"]
```

**转换逻辑示例：**

```rust
// formats/converters/cursor.rs
pub struct CursorConverter;

impl Converter for CursorConverter {
    fn convert(&self, skill: &SkillSource) -> Result<String> {
        let mut output = String::new();

        // Cursor Rules 头部
        output.push_str("# Project Instructions\n\n");

        // 描述
        output.push_str(&format!("{}\n\n", skill.description));

        // 触发条件（Cursor 特有格式）
        if let Some(triggers) = &skill.triggers {
            output.push_str("## When to apply\n");
            for trigger in triggers {
                output.push_str(&format!("- {}\n", trigger.pattern));
            }
            output.push_str("\n");
        }

        // 核心内容
        output.push_str("## Instructions\n\n");
        for principle in &skill.content.principles {
            output.push_str(&format!("- {}\n", principle));
        }

        Ok(output)
    }
}
```

**验收标准：**
- [ ] Claude Code 格式正确生成
- [ ] Cursor Rules 格式正确生成
- [ ] Copilot Instructions 格式正确生成
- [ ] 格式内容保持语义一致

---

#### 2.2 CLI 格式选择

**命令设计：**

```bash
# 默认输出 Claude Code 格式
skillhub pull writing-plans

# 指定输出格式
skillhub pull writing-plans --format cursor
skillhub pull writing-plans --format copilot

# 输出所有格式
skillhub pull writing-plans --format all

# 查看格式预览
skillhub show writing-plans --format cursor
```

**配置文件支持：**

```toml
# ~/.skillhub/config.toml
[default]
format = "claude"

[formats.cursor]
output_dir = ".cursor"
filename = "rules"

[formats.copilot]
output_dir = ".github"
filename = "copilot-instructions.md"
```

---

### 模块 3: 技能图谱 (Week 3-4)

#### 3.1 技能继承机制

**目标：** 支持技能之间的继承关系，实现技能复用

**继承模型：**

```
base-skill (基础技能)
    │
    ├── extended-skill-a (扩展技能A)
    │       │
    │       └── specialized-skill (特化技能)
    │
    └── extended-skill-b (扩展技能B)
```

**Schema 扩展：**

```yaml
# skill.yaml
id: specialized-skill
name: Specialized Skill
extends: extended-skill-a  # 继承父技能

# 覆盖父技能的部分内容
content:
  principles:
    - $inherit: true      # 继承父技能
    - 新增原则           # 追加内容
```

**继承解析算法：**

```rust
pub fn resolve_skill(skill_id: &str) -> Result<ResolvedSkill> {
    let mut resolved = ResolvedSkill::default();
    let mut visited = HashSet::new();

    // 从当前技能开始，向上遍历继承链
    let mut current = load_skill(skill_id)?;
    let inheritance_chain = vec![current.clone()];

    while let Some(parent_id) = &current.extends {
        if visited.contains(parent_id) {
            return Err(Error::CircularInheritance);
        }
        visited.insert(parent_id.clone());

        current = load_skill(parent_id)?;
        inheritance_chain.insert(0, current.clone());
    }

    // 从根到叶合并内容
    for skill in inheritance_chain {
        resolved.merge(skill);
    }

    Ok(resolved)
}
```

**验收标准：**
- [ ] 单层继承正确解析
- [ ] 多层继承正确解析
- [ ] 循环继承检测并报错
- [ ] 继承内容正确合并

---

#### 3.2 技能组合机制

**目标：** 支持多个技能组合成一个技能包

**组合模型：**

```yaml
# skill-pack.yaml
id: fullstack-dev-pack
name: Fullstack Development Pack
description: 全栈开发技能组合包

skills:
  - id: superpowers:writing-plans
    required: true

  - id: superpowers:test-driven-development
    required: true

  - id: superpowers:systematic-debugging
    required: false

  - id: superpowers:verification-before-completion
    required: true
```

**CLI 命令：**

```bash
# 安装技能包
skillhub pull fullstack-dev-pack

# 自动安装包内所有技能
# 输出：
# ✓ 安装 superpowers:writing-plans
# ✓ 安装 superpowers:test-driven-development
# ○ 跳过 superpowers:systematic-debugging (optional)
# ✓ 安装 superpowers:verification-before-completion
```

---

#### 3.3 依赖解析

**目标：** 自动解析技能依赖关系

**依赖图：**

```
skill-a
    ├── skill-b (v1.0+)
    │       └── skill-d (v2.0)
    └── skill-c
            └── skill-d (v1.5+)  # 冲突！
```

**解析算法：**

```rust
pub struct DependencyResolver {
    installed: HashMap<String, Version>,
}

impl DependencyResolver {
    pub fn resolve(&self, skill_id: &str) -> Result<Resolution> {
        let mut resolution = Resolution::default();
        let mut queue = vec![(skill_id.to_string(), None)];

        while let Some((id, required_by)) = queue.pop() {
            let skill = load_skill(&id)?;

            for dep in &skill.dependencies {
                match self.check_version(&dep.id, &dep.version_req) {
                    VersionCheck::Satisfied => continue,
                    VersionCheck::Conflict(current) => {
                        return Err(Error::VersionConflict {
                            skill: dep.id.clone(),
                            required: dep.version_req.clone(),
                            current,
                            required_by: required_by.clone(),
                        });
                    }
                    VersionCheck::NotInstalled => {
                        resolution.to_install.push(dep.clone());
                        queue.push((dep.id.clone(), Some(id.clone())));
                    }
                }
            }
        }

        Ok(resolution)
    }
}
```

**验收标准：**
- [ ] 正确解析简单依赖
- [ ] 检测版本冲突
- [ ] 生成安装顺序
- [ ] 提供冲突解决方案

---

### 模块 4: 增强功能 (Week 4-5)

#### 4.1 自动同步机制

**目标：** 技能更新自动推送到客户端

**同步架构：**

```
┌─────────────────┐     WebSocket      ┌─────────────────┐
│   CLI Client    │◄──────────────────▶│   Backend API   │
└─────────────────┘                    └────────┬────────┘
                                                │
                                                ▼
                                       ┌─────────────────┐
                                       │  Git Webhook    │
                                       │  (技能更新)     │
                                       └─────────────────┘
```

**同步流程：**

1. 技能仓库推送新版本 → Gitea Webhook
2. Backend 接收 Webhook → 解析变更
3. 通知订阅该技能的客户端
4. 客户端询问是否更新

**WebSocket 消息格式：**

```json
{
  "type": "skill_updated",
  "data": {
    "slug": "superpowers:writing-plans",
    "old_version": "1.0.0",
    "new_version": "1.1.0",
    "changelog": "新增 AI 辅助规划功能"
  }
}
```

**CLI 配置：**

```toml
# ~/.skillhub/config.toml
[sync]
auto_check = true
interval = "1h"          # 每小时检查
auto_update = false      # 不自动更新，提示用户
```

---

#### 4.2 增量更新

**目标：** 只下载变更部分，减少网络传输

**增量更新策略：**

| 变更类型 | 更新方式 |
|----------|----------|
| 元数据变更 | 只更新元数据 |
| 内容变更 | Diff + Patch |
| 文件新增 | 只下载新文件 |
| 文件删除 | 删除本地文件 |

**Diff 算法选择：** Git-like delta compression

**API 设计：**

```
GET /api/skills/{slug}/versions/{old_version}..{new_version}/diff

Response:
{
  "operations": [
    {
      "type": "patch",
      "file": "claude-skill.md",
      "diff": "@@ -1,5 +1,6 @@\n..."
    },
    {
      "type": "add",
      "file": "templates/new.md",
      "content": "..."
    }
  ]
}
```

---

#### 4.3 冲突解决

**目标：** 本地修改与远程更新冲突时的处理

**冲突检测：**

```rust
pub struct ConflictDetector;

impl ConflictDetector {
    pub fn detect(
        &self,
        local: &SkillContent,
        remote: &SkillContent,
        base: &SkillContent,
    ) -> Option<Conflict> {
        // 三方合并检测
        let local_changes = diff(base, local);
        let remote_changes = diff(base, remote);

        // 检测重叠修改
        for (file, local_diff) in &local_changes {
            if let Some(remote_diff) = remote_changes.get(file) {
                if self.overlaps(local_diff, remote_diff) {
                    return Some(Conflict {
                        file: file.clone(),
                        local: local_diff.clone(),
                        remote: remote_diff.clone(),
                    });
                }
            }
        }

        None
    }
}
```

**解决方案：**

1. **优先本地** - 保留本地修改，忽略远程
2. **优先远程** - 使用远程版本，丢弃本地
3. **手动合并** - 提示用户编辑
4. **智能合并** - 自动合并非冲突部分

---

## 三、数据库变更

### 新增表

```sql
-- 技能继承关系
CREATE TABLE skill_inheritance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    skill_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(skill_id, parent_id)
);

-- 技能依赖关系
CREATE TABLE skill_dependencies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    skill_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    depends_on_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    version_req VARCHAR(50),
    required BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(skill_id, depends_on_id)
);

-- 技能向量索引
CREATE TABLE skill_embeddings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    skill_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    embedding vector(1536),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(skill_id)
);

-- 用户订阅
CREATE TABLE skill_subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    skill_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    auto_update BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, skill_id)
);
```

---

## 四、API 新增

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | /api/skills/search/semantic | 语义搜索 |
| GET | /api/recommend | 技能推荐 |
| POST | /api/skills/{slug}/formats/{format} | 格式转换 |
| GET | /api/skills/{slug}/dependencies | 依赖树 |
| GET | /api/skills/{slug}/diff/{old}..{new} | 增量更新 |
| WS | /ws | WebSocket 连接 |

---

## 五、CLI 命令新增

| 命令 | 说明 |
|------|------|
| `skillhub sense` | 智能推荐 |
| `skillhub pull --format <format>` | 指定格式下载 |
| `skillhub update [slug]` | 更新技能 |
| `skillhub deps <slug>` | 查看依赖 |
| `skillhub sync` | 同步订阅 |

---

## 六、检查点

| 检查点 | 时间 | 验收项 |
|--------|------|--------|
| CP1 | Week 2 | 智能推荐可用 |
| CP2 | Week 3 | 多格式转换完成 |
| CP3 | Week 4 | 技能图谱功能完成 |
| CP4 | Week 5 | 增强功能完成 |
| CP5 | Week 6 | 全部测试通过 |

---

## 七、依赖条件

- Phase 1 MVP 完成并稳定运行
- Qdrant 向量数据库部署
- Embedding API 可用（OpenAI 或自托管）
- WebSocket 支持配置

---

*文档由 Claude Code 生成*