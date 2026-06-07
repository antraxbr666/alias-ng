mod app;
mod collector;
mod discovery;
mod enricher;
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
    #[arg(value_name = "GROUP")]
    group: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    #[arg(long)]
    print: bool,

    #[arg(long = "generate-zsh-completion")]
    generate_zsh_completion: bool,

    #[arg(long = "output-file", value_name = "FILE")]
    output_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

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

    let aliases = if let Some(ref file) = cli.file {
        parser::Parser::parse_file(file)?
    } else {
        let mut aliases = collector::RuntimeCollector::collect()?;

        let files = discovery::FileDiscovery::discover();
        enricher::MetadataEnricher::enrich(&mut aliases, &files);

        aliases.sort_by(|a, b| {
            let group_cmp = a.group.to_lowercase().cmp(&b.group.to_lowercase());
            if group_cmp == std::cmp::Ordering::Equal {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            } else {
                group_cmp
            }
        });
        aliases.dedup_by(|a, b| a.name == b.name);

        aliases
    };

    let mut aliases = aliases;

    if let Some(ref group_filter) = cli.group {
        aliases.retain(|a| a.group.to_lowercase().contains(&group_filter.to_lowercase()));
    }

    if aliases.is_empty() {
        eprintln!("ang: No aliases found.");
        std::process::exit(1);
    }

    if cli.print {
        for alias in &aliases {
            println!(
                "{}\t{}\t{}\t{}",
                alias.group, alias.name, alias.command, alias.description
            );
        }
        return Ok(());
    }

    let selected = run_tui(aliases)?;

    if let Some(alias_name) = selected {
        if let Some(output_path) = cli.output_file {
            std::fs::write(&output_path, &alias_name)?;
        } else {
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
