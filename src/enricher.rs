use crate::parser::{Alias, Parser};
use std::collections::HashMap;

pub struct MetadataEnricher;

#[derive(Clone)]
struct AliasMeta {
    group: String,
    description: String,
}

impl MetadataEnricher {
    pub fn enrich(runtime_aliases: &mut [Alias], files: &[std::path::PathBuf]) {
        let metadata = Self::build_metadata(files);

        for alias in runtime_aliases.iter_mut() {
            if let Some(meta) = metadata.get(&alias.name) {
                if meta.group != "other" {
                    alias.group = meta.group.clone();
                }
                if !meta.description.is_empty() {
                    alias.description = meta.description.clone();
                }
            }
        }
    }

    fn build_metadata(files: &[std::path::PathBuf]) -> HashMap<String, AliasMeta> {
        let mut map = HashMap::new();

        for file in files {
            if let Ok(content) = std::fs::read_to_string(file) {
                let aliases = Parser::parse_content(&content);
                for alias in aliases {
                    map.insert(
                        alias.name.clone(),
                        AliasMeta {
                            group: alias.group,
                            description: alias.description,
                        },
                    );
                }
            }
        }

        map
    }
}
