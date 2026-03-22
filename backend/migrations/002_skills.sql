-- 技能表
CREATE TABLE skills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    readme TEXT,
    author_id UUID REFERENCES users(id),
    version VARCHAR(20) DEFAULT '1.0.0',
    tags TEXT[],
    is_public BOOLEAN DEFAULT true,
    download_count INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 技能版本表（Docker Tag 模式）
CREATE TABLE skill_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    skill_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    version VARCHAR(50) NOT NULL,       -- v1.0.0, v2.1.0-beta.1
    storage_path VARCHAR(500) NOT NULL,
    content TEXT,                       -- 技能内容
    changelog TEXT,
    digest VARCHAR(64),                 -- 内容哈希
    created_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID REFERENCES users(id),
    UNIQUE(skill_id, version)
);

-- 技能标签表（类似 Docker Tag）
CREATE TABLE skill_tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    skill_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    tag VARCHAR(50) NOT NULL,           -- latest, stable, v1, v1.0.0
    version_id UUID REFERENCES skill_versions(id) ON DELETE CASCADE,
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    updated_by UUID REFERENCES users(id),
    UNIQUE(skill_id, tag)
);

-- 索引
CREATE INDEX idx_skills_slug ON skills(slug);
CREATE INDEX idx_skills_name ON skills(name);
CREATE INDEX idx_skills_author ON skills(author_id);
CREATE INDEX idx_skills_tags ON skills USING GIN(tags);
CREATE INDEX idx_skills_created_at ON skills(created_at DESC);
CREATE INDEX idx_skill_versions_skill_id ON skill_versions(skill_id);
CREATE INDEX idx_skill_tags_skill ON skill_tags(skill_id);
CREATE INDEX idx_skill_tags_version ON skill_tags(version_id);

-- updated_at 触发器
CREATE TRIGGER skills_updated_at
    BEFORE UPDATE ON skills
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

-- 自动创建 latest 标签的函数
CREATE OR REPLACE FUNCTION create_latest_tag()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO skill_tags (skill_id, tag, version_id, updated_by)
    VALUES (NEW.skill_id, 'latest', NEW.id, NEW.created_by);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 当创建第一个版本时自动创建 latest 标签
-- 注意：这个触发器在应用层处理更灵活，这里仅作参考