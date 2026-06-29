use anyhow::Result;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter as TsHighlighter};
use crate::plugins::api::PluginRegistry;
use std::sync::{Arc, Mutex};
use regex::Regex;

pub const HIGHLIGHT_NAMES: &[&str] = &[
    "keyword",
    "function",
    "type",
    "string",
    "comment",
    "variable",
    "constant",
    "number",
    "operator",
    "punctuation",
    "Keyword",
    "Special",
    "String",
    "LightRed"
];

pub struct Highlighter {
    ts_highlighter: Option<TsHighlighter>,
    config: Option<HighlightConfiguration>,
    pub spans: Vec<(usize, usize, usize)>,
    pub extension: String,
    pub plugin_registry: Option<Arc<Mutex<PluginRegistry>>>,
}

impl Highlighter {
    pub fn new(filepath: Option<&std::path::Path>, registry: Option<Arc<Mutex<PluginRegistry>>>) -> Result<Self> {
        let extension = filepath
            .and_then(|p| p.extension())
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();

        let mut config = None;
        let mut ts_highlighter = None;

        if extension == "rs" {
            let mut rust_config = HighlightConfiguration::new(
                tree_sitter_rust::language(),
                "rust",
                tree_sitter_rust::HIGHLIGHTS_QUERY,
                "",
                "",
            )?;
            rust_config.configure(HIGHLIGHT_NAMES);
            config = Some(rust_config);
            ts_highlighter = Some(TsHighlighter::new());
        }

        Ok(Self {
            ts_highlighter,
            config,
            spans: Vec::new(),
            extension,
            plugin_registry: registry,
        })
    }

    pub fn update(&mut self, text: &str) {
        self.spans.clear();

        if let (Some(ts), Some(config)) = (&mut self.ts_highlighter, &self.config) {
            if let Ok(highlights) = ts.highlight(config, text.as_bytes(), None, |_| None) {
                let mut current_style = None;
                for event in highlights {
                    if let Ok(event) = event {
                        match event {
                            HighlightEvent::HighlightStart(s) => current_style = Some(s.0),
                            HighlightEvent::Source { start, end } => {
                                if let Some(style) = current_style {
                                    self.spans.push((start, end, style));
                                }
                            }
                            HighlightEvent::HighlightEnd => current_style = None,
                        }
                    }
                }
            }
        }

        if let Some(registry_arc) = &self.plugin_registry {
            if let Ok(registry) = registry_arc.lock() {
                if let Some(rules) = registry.highlighters.get(&self.extension) {
                    for rule in rules {
                        if let Ok(re) = Regex::new(&rule.pattern) {
                            for cap in re.captures_iter(text) {
                                if let Some(m) = cap.get(0) {
                                    let style_index = match rule.color_name.to_lowercase().as_str() {
                                        "colors.keyword" | "keyword" | "magenta" => 0,
                                        "colors.type" | "type" | "blue" => 1,
                                        "colors.constant" | "constant" | "yellow" => 2,
                                        "colors.string" | "string" | "green" => 3,
                                        "colors.comment" | "comment" | "darkgray" | "gray" | "grey" => 4,
                                        "colors.variable" | "variable" | "cyan" => 5,
                                        "colors.operator" | "operator" | "red" => 6,
                                        "colors.lightred" | "lightred" => 7,
                                        "colors.lightmagenta" | "lightmagenta" => 8,
                                        "colors.white" | "white" => 9,
                                        "special" | "lightyellow" => 11,
                                        "function" | "lightblue" => 15,
                                        _ => 0, 
                                    };
                                    self.spans.push((m.start(), m.end(), style_index));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
