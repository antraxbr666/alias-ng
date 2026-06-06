use std::fs;
use std::path::Path;

/// Represents a parsed alias entry
#[derive(Debug, Clone)]
pub struct Alias {
    pub group: String,
    pub name: String,
    pub command: String,
    pub description: String,
}

/// Parser for alias source files
pub struct Parser;

impl Parser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Alias>> {
        let content = fs::read_to_string(path)?;
        Ok(Self::parse_content(&content))
    }

    pub fn parse_content(content: &str) -> Vec<Alias> {
        let mut aliases = Vec::new();
        let mut current_group = "ungrouped".to_string();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Group header: # GroupName (but not # Key: value or separators)
            if let Some(header) = trimmed.strip_prefix('#') {
                let header = header.trim();
                if !header.is_empty()
                    && !header.contains(':')
                    && !header.chars().all(|c| "=-* ".contains(c))
                    && header.len() > 2
                {
                    current_group = header.to_string();
                }
                continue;
            }

            // Alias definition
            if trimmed.starts_with("alias ") {
                if let Some(alias) = Self::parse_alias_line(trimmed, &current_group) {
                    aliases.push(alias);
                }
            }
        }

        aliases
    }

    fn parse_alias_line(line: &str, group: &str) -> Option<Alias> {
        // Remove "alias " prefix
        let rest = line.strip_prefix("alias ")?;

        // Split name and value at first =
        let (name_part, raw_value) = rest.split_once('=')?;
        let name = name_part.trim().to_string();

        let (command, description) = Self::extract_value_desc(raw_value.trim());

        Some(Alias {
            group: group.to_string(),
            name,
            command,
            description,
        })
    }

    fn extract_value_desc(raw: &str) -> (String, String) {
        // Try to find inline comment
        if let Some(pos) = raw.rfind(" # ") {
            let (value_part, desc) = raw.split_at(pos);
            let desc = desc.trim_start_matches(" # ").trim().to_string();
            let value = Self::unquote(value_part.trim()).to_string();
            return (value, desc);
        }

        // No inline comment
        (Self::unquote(raw).to_string(), String::new())
    }

    fn unquote(s: &str) -> &str {
        if (s.starts_with('\'') && s.ends_with('\'')) || (s.starts_with('"') && s.ends_with('"'))
        {
            &s[1..s.len() - 1]
        } else {
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_alias() {
        let content = r#"
# Docker
alias dcud="docker compose up -d"  # Start containers
"#;
        let aliases = Parser::parse_content(content);
        assert_eq!(aliases.len(), 1);
        assert_eq!(aliases[0].group, "Docker");
        assert_eq!(aliases[0].name, "dcud");
        assert_eq!(aliases[0].command, "docker compose up -d");
        assert_eq!(aliases[0].description, "Start containers");
    }

    #[test]
    fn test_parse_ungrouped() {
        let content = r#"alias ls='eza --color'"#;
        let aliases = Parser::parse_content(content);
        assert_eq!(aliases.len(), 1);
        assert_eq!(aliases[0].group, "ungrouped");
        assert_eq!(aliases[0].name, "ls");
        assert_eq!(aliases[0].command, "eza --color");
    }

    #[test]
    fn test_ignore_reference() {
        let content = r#"
# Sistema: ls, la, l
# Sistema
alias ls='eza --color'  # List files
"#;
        let aliases = Parser::parse_content(content);
        assert_eq!(aliases.len(), 1);
        assert_eq!(aliases[0].group, "Sistema");
    }
}
