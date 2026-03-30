use anyhow::{anyhow, Result};
use flate2::read::GzDecoder;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;

/// 解析后的技能内容
#[derive(Debug, Default)]
pub struct ParsedSkill {
    /// skill.yaml 元数据
    pub manifest: Option<SkillManifest>,
    /// 文件内容映射 (路径 -> 内容)
    pub files: HashMap<String, Vec<u8>>,
}

/// skill.yaml 元数据结构
#[derive(Debug, Deserialize, Default)]
pub struct SkillManifest {
    pub id: Option<String>,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub visibility: Option<String>,
}

/// tar.gz 解析器
pub struct TarGzParser;

impl TarGzParser {
    /// 解析 tar.gz 文件内容
    pub fn parse(data: &[u8]) -> Result<ParsedSkill> {
        let decoder = GzDecoder::new(data);
        let mut archive = tar::Archive::new(decoder);

        let mut parsed = ParsedSkill::default();

        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?.to_string_lossy().to_string();

            // 跳过目录
            if entry.header().entry_type().is_dir() {
                continue;
            }

            // 安全检查：路径穿越
            if path.contains("..") || path.starts_with('/') {
                return Err(anyhow!("路径穿越检测: {}", path));
            }

            // 提取文件内容
            let mut content = Vec::new();
            entry.read_to_end(&mut content)?;

            // 检查是否为 skill.yaml
            let file_name = path.rsplit('/').next().unwrap_or(&path);
            if file_name == "skill.yaml" || file_name == "skill.yml" {
                let yaml_str = String::from_utf8_lossy(&content);
                parsed.manifest = Some(serde_yaml::from_str(&yaml_str)?);
            } else {
                parsed.files.insert(path, content);
            }
        }

        Ok(parsed)
    }

    /// 验证 tar.gz 结构是否包含必需文件
    pub fn validate(parsed: &ParsedSkill) -> Result<()> {
        // 检查 skill.yaml 存在
        if parsed.manifest.is_none() {
            return Err(anyhow!("缺少必需文件: skill.yaml"));
        }

        // 检查 skill.md 存在
        let has_skill_md = parsed.files.keys().any(|path| {
            let file_name = path.rsplit('/').next().unwrap_or(path);
            file_name == "skill.md"
        });

        if !has_skill_md {
            return Err(anyhow!("缺少必需文件: skill.md"));
        }

        // 检查元数据必填字段
        if let Some(ref manifest) = parsed.manifest {
            if manifest.name.is_empty() {
                return Err(anyhow!("skill.yaml 缺少必填字段: name"));
            }
            if manifest.version.is_empty() {
                return Err(anyhow!("skill.yaml 缺少必填字段: version"));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tar_gz() -> Vec<u8> {
        use std::io::Cursor;
        use tar::Builder;

        let mut builder = Builder::new(Vec::new());

        // 添加 skill.yaml
        let yaml_content = b"name: test-skill\nversion: \"1.0.0\"\nauthor: test\n";
        let mut header = tar::Header::new_gnu();
        header.set_size(yaml_content.len() as u64);
        header.set_cksum();
        builder.append_data(&mut header, "skill-test/skill.yaml", Cursor::new(yaml_content)).unwrap();

        // 添加 skill.md
        let md_content = b"# Test Skill\n\nThis is a test skill.";
        let mut header = tar::Header::new_gnu();
        header.set_size(md_content.len() as u64);
        header.set_cksum();
        builder.append_data(&mut header, "skill-test/skill.md", Cursor::new(md_content)).unwrap();

        builder.finish().unwrap()
    }

    #[test]
    fn test_parse_valid_tar_gz() {
        let data = create_test_tar_gz();
        let result = TarGzParser::parse(&data);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert!(parsed.manifest.is_some());
        assert!(parsed.files.contains_key("skill-test/skill.md"));
    }

    #[test]
    fn test_validate_missing_yaml() {
        let parsed = ParsedSkill::default();
        let result = TarGzParser::validate(&parsed);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("skill.yaml"));
    }

    #[test]
    fn test_validate_missing_md() {
        let parsed = ParsedSkill {
            manifest: Some(SkillManifest {
                name: "test".to_string(),
                version: "1.0.0".to_string(),
                ..Default::default()
            }),
            files: HashMap::new(),
        };
        let result = TarGzParser::validate(&parsed);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("skill.md"));
    }

    #[test]
    fn test_path_traversal_detection() {
        // 创建包含路径穿越的 tar.gz
        use std::io::Cursor;
        use tar::Builder;

        let mut builder = Builder::new(Vec::new());
        let content = b"malicious";
        let mut header = tar::Header::new_gnu();
        header.set_size(content.len() as u64);
        header.set_cksum();
        builder.append_data(&mut header, "../etc/passwd", Cursor::new(content)).unwrap();

        let data = builder.finish().unwrap();
        let result = TarGzParser::parse(&data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("路径穿越"));
    }
}