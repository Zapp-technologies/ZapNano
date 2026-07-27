use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect) {
    let popup_area = ratatui::layout::Rect::new(
        area.width.saturating_sub(60) / 2,
        area.height.saturating_sub(18) / 2,
        60,
        18,
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

    let title = Paragraph::new(" Command Reference (!. <cmd>) ")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Yellow)));
        
    f.render_widget(title, chunks[0]);

    let mut items = Vec::new();
    
    items.push(ListItem::new(" s          - Save Current File").style(Style::default().fg(Color::White)));
    items.push(ListItem::new(" saveas <p> - Save As new path").style(Style::default().fg(Color::White)));
    items.push(ListItem::new(" q          - Quit Editor").style(Style::default().fg(Color::White)));
    items.push(ListItem::new(" qnos       - Force Quit (No Save)").style(Style::default().fg(Color::White)));
    items.push(ListItem::new(" s&q        - Save and Quit").style(Style::default().fg(Color::White)));
    items.push(ListItem::new(" close / c  - Close active tab").style(Style::default().fg(Color::White)));
    items.push(ListItem::new(" rename <p> - Rename File").style(Style::default().fg(Color::White)));
    items.push(ListItem::new(" settings   - Open settings menu").style(Style::default().fg(Color::White)));
    items.push(ListItem::new(" ext        - Open extensions manager").style(Style::default().fg(Color::White)));
    items.push(ListItem::new(" exp / ls   - Open file explorer").style(Style::default().fg(Color::White)));
    items.push(ListItem::new(" menu       - Open welcome menu").style(Style::default().fg(Color::White)));
    items.push(ListItem::new(" new        - Open new empty tab").style(Style::default().fg(Color::Yellow)));
    items.push(ListItem::new(" mdir <p>   - Create directory").style(Style::default().fg(Color::Yellow)));
    items.push(ListItem::new(" cd <p>     - Change directory").style(Style::default().fg(Color::Yellow)));
    
    let list = List::new(items).block(Block::default().borders(Borders::LEFT | Borders::RIGHT));
    f.render_widget(list, chunks[1]);
    
    let footer = Paragraph::new(" Esc: Close Overlay ")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::DarkGray)));
        
    f.render_widget(footer, chunks[2]);
}
