use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// 技能验证配置
pub struct ValidatorConfig {
    /// 最大文件大小（字节）
    pub max_file_size: usize,
    /// 最大压缩比（防止压缩炸弹）
    pub max_compression_ratio: usize,
    /// 危险文件扩展名
    pub dangerous_extensions: Vec<&'static str>,
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_compression_ratio: 10,
            dangerous_extensions: vec![
                ".exe", ".bat", ".cmd", ".sh", ".dll", ".so", ".dylib",
                ".app", ".dmg", ".pkg", ".deb", ".rpm",
            ],
        }
    }
}

/// 技能内容验证器
pub struct SkillValidator {
    config: ValidatorConfig,
}

impl Default for SkillValidator {
    fn default() -> Self {
        Self::new(ValidatorConfig::default())
    }
}

impl SkillValidator {
    pub fn new(config: ValidatorConfig) -> Self {
        Self { config }
    }

    /// 验证技能文件
    pub fn validate(&self, files: &HashMap<String, Vec<u8>>) -> Result<()> {
        for (path, content) in files {
            // 检查文件大小
            self.check_file_size(path, content.len())?;

            // 检查危险文件类型
            self.check_dangerous_file(path)?;

            // 检查敏感内容
            self.check_sensitive_content(path, content)?;
        }

        Ok(())
    }

    /// 验证压缩比
    pub fn validate_compression_ratio(
        &self,
        compressed_size: usize,
        decompressed_size: usize,
    ) -> Result<()> {
        if compressed_size == 0 {
            return Ok(());
        }

        let ratio = decompressed_size / compressed_size;
        if ratio > self.config.max_compression_ratio {
            return Err(anyhow!(
                "压缩比过高 ({}x)，可能为压缩炸弹",
                ratio
            ));
        }

        Ok(())
    }

    fn check_file_size(&self, path: &str, size: usize) -> Result<()> {
        if size > self.config.max_file_size {
            return Err(anyhow!(
                "文件 {} 大小超过限制 ({} > {})",
                path,
                format_size(size),
                format_size(self.config.max_file_size)
            ));
        }
        Ok(())
    }

    fn check_dangerous_file(&self, path: &str) -> Result<()> {
        let lower_path = path.to_lowercase();
        for ext in &self.config.dangerous_extensions {
            if lower_path.ends_with(ext) {
                return Err(anyhow!("禁止上传危险文件类型: {}", path));
            }
        }
        Ok(())
    }

    fn check_sensitive_content(&self, path: &str, content: &[u8]) -> Result<()> {
        // 检查是否包含敏感关键词（简单实现）
        let sensitive_patterns: &[&[u8]] = &[
            b"password",
            b"secret",
            b"api_key",
            b"private_key",
        ];

        // 只检查文本文件
        let lower_path = path.to_lowercase();
        if lower_path.ends_with(".md") || lower_path.ends_with(".txt") || lower_path.ends_with(".yaml") {
            let content_lower: Vec<u8> = content.iter().map(|c| c.to_ascii_lowercase()).collect();
            for pattern in sensitive_patterns {
                if content_lower.windows(pattern.len()).any(|w| w == *pattern) {
                    tracing::warn!("文件 {} 可能包含敏感信息", path);
                    // 仅警告，不阻止
                }
            }
        }

        Ok(())
    }
}

/// 格式化文件大小
fn format_size(size: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = 1024 * KB;

    if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_file_size_ok() {
        let validator = SkillValidator::default();
        let mut files = HashMap::new();
        files.insert("test.md".to_string(), vec![0u8; 1000]);
        assert!(validator.validate(&files).is_ok());
    }

    #[test]
    fn test_validate_file_size_exceeded() {
        let config = ValidatorConfig {
            max_file_size: 100, // 100 bytes
            ..Default::default()
        };
        let validator = SkillValidator::new(config);
        let mut files = HashMap::new();
        files.insert("large.md".to_string(), vec![0u8; 200]);
        let result = validator.validate(&files);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("大小超过限制"));
    }

    #[test]
    fn test_validate_dangerous_file() {
        let validator = SkillValidator::default();
        let mut files = HashMap::new();
        files.insert("malware.exe".to_string(), vec![0u8; 100]);
        let result = validator.validate(&files);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("危险文件"));
    }

    #[test]
    fn test_validate_compression_ratio_ok() {
        let validator = SkillValidator::default();
        let result = validator.validate_compression_ratio(1000, 5000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_compression_ratio_exceeded() {
        let validator = SkillValidator::default();
        let result = validator.validate_compression_ratio(100, 2000); // 20x ratio
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("压缩比过高"));
    }
}