use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Clear},
    Frame,
};

pub struct ExplorerItem {
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub depth: usize,
}

pub struct Explorer {
    pub visible: bool,
    pub current_dir: PathBuf,
    pub expanded_dirs: HashSet<PathBuf>,
    pub items: Vec<ExplorerItem>,
    pub selected_idx: usize,
}

fn build_tree(dir: &Path, depth: usize, expanded: &HashSet<PathBuf>, items: &mut Vec<ExplorerItem>) {
    if let Ok(entries) = fs::read_dir(dir) {
        let mut paths: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        paths.sort_by_key(|e| (!e.file_type().map(|f| f.is_dir()).unwrap_or(false), e.file_name()));
        
        for entry in paths {
            let path = entry.path();
            let is_dir = entry.file_type().map(|f| f.is_dir()).unwrap_or(false);
            
            items.push(ExplorerItem {
                path: path.clone(),
                is_dir,
                is_expanded: expanded.contains(&path),
                depth,
            });
            
            if is_dir && expanded.contains(&path) {
                build_tree(&path, depth + 1, expanded, items);
            }
        }
    }
}

impl Explorer {
    pub fn new(root: PathBuf) -> Self {
        let mut exp = Self {
            visible: false,
            current_dir: root,
            expanded_dirs: HashSet::new(),
            items: Vec::new(),
            selected_idx: 0,
        };
        exp.refresh();
        exp
    }

    pub fn refresh(&mut self) {
        self.items.clear();
        build_tree(&self.current_dir, 0, &self.expanded_dirs, &mut self.items);
        if self.selected_idx >= self.items.len() {
            self.selected_idx = self.items.len().saturating_sub(1);
        }
    }

    pub fn toggle_selected(&mut self) -> Option<PathBuf> {
        if self.items.is_empty() { return None; }
        
        let item = &self.items[self.selected_idx];
        if item.is_dir {
            if self.expanded_dirs.contains(&item.path) {
                self.expanded_dirs.remove(&item.path);
            } else {
                self.expanded_dirs.insert(item.path.clone());
            }
            self.refresh();
            None
        } else {
            Some(item.path.clone())
        }
    }
}

fn get_file_icon_and_color(path: &Path, is_dir: bool, is_expanded: bool) -> (&'static str, Color) {
    if is_dir {
        if is_expanded {
            ("", Color::LightBlue)
        } else {
            ("", Color::LightBlue)
        }
    } else {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        match ext.as_str() {
            "rs" => ("", Color::Red),
            "toml" => ("⚙", Color::DarkGray),
            "json" => ("", Color::Yellow),
            "lua" => ("", Color::LightBlue),
            "md" => ("", Color::LightCyan),
            "js" => ("", Color::Yellow),
            "ts" => ("", Color::LightBlue),
            "py" => ("", Color::Yellow),
            "sh" | "bash" | "ps1" => ("", Color::LightGreen),
            "html" | "xml" => ("", Color::LightRed),
            "css" => ("", Color::LightBlue),
            "git" | "gitignore" => ("", Color::LightRed),
            _ => ("", Color::White),
        }
    }
}

pub fn render(frame: &mut Frame, area: Rect, explorer: &mut Explorer) {
    if !explorer.visible { return; }

    let popup_area = Rect::new(
        area.width.saturating_sub(60) / 2,
        area.height.saturating_sub(20) / 2,
        60,
        20,
    );

    frame.render_widget(Clear, popup_area);

    let mut list_items = Vec::new();
    for (i, item) in explorer.items.iter().enumerate() {
        let prefix = "  ".repeat(item.depth);
        let (icon_str, mut icon_color) = get_file_icon_and_color(&item.path, item.is_dir, item.is_expanded);
        let mut file_name_color = if item.is_dir {
            Color::LightBlue
        } else {
            Color::White
        };

        let mut base_style = Style::default();
        if i == explorer.selected_idx {
            base_style = base_style.bg(Color::Blue).fg(Color::White);
            icon_color = Color::White;
            file_name_color = Color::White;
        }

        let file_name = item.path.file_name().unwrap_or_default().to_string_lossy();
        
        let mut spans = Vec::new();
        spans.push(Span::styled(prefix, base_style));
        spans.push(Span::styled(format!("{} ", icon_str), base_style.fg(icon_color)));
        spans.push(Span::styled(file_name.into_owned(), base_style.fg(file_name_color)));

        list_items.push(ListItem::new(Line::from(spans)));
    }

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(" File Explorer (Enter to open/expand, Esc to close) "));

    let mut state = ListState::default();
    state.select(Some(explorer.selected_idx));
    
    frame.render_stateful_widget(list, popup_area, &mut state);
}
