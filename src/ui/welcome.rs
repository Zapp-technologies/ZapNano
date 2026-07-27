use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(10), // Logo
            Constraint::Min(5),     // Recent files / Options
        ])
        .split(area);
        
    let logo_text = vec![
        ratatui::text::Line::from(r#"  ______               _   _                     "#),
        ratatui::text::Line::from(r#" |__  /               | \ | |                    "#),
        ratatui::text::Line::from(r#"    / /  __ _  _ __   |  \| |  __ _  _ __    ___ "#),
        ratatui::text::Line::from(r#"   / /  / _` || '_ \  | . ` | / _` || '_ \  / _ \"#),
        ratatui::text::Line::from(r#"  / /__| (_| || |_) | | |\  || (_| || | | || (_) |"#),
        ratatui::text::Line::from(r#" /_____|\__,_|| .__/  |_| \_| \__,_||_| |_| \___/ "#),
        ratatui::text::Line::from(r#"              | |                                "#),
        ratatui::text::Line::from(r#"              |_|                                "#),
        ratatui::text::Line::from(r#""#),
    ];

    let logo = Paragraph::new(logo_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(logo, chunks[1]);

    let menu_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(chunks[2]);

    let mut items = Vec::new();
    
    items.push(ListItem::new(" === Recent Files ===").style(Style::default().fg(Color::DarkGray)));
    
    let mut selectable_lines = 1;
    
    for file in &app.settings.recent_files {
        let style = if app.welcome_idx == selectable_lines {
            Style::default().bg(Color::Blue).fg(Color::White)
        } else {
            Style::default().fg(Color::White)
        };
        items.push(ListItem::new(format!(" [{}] {}", selectable_lines, file)).style(style));
        selectable_lines += 1;
    }
    
    if app.settings.recent_files.is_empty() {
        items.push(ListItem::new("  (No recent files)").style(Style::default().fg(Color::DarkGray)));
    }
    
    items.push(ListItem::new(""));
    items.push(ListItem::new(" === Actions ===").style(Style::default().fg(Color::DarkGray)));
    let action_new = ListItem::new(" [N] New File").style(if app.welcome_idx == selectable_lines { Style::default().bg(Color::Blue).fg(Color::White) } else { Style::default().fg(Color::White) });
    selectable_lines += 1;
    let action_dir = ListItem::new(" [D] Explore Workspace").style(if app.welcome_idx == selectable_lines { Style::default().bg(Color::Blue).fg(Color::White) } else { Style::default().fg(Color::White) });
    selectable_lines += 1;
    let action_quit = ListItem::new(" [Q] Quit Editor").style(if app.welcome_idx == selectable_lines { Style::default().bg(Color::Blue).fg(Color::White) } else { Style::default().fg(Color::White) });
    selectable_lines += 1;
    let action_cmds = ListItem::new(" [C] Command Reference").style(if app.welcome_idx == selectable_lines { Style::default().bg(Color::Blue).fg(Color::White) } else { Style::default().fg(Color::White) });
    selectable_lines += 1;
    let action_settings = ListItem::new(" [S] Settings").style(if app.welcome_idx == selectable_lines { Style::default().bg(Color::Blue).fg(Color::White) } else { Style::default().fg(Color::White) });
    selectable_lines += 1;
    let action_ext = ListItem::new(" [E] Extensions").style(if app.welcome_idx == selectable_lines { Style::default().bg(Color::Blue).fg(Color::White) } else { Style::default().fg(Color::White) });
    
    items.push(action_new);
    items.push(action_dir);
    items.push(action_quit);
    items.push(action_cmds);
    items.push(action_settings);
    items.push(action_ext);

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(" Menu "));
    f.render_widget(list, menu_chunks[1]);
}
