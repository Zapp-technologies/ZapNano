use crate::editor::Editor;
use crate::settings::ThemeColors;
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::Paragraph,
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, editor: &Editor, theme: &ThemeColors) {
    let filename = if let Some(p) = &editor.buffer.filepath {
        p.display().to_string()
    } else {
        "[No Name]".to_string()
    };
    let modified = if editor.buffer.modified { "[+]" } else { "" };
    
    let left_text = format!(" {} {} ", filename, modified);
    let right_text = format!(" {}:{} ", editor.cursor.line_idx + 1, editor.cursor.char_idx + 1);
    
    let style = Style::default().bg(theme.status_bg).fg(theme.status_fg);
    
    let empty_space = area.width.saturating_sub(left_text.len() as u16 + right_text.len() as u16);
    let spaces = " ".repeat(empty_space as usize);
    let status_str = format!("{}{}{}", left_text, spaces, right_text);
    
    frame.render_widget(Paragraph::new(status_str).style(style), area);
}
