use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::App;
use std::fs;

pub fn render(f: &mut Frame, _area: Rect, app: &App) {
    let popup_area = ratatui::layout::Rect::new(
        f.size().width.saturating_sub(60) / 2,
        f.size().height.saturating_sub(20) / 2,
        60,
        20,
    );
    f.render_widget(ratatui::widgets::Clear, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(popup_area);

    let title = Paragraph::new(" Extensions Manager \n Press 'Enter' to Toggle. Press 'r' to Reload. 'Esc' to close.")
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Yellow).bg(Color::DarkGray));
    f.render_widget(title, chunks[0]);

    let mut items = Vec::new();
    
    let mut exts = Vec::new();
    let ext_dir = crate::plugins::engine::get_extensions_dir();


    if let Ok(entries) = fs::read_dir(&ext_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    exts.push(name.to_string());
                }
            }
        }
    }
    exts.sort();

    for (i, name) in exts.iter().enumerate() {
        let is_disabled = app.settings.disabled_extensions.contains(name);
        let checkbox = if is_disabled { "[ ]" } else { "[X]" };
        
        let style = if i == app.extensions_focus_idx {
            Style::default().bg(Color::Cyan).fg(Color::Black)
        } else {
            if is_disabled {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White)
            }
        };
        items.push(ListItem::new(format!(" {} {} ", checkbox, name)).style(style));
    }

    if exts.is_empty() {
        items.push(ListItem::new(" No extensions found.").style(Style::default().fg(Color::DarkGray)));
    }

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(" Installed Plugins "));
    f.render_widget(list, chunks[1]);
}
