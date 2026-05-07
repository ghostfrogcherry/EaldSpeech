mod translate;

use crossterm::{
    event::{
        self, DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste,
        EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{error::Error, io, time::Duration};

use translate::Anglish;

#[derive(Clone, Copy)]
struct Theme {
    name: &'static str,
    title: Color,
    input_border: Color,
    input_text: Color,
    output_border: Color,
    output_text: Color,
    arrow: Color,
    status: Color,
    flash: Color,
}

const THEMES: &[Theme] = &[
    Theme { name: "Catppuccin", title: Color::Rgb(203, 166, 247), input_border: Color::Rgb(137, 180, 250), input_text: Color::Rgb(205, 214, 244), output_border: Color::Rgb(166, 227, 161), output_text: Color::Rgb(205, 214, 244), arrow: Color::Rgb(245, 194, 231), status: Color::Rgb(147, 153, 178), flash: Color::Rgb(249, 226, 175) },
    Theme { name: "Nord",       title: Color::Rgb(136, 192, 208), input_border: Color::Rgb(94, 129, 172),  input_text: Color::Rgb(236, 239, 244), output_border: Color::Rgb(163, 190, 140), output_text: Color::Rgb(236, 239, 244), arrow: Color::Rgb(208, 135, 112), status: Color::Rgb(148, 152, 163), flash: Color::Rgb(235, 203, 139) },
    Theme { name: "Dracula",    title: Color::Rgb(189, 147, 249), input_border: Color::Rgb(139, 233, 253), input_text: Color::Rgb(248, 248, 242), output_border: Color::Rgb(80, 250, 123),  output_text: Color::Rgb(248, 248, 242), arrow: Color::Rgb(255, 121, 198), status: Color::Rgb(98, 114, 164),  flash: Color::Rgb(241, 250, 140) },
    Theme { name: "Tokyo Night",title: Color::Rgb(187, 154, 247), input_border: Color::Rgb(122, 162, 247), input_text: Color::Rgb(169, 177, 214), output_border: Color::Rgb(158, 206, 106), output_text: Color::Rgb(169, 177, 214), arrow: Color::Rgb(224, 175, 104), status: Color::Rgb(120, 126, 156), flash: Color::Rgb(224, 175, 104) },
    Theme { name: "One Dark",   title: Color::Rgb(198, 120, 221), input_border: Color::Rgb(97, 175, 239),  input_text: Color::Rgb(171, 178, 191), output_border: Color::Rgb(152, 195, 121), output_text: Color::Rgb(171, 178, 191), arrow: Color::Rgb(229, 192, 123), status: Color::Rgb(92, 99, 112),   flash: Color::Rgb(229, 192, 123) },
    Theme { name: "Gruvbox",    title: Color::Rgb(131, 165, 152), input_border: Color::Rgb(184, 187, 38),  input_text: Color::Rgb(235, 219, 178), output_border: Color::Rgb(254, 128, 25),  output_text: Color::Rgb(235, 219, 178), arrow: Color::Rgb(214, 93, 14),   status: Color::Rgb(146, 131, 116), flash: Color::Rgb(250, 189, 47) },
];

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Fwd,
    Rev,
}

struct Flash {
    message: String,
    color: Color,
    expires: u64,
}

struct App {
    input: String,
    translated: String,
    anglish: Anglish,
    mode: Mode,
    theme_idx: usize,
    frame: u64,
    show_about: bool,
    flash: Option<Flash>,
    clipboard: Option<arboard::Clipboard>,
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
            translated: String::new(),
            anglish: Anglish::new(),
            mode: Mode::Fwd,
            theme_idx: 0,
            frame: 0,
            show_about: false,
            flash: None,
            clipboard: arboard::Clipboard::new().ok(),
        }
    }

    fn translate(&mut self) {
        self.translated = match self.mode {
            Mode::Fwd => self.anglish.translate(&self.input),
            Mode::Rev => self.anglish.translate_reverse(&self.input),
        };
    }

    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Fwd => Mode::Rev,
            Mode::Rev => Mode::Fwd,
        };
        let msg = match self.mode {
            Mode::Fwd => " English \u{2192} Anglish ",
            Mode::Rev => " Anglish \u{2192} English ",
        };
        let theme = THEMES[self.theme_idx];
        self.flash = Some(Flash {
            message: msg.to_string(),
            color: theme.flash,
            expires: self.frame + 15,
        });
        self.translate();
    }

    fn toggle_about(&mut self) {
        self.show_about = !self.show_about;
    }

    fn cycle_theme(&mut self) {
        self.theme_idx = (self.theme_idx + 1) % THEMES.len();
        let theme = THEMES[self.theme_idx];
        self.flash = Some(Flash {
            message: format!(" Theme: {} ", theme.name),
            color: theme.flash,
            expires: self.frame + 15,
        });
    }

    fn clear(&mut self) {
        self.input.clear();
        self.translated.clear();
    }

    fn copy_output(&mut self) {
        if let Some(ref mut cb) = self.clipboard {
            if cb.set_text(self.translated.clone()).is_ok() {
                let theme = THEMES[self.theme_idx];
                self.flash = Some(Flash {
                    message: " Copied output to clipboard! ".into(),
                    color: theme.flash,
                    expires: self.frame + 15,
                });
                return;
            }
        }
        self.flash = Some(Flash {
            message: " Clipboard not available ".into(),
            color: Color::Red,
            expires: self.frame + 15,
        });
    }

    fn paste(&mut self, text: &str) {
        self.input.push_str(text);
        self.translate();
    }

    fn stats(&self) -> String {
        let words = if self.input.is_empty() {
            0
        } else {
            self.input.split_whitespace().count()
        };
        let chars = self.input.chars().count();
        let mode_str = match self.mode {
            Mode::Fwd => "EN\u{2192}ANG",
            Mode::Rev => "ANG\u{2192}EN",
        };
        format!(
            " {} | Theme: {} | {} words {} chars ",
            mode_str,
            THEMES[self.theme_idx].name,
            words,
            chars,
        )
    }

    fn handle_key(&mut self, key: KeyCode, mods: KeyModifiers) -> bool {
        if self.show_about {
            match key {
                KeyCode::Esc | KeyCode::Char('a') if mods.contains(KeyModifiers::CONTROL) => {
                    self.show_about = false;
                }
                _ => {}
            }
            return true;
        }
        let ctrl = mods.contains(KeyModifiers::CONTROL);
        match key {
            KeyCode::Esc => return false,
            KeyCode::Tab => self.cycle_theme(),
            KeyCode::Char('a') if ctrl => self.toggle_about(),
            KeyCode::Char('q') if ctrl => return false,
            KeyCode::Char('r') if ctrl => self.toggle_mode(),
            KeyCode::Char('c') if ctrl => self.copy_output(),
            KeyCode::Char('l') if ctrl => self.clear(),
            KeyCode::Char(c) if !ctrl => {
                self.input.push(c);
                self.translate();
            }
            KeyCode::Backspace => {
                self.input.pop();
                self.translate();
            }
            KeyCode::Enter => {
                self.input.push('\n');
                self.translate();
            }
            _ => {}
        }
        true
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(
        stderr,
        EnterAlternateScreen,
        EnableMouseCapture,
        EnableBracketedPaste,
    )?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        DisableBracketedPaste,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stderr>>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        app.frame += 1;

        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if !app.handle_key(key.code, key.modifiers) {
                        return Ok(());
                    }
                }
                Event::Paste(text) => app.paste(&text),
                _ => {}
            }
        }
    }
}

fn about_page(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Min(10),
            Constraint::Percentage(15),
        ])
        .split(area);
    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Min(40),
            Constraint::Percentage(15),
        ])
        .split(vert[1]);

    let theme = THEMES[app.theme_idx];
    let text = vec![
        Line::from(Span::styled(
            "  EALDSP\u{0112}C \u{00d6}VERSETTEND  ",
            Style::default().fg(theme.title).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Anglish is English stripped of French and Latin"),
        Line::from("influence, using only words of Germanic origin."),
        Line::from("This app translates modern English into Anglish"),
        Line::from("and back again."),
        Line::from(""),
        Line::from(Span::styled(" How to use ", Style::default().fg(theme.title).add_modifier(Modifier::BOLD))),
        Line::from("  Type in the top panel \u{2014} the translation"),
        Line::from("  appears below in real time."),
        Line::from("  Press Ctrl+R to switch direction."),
        Line::from(""),
        Line::from(Span::styled(" Keybindings ", Style::default().fg(theme.title).add_modifier(Modifier::BOLD))),
        Line::from("  Ctrl+Q / Esc    Quit"),
        Line::from("  Tab             Cycle theme"),
        Line::from("  Ctrl+R          Toggle direction"),
        Line::from("  Ctrl+A          About page"),
        Line::from("  Ctrl+C          Copy output"),
        Line::from("  Ctrl+L          Clear input"),
        Line::from(""),
        Line::from(Span::styled(" Credits ", Style::default().fg(theme.title).add_modifier(Modifier::BOLD))),
        Line::from("  Rust + ratatui + crossterm"),
        Line::from("  arboard for clipboard"),
        Line::from("  Anglish is a hobbyist conlang."),
        Line::from(""),
        Line::from(Span::styled("  Ctrl+A or Esc to close ", Style::default().fg(Color::DarkGray))),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.title))
        .style(Style::default().bg(Color::Rgb(0, 0, 0)));
    let paragraph = Paragraph::new(Text::from(text))
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
    f.render_widget(Clear, horiz[1]);
    f.render_widget(paragraph, horiz[1]);
}

fn ui(f: &mut Frame, app: &mut App) {
    if app.show_about {
        about_page(f, app);
        return;
    }
    let theme = THEMES[app.theme_idx];
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(3),
                Constraint::Length(1),
                Constraint::Min(3),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(size);

    // title
    let title = Line::from(Span::styled(
        "  EALDSP\u{0112}C   \u{00d6}VERSETTEND  ",
        Style::default().fg(theme.title).add_modifier(Modifier::BOLD),
    ));
    f.render_widget(title, chunks[0]);

    // input panel
    let input_title = if app.mode == Mode::Fwd {
        " English "
    } else {
        " Ealdsp\u{0113}c (to overset) "
    };
    let input_block = Block::default()
        .title(input_title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.input_border));
    let input_paragraph = Paragraph::new(Text::from(app.input.as_str()))
        .style(Style::default().fg(theme.input_text))
        .block(input_block)
        .wrap(Wrap { trim: false });
    f.render_widget(input_paragraph, chunks[1]);

    // cursor
    let cursor_pos = if app.input.is_empty() {
        0
    } else {
        app.input.len() as u16
    };
    let x = chunks[1].x + 1 + cursor_pos.min(chunks[1].width.saturating_sub(3));
    let y = chunks[1].y + 1 + app.input.matches('\n').count() as u16;
    f.set_cursor_position((x, y.min(chunks[1].y + chunks[1].height - 2)));

    // arrow
    let arrow = Line::from(Span::styled(
        "    \u{25B6}    ",
        Style::default()
            .fg(theme.arrow)
            .add_modifier(Modifier::BOLD),
    ));
    f.render_widget(arrow, chunks[2]);

    // output panel
    let output_title = if app.mode == Mode::Fwd {
        " Ealdsp\u{0113}c "
    } else {
        " English "
    };
    let output_block = Block::default()
        .title(output_title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.output_border));
    let output_paragraph = Paragraph::new(Text::from(app.translated.as_str()))
        .style(Style::default().fg(theme.output_text))
        .block(output_block)
        .wrap(Wrap { trim: false });
    f.render_widget(output_paragraph, chunks[3]);

    // flash message
    if let Some(ref flash) = app.flash {
        if app.frame < flash.expires {
            let flash_line = Line::from(Span::styled(
                flash.message.as_str(),
                Style::default()
                    .fg(flash.color)
                    .add_modifier(Modifier::BOLD),
            ));
            let flash_para = Paragraph::new(flash_line);
            let area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50)])
                .split(chunks[4])[0];
            f.render_widget(flash_para, area);
        } else {
            app.flash = None;
        }
    }

    // stats bar
    let stats = app.stats();
    let stats_line = Line::from(Span::styled(
        stats,
        Style::default().fg(theme.status),
    ));
    f.render_widget(stats_line, chunks[4]);

    // help bar
    let help = Line::from(Span::styled(
        " Ctrl+Q/Esc:quit  Tab:theme  Ctrl+R:reverse  Ctrl+A:about  Ctrl+C:copy  Ctrl+L:clear ",
        Style::default().fg(Color::DarkGray),
    ));
    f.render_widget(help, chunks[5]);
}
