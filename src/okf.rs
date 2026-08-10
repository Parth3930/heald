use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Frontmatter {
    /// `type` is optional — skills often omit it and use `name` instead.
    #[serde(default = "default_type")]
    pub r#type: String,
    pub title: Option<String>,
    /// `name` is an alias for `title` used by some skill formats.
    pub name: Option<String>,
    pub description: Option<String>,
    pub resource: Option<String>,
    pub tags: Option<Vec<String>>,
    pub timestamp: Option<String>,
    pub triggers: Option<Vec<String>>,
}

fn default_type() -> String { "skill".to_string() }

impl Frontmatter {
    /// Returns the best available title: `title` > `name` > type.
    pub fn effective_title(&self) -> &str {
        self.title.as_deref()
            .or(self.name.as_deref())
            .unwrap_or(&self.r#type)
    }
}

impl Default for Frontmatter {
    fn default() -> Self {
        Self {
            r#type: default_type(),
            title: None,
            name: None,
            description: None,
            resource: None,
            tags: None,
            timestamp: None,
            triggers: None,
        }
    }
}


#[derive(Debug, Clone)]
pub struct Document {
    pub frontmatter: Frontmatter,
    pub content: String,
}

impl Document {
    pub fn parse(content: &str) -> Result<Self, String> {
        if !content.starts_with("---\n") && !content.starts_with("---\r\n") {
            return Err("Missing YAML frontmatter".into());
        }
        
        let split_idx = content[4..].find("\n---").map(|i| i + 4);
        if let Some(idx) = split_idx {
            let frontmatter_str = &content[4..idx];
            let content_str = &content[idx + 4..].trim_start();
            
            let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter_str)
                .map_err(|e| format!("Invalid YAML frontmatter: {}", e))?;
                
            Ok(Self {
                frontmatter,
                content: content_str.to_string(),
            })
        } else {
            Err("Unclosed YAML frontmatter".into())
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        Self::parse(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_document() {
        let doc = "---\ntype: decision\ntitle: test\n---\nbody";
        let parsed = Document::parse(doc).unwrap();
        assert_eq!(parsed.frontmatter.r#type, "decision");
        assert_eq!(parsed.frontmatter.title.unwrap(), "test");
        assert_eq!(parsed.content, "body");
    }

    #[test]
    fn test_missing_type() {
        let doc = "---\ntitle: test\n---\nbody";
        let parsed = Document::parse(doc).unwrap();
        assert_eq!(parsed.frontmatter.r#type, "skill");
    }

    #[test]
    fn test_malformed_yaml() {
        let doc = "---\ntype: [decision\n---\nbody";
        let parsed = Document::parse(doc);
        assert!(parsed.is_err());
    }
}

