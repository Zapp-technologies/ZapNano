use mlua::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct HighlightRule {
    pub pattern: String,
    pub color_name: String,
}

#[derive(Clone, Debug)]
pub struct SnippetRule {
    pub prefix: String,
    pub body: String,
    pub description: String,
}

#[derive(Default, Clone)]
pub struct PluginRegistry {
    pub highlighters: HashMap<String, Vec<HighlightRule>>,
    pub snippets: HashMap<String, Vec<SnippetRule>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            highlighters: HashMap::new(),
            snippets: HashMap::new(),
        }
    }
}

pub fn register_api(lua: &Lua, registry: Arc<Mutex<PluginRegistry>>) -> LuaResult<()> {
    let globals = lua.globals();
    let zap_table = lua.create_table()?;

    let print_func = lua.create_function(|_, msg: String| {
        use std::io::Write;
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("zapnano.log") {
            let _ = writeln!(file, "Lua: {}", msg);
        }
        Ok(())
    })?;
    zap_table.set("print", print_func)?;

    let reg_clone = Arc::clone(&registry);
    let register_highlighter = lua.create_function(move |_, (ext, rules): (String, Vec<HashMap<String, String>>)| {
        let mut parsed_rules = Vec::new();
        for rule in rules {
            if let (Some(pat), Some(col)) = (rule.get("pattern"), rule.get("color")) {
                parsed_rules.push(HighlightRule {
                    pattern: pat.to_string(),
                    color_name: col.to_string(),
                });
            }
        }
        
        if let Ok(mut reg) = reg_clone.lock() {
            let entry = reg.highlighters.entry(ext).or_insert_with(Vec::new);
            entry.extend(parsed_rules);
        }
        Ok(())
    })?;
    zap_table.set("register_highlighter", register_highlighter)?;

    let reg_clone2 = Arc::clone(&registry);
    let register_snippets = lua.create_function(move |_, (ext, rules): (String, Vec<HashMap<String, String>>)| {
        let mut parsed_rules = Vec::new();
        for rule in rules {
            if let (Some(pref), Some(body)) = (rule.get("prefix"), rule.get("body")) {
                let desc = rule.get("description").cloned().unwrap_or_default();
                parsed_rules.push(SnippetRule {
                    prefix: pref.to_string(),
                    body: body.to_string(),
                    description: desc,
                });
            }
        }
        
        if let Ok(mut reg) = reg_clone2.lock() {
            let entry = reg.snippets.entry(ext).or_insert_with(Vec::new);
            entry.extend(parsed_rules);
        }
        Ok(())
    })?;
    zap_table.set("register_snippets", register_snippets)?;
    
    globals.set("zap", zap_table)?;
    Ok(())
}
