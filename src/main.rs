mod app;
mod parser;
mod ui;

use anyhow::Result;
use app::{App, AppMode};
use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::env;
use std::io::{self, stdout, BufWriter};
use std::path::PathBuf;
use std::time::Duration;
use ui::draw;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(name = "ang")]
#[command(about = "Alias Next Generation — A modern alias browser")]
#[command(version = VERSION)]
struct Cli {
    /// Filter aliases by group name
    #[arg(value_name = "GROUP")]
    group: Option<String>,

    /// Alias file to scan (default: ~/.zsh/04-aliases.zsh)
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Print all aliases as TSV and exit
    #[arg(long)]
    print: bool,

    /// Generate zsh completion script
    #[arg(long = "generate-zsh-completion")]
    generate_zsh_completion: bool,

    /// Write selected alias to file instead of stdout (for zsh widget integration)
    #[arg(long = "output-file", value_name = "FILE")]
    output_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle --generate-zsh-completion
    if cli.generate_zsh_completion {
        let mut cmd = Cli::command();
        let bin_name = cmd.get_name().to_string();
        generate(
            Shell::Zsh,
            &mut cmd,
            bin_name,
            &mut BufWriter::new(io::stdout().lock()),
        );
        return Ok(());
    }

    // Determine alias file to parse
    let alias_file = cli.file.unwrap_or_else(|| {
        env::var("ANG_ALIAS_FILES")
            .ok()
            .and_then(|s| s.split(':').next().map(PathBuf::from))
            .unwrap_or_else(|| {
                dirs::home_dir()
                    .unwrap_or_default()
                    .join(".zsh/04-aliases.zsh")
            })
    });

    // Parse aliases
    let mut aliases = parser::Parser::parse_file(&alias_file)?;

    // Filter by group if specified
    if let Some(ref group_filter) = cli.group {
        aliases.retain(|a| a.group.to_lowercase().contains(&group_filter.to_lowercase()));
    }

    if aliases.is_empty() {
        eprintln!("ang: No aliases found.");
        std::process::exit(1);
    }

    // If --print flag, just print the list and exit (for scripting)
    if cli.print {
        for alias in &aliases {
            println!("{}\t{}\t{}\t{}", alias.group, alias.name, alias.command, alias.description);
        }
        return Ok(());
    }

    // Run TUI
    let selected = run_tui(aliases)?;

    if let Some(alias_name) = selected {
        if let Some(output_path) = cli.output_file {
            // Write to file for widget integration (silent)
            std::fs::write(&output_path, &alias_name)?;
        } else {
            // Print to stdout for standalone usage
            println!("{}", alias_name);
        }
    }

    Ok(())
}

fn run_tui(aliases: Vec<parser::Alias>) -> Result<Option<String>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(aliases);
    let mut result: Option<String> = None;

    loop {
        terminal.draw(|f| draw(f, &app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.mode {
                    AppMode::Normal => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('/') | KeyCode::Char('s') => {
                            app.toggle_mode();
                        }
                        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
                        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
                        KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.selected = 0;
                        }
                        KeyCode::Char('g') => app.selected = 0,
                        KeyCode::Char('G') => {
                            app.selected = app.filtered.len().saturating_sub(1);
                        }
                        KeyCode::Enter => {
                            if let Some(alias) = app.selected_alias() {
                                result = Some(alias.name.clone());
                                break;
                            }
                        }
                        _ => {}
                    },
                    AppMode::Search => match key.code {
                        KeyCode::Esc => {
                            app.search_query.clear();
                            app.filter();
                            app.toggle_mode();
                        }
                        KeyCode::Backspace => {
                            app.search_query.pop();
                            app.filter();
                        }
                        KeyCode::Enter => {
                            app.toggle_mode();
                        }
                        KeyCode::Char(c) => {
                            app.search_query.push(c);
                            app.filter();
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;

    Ok(result)
}
