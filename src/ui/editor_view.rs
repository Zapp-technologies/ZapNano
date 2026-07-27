use crate::editor::Editor;
use crate::settings::ThemeColors;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, editor: &Editor, scroll_offset: usize, theme: &ThemeColors) {
    let mut lines = Vec::new();
    let text = editor.buffer.text.to_string();
    
    let mut styles = vec![Style::default().fg(theme.fg); text.len()];
    for &(start, end, style_idx) in &editor.highlighter.spans {
        let style = match style_idx {
            0 => Style::default().fg(Color::Magenta),
            1 => Style::default().fg(Color::Blue),
            2 => Style::default().fg(Color::Yellow),
            3 => Style::default().fg(Color::Green),
            4 => Style::default().fg(Color::DarkGray),
            5 => Style::default().fg(Color::Cyan),
            6 => Style::default().fg(Color::Red),
            7 => Style::default().fg(Color::LightRed),
            8 => Style::default().fg(Color::LightMagenta),
            9 => Style::default().fg(Color::White),
            10 => Style::default().fg(Color::Magenta),     
            11 => Style::default().fg(Color::LightYellow), 
            12 => Style::default().fg(Color::Green),       
            13 => Style::default().fg(Color::LightRed),    
            14 => Style::default().fg(Color::DarkGray),    
            15 => Style::default().fg(Color::LightBlue),   
            16 => Style::default().fg(Color::Cyan),       
            17 => Style::default().fg(Color::Yellow),      
            _ => Style::default().fg(theme.fg),
        };
        for i in start..end {
            if i < styles.len() {
                styles[i] = style;
            }
        }
    }
    
    let start_line = scroll_offset;
    let end_line = std::cmp::min(scroll_offset + area.height as usize, editor.buffer.len_lines());
    
    for i in start_line..end_line {
        let line = editor.buffer.text.line(i);
        let mut spans = Vec::new();
        let line_num_str = format!("{:4} ", i + 1);
        spans.push(Span::styled(line_num_str, Style::default().fg(theme.line_nr_fg).bg(theme.bg)));
        
        let start_char_idx = editor.buffer.line_to_char(i);
        let mut current_style = Style::default().bg(theme.bg);
        let mut current_str = String::new();
        
        let mut byte_offset = editor.buffer.text.char_to_byte(start_char_idx);
        let mut current_char_idx = start_char_idx;
        let selection_bounds = editor.cursor.get_selection_char_bounds(&editor.buffer);

        for ch in line.chars() {
            if ch == '\n' || ch == '\r' { 
                byte_offset += ch.len_utf8();
                current_char_idx += 1;
                continue; 
            }
            let mut s = if byte_offset < styles.len() { styles[byte_offset] } else { Style::default().fg(theme.fg) };
            s = s.bg(theme.bg);
            
            if let Some((sel_start, sel_end)) = selection_bounds {
                if current_char_idx >= sel_start && current_char_idx < sel_end {
                    s = s.bg(theme.selection_bg);
                }
            }
            
            if s != current_style && !current_str.is_empty() {
                spans.push(Span::styled(current_str.clone(), current_style));
                current_str.clear();
            }
            current_style = s;
            if ch == '\t' {
                current_str.push_str("    ");
            } else {
                current_str.push(ch);
            }
            byte_offset += ch.len_utf8();
            current_char_idx += 1;
        }
        
        if !current_str.is_empty() {
            spans.push(Span::styled(current_str, current_style));
        }
        
        lines.push(Line::from(spans));
    }
    
    frame.render_widget(Paragraph::new(lines).style(Style::default().bg(theme.bg)), area);
}
