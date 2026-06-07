use crate::parser::Alias;

#[derive(Debug, Clone)]
pub enum AppMode {
    Normal,
    Search,
}

pub struct App {
    pub aliases: Vec<Alias>,
    pub filtered: Vec<usize>, // indices into aliases
    pub selected: usize,
    pub search_query: String,
    pub mode: AppMode,
}

impl App {
    pub fn new(aliases: Vec<Alias>) -> Self {
        let count = aliases.len();
        Self {
            aliases,
            filtered: (0..count).collect(),
            selected: 0,
            search_query: String::new(),
            mode: AppMode::Normal,
        }
    }

    pub fn filter(&mut self) {
        if self.search_query.is_empty() {
            self.filtered = (0..self.aliases.len()).collect();
        } else {
            let query = self.search_query.to_lowercase();
            self.filtered = self
                .aliases
                .iter()
                .enumerate()
                .filter(|(_, a)| a.command.to_lowercase().contains(&query))
                .map(|(i, _)| i)
                .collect();
        }
        self.selected = self.selected.min(self.filtered.len().saturating_sub(1));
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.filtered.len() {
            self.selected += 1;
        }
    }

    pub fn selected_alias(&self) -> Option<&Alias> {
        self.filtered.get(self.selected).map(|&i| &self.aliases[i])
    }

    pub fn toggle_mode(&mut self) {
        match self.mode {
            AppMode::Normal => self.mode = AppMode::Search,
            AppMode::Search => self.mode = AppMode::Normal,
        }
    }
}
