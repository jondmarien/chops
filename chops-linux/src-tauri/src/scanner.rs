use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use uuid::Uuid;
use chrono::{Utc, TimeZone};
use std::fs;
use serde_json::json;
use crate::tools::ToolSource;
use crate::parser::parse_skill_file;
use crate::db::models::Skill;

pub struct Scanner;

impl Scanner {
    pub fn scan_all(home: &str, config_home: &str, custom_paths: &[String]) -> Vec<Skill> {
        let mut skills = Vec::new();
        
        let tools = vec![
            ToolSource::Claude,
            ToolSource::Cursor,
            ToolSource::Windsurf,
            ToolSource::Codex,
            ToolSource::Amp,
            ToolSource::Agents,
        ];
        
        for tool in tools {
            if tool.is_installed(home, config_home) {
                for path in tool.global_paths(home, config_home) {
                    Self::scan_directory(&path, tool, "skill", &mut skills);
                }
                for path in tool.global_agent_paths(home) {
                    Self::scan_directory(&path, tool, "agent", &mut skills);
                }
                for path in tool.global_rule_paths(home) {
                    Self::scan_directory(&path, tool, "rule", &mut skills);
                }
            }
        }
        
        for path in custom_paths {
            Self::scan_directory(path, ToolSource::Custom, "skill", &mut skills);
        }
        
        skills
    }
    
    fn scan_directory(dir_path: &str, tool: ToolSource, kind: &str, skills: &mut Vec<Skill>) {
        let path = Path::new(dir_path);
        if !path.exists() {
            return;
        }
        
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.is_file() {
                let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
                if ext == "md" || ext == "mdc" || ext == "toml" {
                    if let Ok(skill) = Self::process_file(p, tool, kind) {
                        skills.push(skill);
                    }
                }
            }
        }
    }
    
    fn process_file(p: &Path, tool: ToolSource, kind: &str) -> anyhow::Result<Skill> {
        let metadata = fs::metadata(p)?;
        let parsed = parse_skill_file(p)?;
        
        let path_str = p.to_string_lossy().to_string();
        
        let modified = metadata.modified()
            .ok()
            .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
            .map(|secs| Utc.timestamp_opt(secs as i64, 0).unwrap());
            
        Ok(Skill {
            id: Uuid::new_v4().to_string(),
            file_path: path_str.clone(),
            resolved_path: path_str.clone(), // simplify for MVP
            tool_source: tool.name().to_string(),
            is_directory: false, // simplified
            is_global: tool == ToolSource::Agents,
            name: parsed.name,
            description: parsed.description,
            content: Some(parsed.content),
            frontmatter: parsed.frontmatter,
            installed_paths: Some(json!([path_str])),
            tool_sources: Some(json!([tool.name()])),
            file_modified_date: modified,
            file_size: Some(metadata.len() as i64),
            item_kind: Some(kind.to_string()),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        })
    }
}
