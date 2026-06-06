mod app;
mod parser;
mod ui;

use anyhow::Result;
use app::{App, AppMode};
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::env;
use std::io::stdout;
use std::path::PathBuf;
use std::time::Duration;
use ui::draw;

#[derive(Parser, Debug)]
#[command(name = "ang")]
#[command(about = "Alias Next Generation — A modern alias browser")]
#[command(version = "0.1.0")]
struct Cli {
    /// Filter aliases by group name
    #[arg(value_name = "GROUP")]
    group: Option<String>,

    /// Alias file to scan (default: ~/.zsh/04-aliases.zsh)
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Print selected alias to stdout instead of copying to clipboard
    #[arg(long)]
    print: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

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
        if cli.print {
            println!("{}", alias_name);
        } else {
            // Copy to clipboard using native Linux tools (more reliable than arboard)
            copy_to_clipboard(&alias_name);
        }
    }

    Ok(())
}

fn copy_to_clipboard(text: &str) {
    // Try Wayland first, then X11
    let copied = try_copy_command("wl-copy", &["--type", "text/plain"], text)
        || try_copy_command("xclip", &["-selection", "clipboard"], text)
        || try_copy_command("xsel", &["--clipboard", "--input"], text);

    if !copied {
        // Fallback: print to stdout so user can copy manually
        println!("{}", text);
    }
}

fn try_copy_command(cmd: &str, args: &[&str], text: &str) -> bool {
    std::process::Command::new(cmd)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(text.as_bytes())?;
            }
            child.wait()
        })
        .map(|status| status.success())
        .unwrap_or(false)
}

fn run_tui(aliases: Vec<parser::Alias>) -> Result<Option<String>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(aliases);
    let mut result: Option<String> = None;
    let mut should_exit = false;

    loop {
        terminal.draw(|f| draw(f, &app))?;

        if should_exit {
            std::thread::sleep(Duration::from_millis(400));
            break;
        }

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
                                app.copied = Some(alias.name.clone());
                                should_exit = true;
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
