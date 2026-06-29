use crate::app::{App, PromptType, QuitContext};
use crate::ui::{editor_view, status_bar};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1), Constraint::Length(1)])
        .split(f.size());

    let theme = crate::settings::ThemeColors::get(app.settings.theme.as_deref());

    let mut tab_spans = Vec::new();
    for (i, ed) in app.editors.iter().enumerate() {
        let title = match &ed.buffer.filepath {
            Some(p) => p.file_name().unwrap_or_default().to_string_lossy().to_string(),
            None => String::from("[No Name]"),
        };
        let mod_indicator = if ed.buffer.modified { "*" } else { "" };
        let style = if i == app.active_editor {
            Style::default().bg(theme.tab_active_bg).fg(theme.tab_active_fg)
        } else {
            Style::default().bg(theme.tab_bar_bg).fg(theme.tab_inactive_fg)
        };
        tab_spans.push(ratatui::text::Span::styled(format!(" {} {}{} |", i + 1, title, mod_indicator), style));
    }
    
    f.render_widget(Paragraph::new(ratatui::text::Line::from(tab_spans)).style(Style::default().bg(theme.tab_bar_bg)), chunks[0]);

    editor_view::render(f, chunks[1], &app.editors[app.active_editor], app.scroll_offset, &theme);
    
    if app.prompt != PromptType::None {
        let prefix = match app.prompt {
            PromptType::Command => " !. ",
            PromptType::SaveAs => " Save As: ",
            PromptType::Search => " Find: ",
            _ => "",
        };
        let cmd_str = format!("{}{}", prefix, app.prompt_input);
        let cmd_str_padded = format!("{:<width$}", cmd_str, width = chunks[2].width as usize);
        f.render_widget(Paragraph::new(cmd_str_padded).style(Style::default().bg(theme.prompt_bg).fg(theme.prompt_fg)), chunks[2]);
    } else {
        status_bar::render(f, chunks[2], &app.editors[app.active_editor], &theme);
    }

    if app.show_commands {
        crate::ui::commands_view::render(f, f.size());
    } else if app.show_settings {
        crate::ui::settings_view::render(f, f.size(), app);
    } else if app.show_extensions {
        crate::ui::extensions_view::render(f, f.size(), app);
    } else if app.explorer.visible {
        crate::ui::explorer::render(f, f.size(), &mut app.explorer);
    } else if app.show_welcome {
        crate::ui::welcome::render(f, f.size(), app);
    } else if app.quit_context != QuitContext::None {
        let popup_area = ratatui::layout::Rect::new(
            f.size().width.saturating_sub(90) / 2,
            f.size().height.saturating_sub(3) / 2,
            90,
            3,
        );
        f.render_widget(ratatui::widgets::Clear, popup_area);
        
        let msg = match app.quit_context {
            QuitContext::CloseTab(_) => " Tab possesses unsaved changes! Press 'y' to force close, any other key to cancel. ",
            _ => " Unsaved changes! Press 'y' to force quit, any other key to cancel. ",
        };
        f.render_widget(
            Paragraph::new(msg)
                .style(Style::default().bg(theme.prompt_bg).fg(Color::White)),
            popup_area,
        );
    } else if app.autocomplete.visible && !app.autocomplete.options.is_empty() {
        let active = app.active_editor;
        let cursor_relative_y = (app.editors[active].cursor.line_idx.saturating_sub(app.scroll_offset)) as u16;
        let cursor_x = 5 + app.editors[active].cursor.char_idx as u16;
        
        let popup_y = if cursor_relative_y + 7 < chunks[1].height {
            cursor_relative_y + 1
        } else {
            cursor_relative_y.saturating_sub(6)
        };
        
        let list_height = (app.autocomplete.options.len() as u16 + 2).min(6);
        let popup_area = ratatui::layout::Rect::new(
            (chunks[1].x + cursor_x).min(chunks[1].width.saturating_sub(40)),
            chunks[1].y + popup_y,
            40,
            list_height,
        );
        
        f.render_widget(ratatui::widgets::Clear, popup_area);
        
        let mut items = Vec::new();
        let start_idx = if app.autocomplete.selected_idx >= 4 { app.autocomplete.selected_idx - 3 } else { 0 };
        for (i, opt) in app.autocomplete.options.iter().enumerate().skip(start_idx).take(4) {
            let style = if i == app.autocomplete.selected_idx {
                Style::default().bg(Color::Cyan).fg(Color::Black)
            } else {
                Style::default().fg(Color::White)
            };
            items.push(ratatui::widgets::ListItem::new(format!(" {} - {}", opt.prefix, opt.description)).style(style));
        }
        
        let list = ratatui::widgets::List::new(items)
            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).title(" Snippets "))
            .style(Style::default().bg(Color::Blue));
        
        f.render_widget(list, popup_area);
        
        let active = app.active_editor;
        let cursor_y = (app.editors[active].cursor.line_idx.saturating_sub(app.scroll_offset)) as u16;
        f.set_cursor(chunks[1].x + cursor_x, chunks[1].y + cursor_y);
        
    } else if app.prompt != PromptType::None {
        let prefix_len = match app.prompt {
            PromptType::Command => 4,
            PromptType::SaveAs => 10,
            PromptType::Search => 7,
            _ => 0,
        };
        let cursor_x = prefix_len + app.prompt_input.len() as u16;
        f.set_cursor(chunks[2].x + cursor_x, chunks[2].y);
    } else {
        let active = app.active_editor;
        let cursor_y = (app.editors[active].cursor.line_idx.saturating_sub(app.scroll_offset)) as u16;

        let line = app.editors[active].buffer.text.line(app.editors[active].cursor.line_idx);
        let mut cursor_x = 5;
        for (i, _) in line.chars().enumerate() {
            if i == app.editors[active].cursor.char_idx {
                break;
            }
            cursor_x += 1;
        }

        f.set_cursor(chunks[1].x + cursor_x, chunks[1].y + cursor_y);
    }
}
