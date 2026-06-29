use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
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
        let icon = if item.is_dir {
            if item.is_expanded { "v " } else { "> " }
        } else {
            "  "
        };
        
        let file_name = item.path.file_name().unwrap_or_default().to_string_lossy();
        let display_text = format!("{}{}{}", prefix, icon, file_name);

        let style = if i == explorer.selected_idx {
            Style::default().bg(Color::Blue).fg(Color::White)
        } else if item.is_dir {
            Style::default().fg(Color::LightBlue)
        } else {
            Style::default().fg(Color::White)
        };

        list_items.push(ListItem::new(display_text).style(style));
    }

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(" File Explorer (Enter to open/expand, Esc to close) "));

    let mut state = ListState::default();
    state.select(Some(explorer.selected_idx));
    
    frame.render_stateful_widget(list, popup_area, &mut state);
}
