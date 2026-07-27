use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use ratatui::style::Color;
use crossterm::event::{KeyCode, KeyModifiers};

pub fn get_available_themes() -> Vec<String> {
    let mut themes = vec![
        "Default".to_string(), "Dracula".to_string(), "Oceanic".to_string(),
        "Light".to_string(), "Monokai".to_string(), "Nord".to_string(),
        "Gruvbox".to_string(), "SolarizedDark".to_string()
    ];
    if let Some(config_dir) = dirs::config_dir() {
        let themes_dir = config_dir.join("zapnano").join("themes");
        if let Ok(entries) = fs::read_dir(themes_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        themes.push(name.to_string());
                    }
                }
            }
        }
    }
    themes
}

fn parse_color(c: &str) -> Color {
    if c.starts_with('#') && c.len() == 7 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&c[1..3], 16),
            u8::from_str_radix(&c[3..5], 16),
            u8::from_str_radix(&c[5..7], 16),
        ) {
            return Color::Rgb(r, g, b);
        }
    }
    match c.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" | "darkgray" => Color::DarkGray,
        "lightred" => Color::LightRed,
        "lightgreen" => Color::LightGreen,
        "lightyellow" => Color::LightYellow,
        "lightblue" => Color::LightBlue,
        "lightmagenta" => Color::LightMagenta,
        "lightcyan" => Color::LightCyan,
        "white" | _ => Color::White,
    }
}

#[derive(Deserialize)]
struct ThemeDefinition {
    bg: Option<String>,
    fg: Option<String>,
    tab_bar_bg: Option<String>,
    tab_active_bg: Option<String>,
    tab_active_fg: Option<String>,
    tab_inactive_fg: Option<String>,
    prompt_bg: Option<String>,
    prompt_fg: Option<String>,
    status_bg: Option<String>,
    status_fg: Option<String>,
    selection_bg: Option<String>,
    line_nr_fg: Option<String>,
}

#[derive(Clone)]
pub struct ThemeColors {
    pub bg: Color,
    pub fg: Color,
    pub tab_bar_bg: Color,
    pub tab_active_bg: Color,
    pub tab_active_fg: Color,
    pub tab_inactive_fg: Color,
    pub prompt_bg: Color,
    pub prompt_fg: Color,
    pub status_bg: Color,
    pub status_fg: Color,
    pub selection_bg: Color,
    pub line_nr_fg: Color,
}

impl ThemeColors {
    pub fn get(theme_name: Option<&str>) -> Self {
        let name = theme_name.unwrap_or("Default");
        if let Some(config_dir) = dirs::config_dir() {
            let path = config_dir.join("zapnano").join("themes").join(format!("{}.toml", name));
            if let Ok(contents) = fs::read_to_string(&path) {
                if let Ok(def) = toml::from_str::<ThemeDefinition>(&contents) {
                    return ThemeColors {
                        bg: def.bg.as_deref().map(parse_color).unwrap_or(Color::Reset),
                        fg: def.fg.as_deref().map(parse_color).unwrap_or(Color::Reset),
                        tab_bar_bg: def.tab_bar_bg.as_deref().map(parse_color).unwrap_or(Color::Black),
                        tab_active_bg: def.tab_active_bg.as_deref().map(parse_color).unwrap_or(Color::Blue),
                        tab_active_fg: def.tab_active_fg.as_deref().map(parse_color).unwrap_or(Color::White),
                        tab_inactive_fg: def.tab_inactive_fg.as_deref().map(parse_color).unwrap_or(Color::DarkGray),
                        prompt_bg: def.prompt_bg.as_deref().map(parse_color).unwrap_or(Color::Blue),
                        prompt_fg: def.prompt_fg.as_deref().map(parse_color).unwrap_or(Color::White),
                        status_bg: def.status_bg.as_deref().map(parse_color).unwrap_or(Color::White),
                        status_fg: def.status_fg.as_deref().map(parse_color).unwrap_or(Color::Black),
                        selection_bg: def.selection_bg.as_deref().map(parse_color).unwrap_or(Color::DarkGray),
                        line_nr_fg: def.line_nr_fg.as_deref().map(parse_color).unwrap_or(Color::DarkGray),
                    };
                }
            }
        }

        ThemeColors {
            bg: Color::Reset,
            fg: Color::Reset,
            tab_bar_bg: Color::Black,
            tab_active_bg: Color::Blue,
            tab_active_fg: Color::White,
            tab_inactive_fg: Color::DarkGray,
            prompt_bg: Color::Blue,
            prompt_fg: Color::White,
            status_bg: Color::White,
            status_fg: Color::Black,
            selection_bg: Color::DarkGray,
            line_nr_fg: Color::DarkGray,
        }
    }
}

pub fn install_builtin_themes(themes_dir: &std::path::Path) {
    let builtins = vec![
        ("Dracula", "bg = \"#282a36\"\nfg = \"#f8f8f2\"\ntab_bar_bg = \"#21222c\"\ntab_active_bg = \"#6272a4\"\ntab_active_fg = \"white\"\ntab_inactive_fg = \"#6272a4\"\nprompt_bg = \"#6272a4\"\nprompt_fg = \"white\"\nstatus_bg = \"#44475a\"\nstatus_fg = \"white\"\nselection_bg = \"#44475a\"\nline_nr_fg = \"#6272a4\""),
        ("Oceanic", "bg = \"#1b2b34\"\nfg = \"#d8dee9\"\ntab_bar_bg = \"#16242c\"\ntab_active_bg = \"#6699cc\"\ntab_active_fg = \"white\"\ntab_inactive_fg = \"#65737e\"\nprompt_bg = \"#6699cc\"\nprompt_fg = \"white\"\nstatus_bg = \"#65737e\"\nstatus_fg = \"white\"\nselection_bg = \"#4f5b66\"\nline_nr_fg = \"#65737e\""),
        ("Light", "bg = \"#fafafa\"\nfg = \"#1e1e1e\"\ntab_bar_bg = \"#e6e6e6\"\ntab_active_bg = \"#2965cc\"\ntab_active_fg = \"white\"\ntab_inactive_fg = \"#646464\"\nprompt_bg = \"#2965cc\"\nprompt_fg = \"white\"\nstatus_bg = \"#c8c8c8\"\nstatus_fg = \"black\"\nselection_bg = \"#d2e6ff\"\nline_nr_fg = \"#969696\""),
        ("Monokai", "bg = \"#272822\"\nfg = \"#f8f8f2\"\ntab_bar_bg = \"#1e1f1c\"\ntab_active_bg = \"#66d9ef\"\ntab_active_fg = \"black\"\ntab_inactive_fg = \"#75715e\"\nprompt_bg = \"#66d9ef\"\nprompt_fg = \"black\"\nstatus_bg = \"#75715e\"\nstatus_fg = \"white\"\nselection_bg = \"#49483e\"\nline_nr_fg = \"#75715e\""),
        ("Nord", "bg = \"#2e3440\"\nfg = \"#d8dee9\"\ntab_bar_bg = \"#3b4252\"\ntab_active_bg = \"#88c0d0\"\ntab_active_fg = \"black\"\ntab_inactive_fg = \"#4c566a\"\nprompt_bg = \"#88c0d0\"\nprompt_fg = \"black\"\nstatus_bg = \"#434c5e\"\nstatus_fg = \"white\"\nselection_bg = \"#434c5e\"\nline_nr_fg = \"#4c566a\""),
        ("Gruvbox", "bg = \"#282828\"\nfg = \"#ebdbb2\"\ntab_bar_bg = \"#1d2021\"\ntab_active_bg = \"#d79921\"\ntab_active_fg = \"black\"\ntab_inactive_fg = \"#928374\"\nprompt_bg = \"#d79921\"\nprompt_fg = \"black\"\nstatus_bg = \"#504945\"\nstatus_fg = \"white\"\nselection_bg = \"#665c54\"\nline_nr_fg = \"#928374\""),
        ("SolarizedDark", "bg = \"#002b36\"\nfg = \"#839496\"\ntab_bar_bg = \"#073642\"\ntab_active_bg = \"#268bd2\"\ntab_active_fg = \"white\"\ntab_inactive_fg = \"#586e75\"\nprompt_bg = \"#268bd2\"\nprompt_fg = \"white\"\nstatus_bg = \"#073642\"\nstatus_fg = \"white\"\nselection_bg = \"#073642\"\nline_nr_fg = \"#586e75\""),
    ];
    for (name, content) in builtins {
        let path = themes_dir.join(format!("{}.toml", name));
        if !path.exists() {
            let _ = fs::write(&path, content);
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BracketPair {
    pub open: String,
    pub close: String,
    #[serde(default)]
    pub indent_size: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IndentPairConfig {
    #[serde(default = "default_indent_pairs")]
    pub pairs: Vec<BracketPair>,
}

impl Default for IndentPairConfig {
    fn default() -> Self {
        Self {
            pairs: default_indent_pairs(),
        }
    }
}

fn default_indent_pairs() -> Vec<BracketPair> {
    vec![
        BracketPair { open: "{".to_string(), close: "}".to_string(), indent_size: None },
        BracketPair { open: "(".to_string(), close: ")".to_string(), indent_size: None },
        BracketPair { open: "[".to_string(), close: "]".to_string(), indent_size: None },
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyCombination {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl Serialize for KeyCombination {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut parts = Vec::new();
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            parts.push("ctrl");
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            parts.push("alt");
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            parts.push("shift");
        }
        let code_str = match self.code {
            KeyCode::Left => "left".to_string(),
            KeyCode::Right => "right".to_string(),
            KeyCode::Up => "up".to_string(),
            KeyCode::Down => "down".to_string(),
            KeyCode::Esc => "esc".to_string(),
            KeyCode::Enter => "enter".to_string(),
            KeyCode::Tab => "tab".to_string(),
            KeyCode::Backspace => "backspace".to_string(),
            KeyCode::Delete => "delete".to_string(),
            KeyCode::Char(c) => c.to_string(),
            _ => "unknown".to_string(),
        };
        parts.push(&code_str);
        serializer.serialize_str(&parts.join("+"))
    }
}

impl<'de> Deserialize<'de> for KeyCombination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        KeyCombination::parse(&s).ok_or_else(|| serde::de::Error::custom(format!("invalid key combination: {}", s)))
    }
}

impl KeyCombination {
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<String> = s.split('+').map(|p| p.trim().to_lowercase()).collect();
        let mut modifiers = KeyModifiers::empty();
        let mut code = None;

        for part in &parts {
            match part.as_str() {
                "ctrl" | "control" => modifiers.insert(KeyModifiers::CONTROL),
                "alt" | "option" => modifiers.insert(KeyModifiers::ALT),
                "shift" => modifiers.insert(KeyModifiers::SHIFT),
                "left" => code = Some(KeyCode::Left),
                "right" => code = Some(KeyCode::Right),
                "up" => code = Some(KeyCode::Up),
                "down" => code = Some(KeyCode::Down),
                "esc" | "escape" => code = Some(KeyCode::Esc),
                "enter" | "return" => code = Some(KeyCode::Enter),
                "tab" => code = Some(KeyCode::Tab),
                "backspace" => code = Some(KeyCode::Backspace),
                "delete" | "del" => code = Some(KeyCode::Delete),
                s if s.len() == 1 => {
                    let c = s.chars().next()?;
                    code = Some(KeyCode::Char(c));
                }
                _ => return None,
            }
        }

        code.map(|c| KeyCombination { code: c, modifiers })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KeybindingsSettings {
    #[serde(default = "default_prev_tab")]
    pub prev_tab: KeyCombination,
    #[serde(default = "default_next_tab")]
    pub next_tab: KeyCombination,
    #[serde(default = "default_save")]
    pub save: KeyCombination,
    #[serde(default = "default_quit")]
    pub quit: KeyCombination,
    #[serde(default = "default_search")]
    pub search: KeyCombination,
    #[serde(default = "default_select_all")]
    pub select_all: KeyCombination,
    #[serde(default = "default_undo")]
    pub undo: KeyCombination,
    #[serde(default = "default_redo")]
    pub redo: KeyCombination,
    #[serde(default = "default_copy")]
    pub copy: KeyCombination,
    #[serde(default = "default_paste")]
    pub paste: KeyCombination,
}

impl Default for KeybindingsSettings {
    fn default() -> Self {
        Self {
            prev_tab: default_prev_tab(),
            next_tab: default_next_tab(),
            save: default_save(),
            quit: default_quit(),
            search: default_search(),
            select_all: default_select_all(),
            undo: default_undo(),
            redo: default_redo(),
            copy: default_copy(),
            paste: default_paste(),
        }
    }
}

fn default_prev_tab() -> KeyCombination { KeyCombination { code: KeyCode::Left, modifiers: KeyModifiers::ALT } }
fn default_next_tab() -> KeyCombination { KeyCombination { code: KeyCode::Right, modifiers: KeyModifiers::ALT } }
fn default_save() -> KeyCombination { KeyCombination { code: KeyCode::Char('s'), modifiers: KeyModifiers::CONTROL } }
fn default_quit() -> KeyCombination { KeyCombination { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL } }
fn default_search() -> KeyCombination { KeyCombination { code: KeyCode::Char('f'), modifiers: KeyModifiers::CONTROL } }
fn default_select_all() -> KeyCombination { KeyCombination { code: KeyCode::Char('a'), modifiers: KeyModifiers::CONTROL } }
fn default_undo() -> KeyCombination { KeyCombination { code: KeyCode::Char('z'), modifiers: KeyModifiers::CONTROL } }
fn default_redo() -> KeyCombination { KeyCombination { code: KeyCode::Char('y'), modifiers: KeyModifiers::CONTROL } }
fn default_copy() -> KeyCombination { KeyCombination { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL } }
fn default_paste() -> KeyCombination { KeyCombination { code: KeyCode::Char('v'), modifiers: KeyModifiers::CONTROL } }

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub recent_files: Vec<String>,
    pub theme: Option<String>,
    #[serde(default = "default_tab_size")]
    pub tab_size: usize,
    #[serde(default)]
    pub disabled_extensions: Vec<String>,
    #[serde(default)]
    pub disable_autocomplete: bool,
    #[serde(default)]
    pub indent: IndentPairConfig,
    #[serde(default)]
    pub keybindings: KeybindingsSettings,
    #[serde(default)]
    pub disable_paste_grouping: bool,
}

fn default_tab_size() -> usize { 4 }

impl Default for Settings {
    fn default() -> Self {
        Self {
            recent_files: Vec::new(),
            theme: None,
            tab_size: 4,
            disabled_extensions: Vec::new(),
            disable_autocomplete: false,
            indent: IndentPairConfig::default(),
            keybindings: KeybindingsSettings::default(),
            disable_paste_grouping: false,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        let zapnano_dir = config_dir.join("zapnano");
        let themes_dir = zapnano_dir.join("themes");
        let _ = fs::create_dir_all(&themes_dir);
        
        install_builtin_themes(&themes_dir);

        let settings_path = zapnano_dir.join("settings.toml");
        
        if let Ok(contents) = fs::read_to_string(&settings_path) {
            if let Ok(settings) = toml::from_str(&contents) {
                return settings;
            }
        }
        Settings::default()
    }

    pub fn save(&self) {
        if let Some(config_dir) = dirs::config_dir() {
            let zapnano_dir = config_dir.join("zapnano");
            let _ = fs::create_dir_all(&zapnano_dir);
            let _ = fs::create_dir_all(zapnano_dir.join("themes"));
            let settings_path = zapnano_dir.join("settings.toml");
            
            if let Ok(contents) = toml::to_string(self) {
                let _ = fs::write(&settings_path, contents);
            }
        }
    }

    pub fn add_recent(&mut self, file: &str) {
        self.recent_files.retain(|f| f != file);
        self.recent_files.insert(0, file.to_string());
        self.recent_files.truncate(8); 
        self.save();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_combination_parse() {
        let keys = vec![
            ("ctrl+s", KeyCombination { code: KeyCode::Char('s'), modifiers: KeyModifiers::CONTROL }),
            ("alt+left", KeyCombination { code: KeyCode::Left, modifiers: KeyModifiers::ALT }),
            ("ctrl+alt+shift+right", KeyCombination { code: KeyCode::Right, modifiers: KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SHIFT }),
        ];

        for (s, expected) in keys {
            let parsed = KeyCombination::parse(s).unwrap();
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn test_settings_default() {
        let settings = Settings::default();
        assert_eq!(settings.tab_size, 4);
        assert_eq!(settings.keybindings.save.code, KeyCode::Char('s'));
    }
}
