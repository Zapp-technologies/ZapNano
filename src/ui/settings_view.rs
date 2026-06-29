use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
    Frame,
};
use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let popup_area = ratatui::layout::Rect::new(
        area.width.saturating_sub(60) / 2,
        area.height.saturating_sub(15) / 2,
        60,
        15,
    );
    
    f.render_widget(Clear, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), 
            Constraint::Min(4),   
            Constraint::Length(3), 
        ])
        .split(popup_area);

    let title = Paragraph::new(" Zap Nano Configuration ")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Cyan)));
        
    f.render_widget(title, chunks[0]);

    let mut items = Vec::new();
    
    let tab_str = format!(" Tab Size: < {} >", app.settings.tab_size);
    items.push(ListItem::new(tab_str).style(if app.settings_focus_idx == 0 { Style::default().bg(Color::Blue).fg(Color::White) } else { Style::default().fg(Color::White) }));
    
    let theme_str = format!(" Theme: < {} >", app.settings.theme.as_deref().unwrap_or("Default"));
    items.push(ListItem::new(theme_str).style(if app.settings_focus_idx == 1 { Style::default().bg(Color::Blue).fg(Color::White) } else { Style::default().fg(Color::White) }));

    let auto_str = format!(" Autocomplete: < {} >", if app.settings.disable_autocomplete { "Disabled" } else { "Enabled" });
    items.push(ListItem::new(auto_str).style(if app.settings_focus_idx == 2 { Style::default().bg(Color::Blue).fg(Color::White) } else { Style::default().fg(Color::White) }));

    let list = List::new(items).block(Block::default().borders(Borders::LEFT | Borders::RIGHT));
    f.render_widget(list, chunks[1]);
    
    let footer = Paragraph::new(" Up/Down: Select | Left/Right: Adjust | Esc: Save & Close ")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::DarkGray)));
        
    f.render_widget(footer, chunks[2]);
}
