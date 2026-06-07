use std::fs;
use std::path::PathBuf;

pub struct FileDiscovery;

impl FileDiscovery {
    pub fn discover() -> Vec<PathBuf> {
        let home = dirs::home_dir().unwrap_or_default();
        let zsh_dir = home.join(".zsh");

        if !zsh_dir.exists() {
            return Vec::new();
        }

        let pattern = zsh_dir.join("*.zsh");
        let pattern_str = pattern.to_string_lossy();

        glob::glob(&pattern_str)
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.ok())
            .filter(|path| path.is_file())
            .filter(|path| Self::contains_aliases(path))
            .collect()
    }

    fn contains_aliases(path: &PathBuf) -> bool {
        if let Ok(content) = fs::read_to_string(path) {
            content.lines().any(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("alias ") || trimmed.starts_with("alias\t")
            })
        } else {
            false
        }
    }
}
