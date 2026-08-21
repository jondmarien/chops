use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolSource {
    Claude,
    Cursor,
    Windsurf,
    Codex,
    Amp,
    Agents, // Global
    Custom,
}

impl ToolSource {
    pub fn name(&self) -> &'static str {
        match self {
            ToolSource::Claude => "Claude Code",
            ToolSource::Cursor => "Cursor",
            ToolSource::Windsurf => "Windsurf",
            ToolSource::Codex => "Codex",
            ToolSource::Amp => "Amp",
            ToolSource::Agents => "Global (Agents)",
            ToolSource::Custom => "Custom",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ToolSource::Claude => "claude",
            ToolSource::Cursor => "cursor",
            ToolSource::Windsurf => "windsurf",
            ToolSource::Codex => "codex",
            ToolSource::Amp => "amp",
            ToolSource::Agents => "agents",
            ToolSource::Custom => "custom",
        }
    }

    pub fn global_paths(&self, home: &str, config_home: &str) -> Vec<String> {
        match self {
            ToolSource::Claude => vec![format!("{}/.claude/skills", home)],
            ToolSource::Cursor => vec![format!("{}/.cursor/skills", home)],
            ToolSource::Codex => vec![format!("{}/.codex/skills", home)],
            ToolSource::Amp => vec![format!("{}/amp/skills", config_home)],
            ToolSource::Agents => vec![format!("{}/.agents/skills", home)],
            _ => vec![],
        }
    }

    pub fn global_agent_paths(&self, home: &str) -> Vec<String> {
        match self {
            ToolSource::Claude => vec![format!("{}/.claude/agents", home)],
            ToolSource::Cursor => vec![format!("{}/.cursor/agents", home)],
            ToolSource::Codex => vec![format!("{}/.codex/agents", home)],
            _ => vec![],
        }
    }

    pub fn global_rule_paths(&self, home: &str) -> Vec<String> {
        match self {
            ToolSource::Cursor => vec![format!("{}/.cursor/rules", home)],
            ToolSource::Windsurf => vec![
                format!("{}/.codeium/windsurf/memories", home),
                format!("{}/.windsurf/rules", home),
            ],
            _ => vec![],
        }
    }

    pub fn is_installed(&self, home: &str, config_home: &str) -> bool {
        let path_exists = |p: &str| Path::new(p).exists();
        
        match self {
            ToolSource::Claude => path_exists(&format!("{}/.claude", home)),
            ToolSource::Cursor => path_exists(&format!("{}/.cursor", home)) || path_exists(&format!("{}/.config/Cursor", home)),
            ToolSource::Windsurf => path_exists(&format!("{}/.codeium", home)) || path_exists(&format!("{}/.windsurf", home)),
            ToolSource::Codex => path_exists(&format!("{}/.codex", home)),
            ToolSource::Amp => path_exists(&format!("{}/amp", config_home)),
            ToolSource::Agents => path_exists(&format!("{}/.agents", home)),
            ToolSource::Custom => true,
        }
    }
}
