use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::{Result, anyhow};
use serde_json::Value as JsonValue;
use regex::Regex;

#[derive(Debug, Default)]
pub struct ParsedSkill {
    pub name: String,
    pub description: Option<String>,
    pub frontmatter: Option<JsonValue>,
    pub content: String,
}

pub fn parse_skill_file<P: AsRef<Path>>(path: P) -> Result<ParsedSkill> {
    let path = path.as_ref();
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let raw = fs::read_to_string(path)?;

    match ext {
        "md" => parse_markdown(&raw),
        "mdc" => parse_mdc(&raw),
        "toml" => parse_toml(&raw),
        _ => parse_markdown(&raw), // Default fallback
    }
}

fn parse_markdown(raw: &str) -> Result<ParsedSkill> {
    let re = Regex::new(r"(?s)^---\n(.*?)\n---\n(.*)").unwrap();
    if let Some(caps) = re.captures(raw) {
        let frontmatter_str = caps.get(1).unwrap().as_str();
        let content = caps.get(2).unwrap().as_str().trim().to_string();
        
        let mut frontmatter: HashMap<String, JsonValue> = serde_yaml::from_str(frontmatter_str)?;
        let name = frontmatter.get("name").and_then(|v| v.as_str()).unwrap_or("Untitled").to_string();
        let description = frontmatter.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        Ok(ParsedSkill {
            name,
            description,
            frontmatter: Some(serde_json::to_value(frontmatter)?),
            content,
        })
    } else {
        Ok(ParsedSkill {
            name: "Untitled".to_string(),
            description: None,
            frontmatter: None,
            content: raw.to_string(),
        })
    }
}

fn parse_mdc(raw: &str) -> Result<ParsedSkill> {
    let mut lines = raw.lines();
    let mut name = String::from("Untitled");
    let mut description = None;
    
    // Simplistic parsing for MDC:
    // # Name
    // description: Desc
    // 
    // content
    
    if let Some(first) = lines.next() {
        if first.starts_with("# ") {
            name = first[2..].trim().to_string();
        }
    }
    
    let mut content = String::new();
    let mut in_content = false;
    for line in lines {
        if !in_content && line.starts_with("description:") {
            description = Some(line["description:".len()..].trim().to_string());
        } else if line.trim().is_empty() && !in_content {
            in_content = true;
        } else if in_content {
            content.push_str(line);
            content.push('\n');
        }
    }
    
    Ok(ParsedSkill {
        name,
        description,
        frontmatter: None,
        content: content.trim().to_string(),
    })
}

fn parse_toml(raw: &str) -> Result<ParsedSkill> {
    // Simple TOML parsing for MVP
    let doc = raw.parse::<toml::Value>()?;
    
    let name = doc.get("metadata")
        .and_then(|m| m.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("Untitled")
        .to_string();
        
    let description = doc.get("metadata")
        .and_then(|m| m.get("description"))
        .and_then(|d| d.as_str())
        .map(|s| s.to_string());
        
    let content = doc.get("content")
        .and_then(|c| c.get("text"))
        .and_then(|t| t.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
        
    Ok(ParsedSkill {
        name,
        description,
        frontmatter: None,
        content,
    })
}
