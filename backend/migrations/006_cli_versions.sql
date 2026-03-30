-- CLI 版本管理迁移
-- 创建 CLI 版本信息表和下载链接表

-- CLI 版本信息表
CREATE TABLE IF NOT EXISTS cli_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    version VARCHAR(32) NOT NULL UNIQUE,
    changelog TEXT,
    release_date DATE,
    min_version VARCHAR(32),
    force_update BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 下载链接表
CREATE TABLE IF NOT EXISTS cli_downloads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    version_id UUID REFERENCES cli_versions(id) ON DELETE CASCADE,
    platform VARCHAR(32) NOT NULL,
    filename VARCHAR(128) NOT NULL,
    url VARCHAR(512) NOT NULL,
    size BIGINT,
    checksum VARCHAR(128),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(version_id, platform)
);

-- 触发器：自动更新 updated_at
CREATE OR REPLACE FUNCTION update_cli_versions_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_cli_versions_updated_at
    BEFORE UPDATE ON cli_versions
    FOR EACH ROW
    EXECUTE FUNCTION update_cli_versions_updated_at();

-- 初始数据：v0.1.0 版本
INSERT INTO cli_versions (version, changelog, release_date, min_version) VALUES
('0.1.0',
 '### 新功能
- 初始化配置 (init)
- 列出技能 (list)
- 下载技能 (pull)
- 搜索技能 (search)
- 显示详情 (show)
- 标签管理 (tag)
- 用户登录 (login)
- 用户登出 (logout)
- 版本信息 (version)
- 本地技能 (local)
- 更新技能 (update)
- 删除技能 (remove)
- 版本历史 (versions)',
 '2026-03-30',
 '0.1.0');

INSERT INTO cli_downloads (version_id, platform, filename, url) VALUES
((SELECT id FROM cli_versions WHERE version = '0.1.0'), 'linux-x86_64', 'skillhub-linux-x86_64', '/downloads/cli/skillhub-linux-x86_64'),
((SELECT id FROM cli_versions WHERE version = '0.1.0'), 'linux-arm64', 'skillhub-linux-arm64', '/downloads/cli/skillhub-linux-arm64'),
((SELECT id FROM cli_versions WHERE version = '0.1.0'), 'macos-x86_64', 'skillhub-macos-x86_64', '/downloads/cli/skillhub-macos-x86_64'),
((SELECT id FROM cli_versions WHERE version = '0.1.0'), 'macos-arm64', 'skillhub-macos-arm64', '/downloads/cli/skillhub-macos-arm64'),
((SELECT id FROM cli_versions WHERE version = '0.1.0'), 'windows-x86_64', 'skillhub-windows-x86_64.exe', '/downloads/cli/skillhub-windows-x86_64.exe');

-- 索引
CREATE INDEX idx_cli_versions_version ON cli_versions(version);
CREATE INDEX idx_cli_downloads_version ON cli_downloads(version_id);