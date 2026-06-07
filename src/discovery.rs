use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FileDiscovery;

impl FileDiscovery {
    pub fn discover() -> Vec<PathBuf> {
        let mut files = Vec::new();

        let base = Self::get_base_dir();
        let zshrc = base.join(".zshrc");

        if zshrc.exists() {
            if let Ok(content) = fs::read_to_string(&zshrc) {
                let sourced = Self::parse_source_commands(&content, &base);
                files.extend(sourced);
            }
        }

        let zsh_dir = base.join(".zsh");
        if zsh_dir.exists() {
            files.extend(Self::scan_dir(&zsh_dir));
        }

        let xdg_config = env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| base.join(".config"));

        let config_zsh = xdg_config.join("zsh");
        if config_zsh.exists() {
            files.extend(Self::scan_dir(&config_zsh));
        }

        let home = dirs::home_dir().unwrap_or_default();
        let oh_my_zsh = home.join(".oh-my-zsh").join("plugins");
        if oh_my_zsh.exists() {
            files.extend(Self::scan_plugins(&oh_my_zsh));
        }

        files.sort();
        files.dedup();
        files
    }

    fn get_base_dir() -> PathBuf {
        env::var("ZDOTDIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| dirs::home_dir().unwrap_or_default())
    }

    fn parse_source_commands(content: &str, base: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }

            if let Some(path) = Self::extract_source_path(trimmed) {
                let expanded = shellexpand::tilde(&path);
                let path = PathBuf::from(expanded.as_ref());

                let path = if path.is_relative() {
                    base.join(path)
                } else {
                    path
                };

                if path.exists() && path.is_file() {
                    files.push(path);
                }
            }
        }

        files
    }

    fn extract_source_path(line: &str) -> Option<String> {
        let line = line.trim();

        if line.starts_with("source ") {
            let path = line.strip_prefix("source ")?.trim();
            Some(Self::clean_path(path))
        } else if line.starts_with(". ") {
            let path = line.strip_prefix(". ")?.trim();
            Some(Self::clean_path(path))
        } else {
            None
        }
    }

    fn clean_path(path: &str) -> String {
        let path = path.trim();
        if (path.starts_with('"') && path.ends_with('"'))
            || (path.starts_with('\'') && path.ends_with('\''))
        {
            path[1..path.len() - 1].to_string()
        } else {
            path.to_string()
        }
    }

    fn scan_dir(dir: &Path) -> Vec<PathBuf> {
        let pattern = dir.join("*.zsh");
        let pattern_str = pattern.to_string_lossy();

        glob::glob(&pattern_str)
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.ok())
            .filter(|path| path.is_file())
            .collect()
    }

    fn scan_plugins(dir: &Path) -> Vec<PathBuf> {
        let pattern = dir.join("*").join("*.plugin.zsh");
        let pattern_str = pattern.to_string_lossy();

        glob::glob(&pattern_str)
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.ok())
            .filter(|path| path.is_file())
            .collect()
    }
}
