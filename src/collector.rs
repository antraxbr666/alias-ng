use crate::parser::Alias;
use anyhow::{bail, Result};
use std::process::Command;

pub struct RuntimeCollector;

impl RuntimeCollector {
    pub fn collect() -> Result<Vec<Alias>> {
        let zsh_check = Command::new("zsh").arg("--version").output();

        if zsh_check.is_err() {
            bail!("zsh is not installed. ang requires zsh to function.");
        }

        let output = Command::new("zsh")
            .args(["-fc", "source ~/.zshrc 2>/dev/null; alias -L"])
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let aliases = Self::parse_output(&stdout);
        Ok(aliases)
    }

    fn parse_output(output: &str) -> Vec<Alias> {
        output
            .lines()
            .filter(|line| line.starts_with("alias "))
            .filter_map(|line| Self::parse_line(line))
            .collect()
    }

    fn parse_line(line: &str) -> Option<Alias> {
        let rest = line.strip_prefix("alias ")?;

        let (name, value) = if rest.starts_with('\'') {
            let end = rest[1..].find('\'')?;
            let name = &rest[1..end + 1];
            let remaining = &rest[end + 2..];
            let remaining = remaining.strip_prefix('=')?;
            (name, Self::unquote_value(remaining))
        } else if rest.starts_with('"') {
            let end = rest[1..].find('"')?;
            let name = &rest[1..end + 1];
            let remaining = &rest[end + 2..];
            let remaining = remaining.strip_prefix('=')?;
            (name, Self::unquote_value(remaining))
        } else {
            let (name_part, remaining) = rest.split_once('=')?;
            (name_part, Self::unquote_value(remaining))
        };

        Some(Alias {
            group: "other".to_string(),
            name: name.to_string(),
            command: value.to_string(),
            description: String::new(),
        })
    }

    fn unquote_value(s: &str) -> String {
        let s = s.trim();
        if (s.starts_with('\'') && s.ends_with('\''))
            || (s.starts_with('"') && s.ends_with('"'))
        {
            s[1..s.len() - 1].to_string()
        } else {
            s.to_string()
        }
    }
}
