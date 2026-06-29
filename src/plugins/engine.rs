use mlua::prelude::*;
use crate::plugins::api::{PluginRegistry, register_api};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct LuaEngine {
    pub lua: Lua,
    pub registry: Arc<Mutex<PluginRegistry>>,
}

pub fn get_extensions_dir() -> PathBuf {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            if let Some(app_dir) = exe_dir.parent() {
                let res_ext = app_dir.join("resources").join("extensions");
                if res_ext.exists() {
                    return res_ext;
                }
            }
        }
    }
    PathBuf::from("extensions")
}

impl LuaEngine {
    pub fn new(disabled_extensions: &[String]) -> Result<Self> {
        let lua = Lua::new();
        let registry = Arc::new(Mutex::new(PluginRegistry::new()));
        
        register_api(&lua, Arc::clone(&registry)).map_err(|e| anyhow::anyhow!("Failed to register API: {}", e))?;
        
        let engine = Self { lua, registry };
        
        let ext_dir = get_extensions_dir();
        if !ext_dir.exists() {
            let _ = fs::create_dir(&ext_dir);
        }

        engine.load_all_plugins(&ext_dir, disabled_extensions);

        Ok(engine)
    }

    pub fn load_all_plugins(&self, ext_dir: &PathBuf, disabled: &[String]) {
        if let Ok(entries) = fs::read_dir(ext_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    let dirname = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    if disabled.contains(&dirname) { continue; }
                    let init_file = path.join("init.lua");
                    if init_file.exists() && init_file.is_file() {
                        if let Err(e) = self.load_plugin(&init_file) {
                            let _ = self.lua.globals().get::<_, mlua::Table>("zap")
                                .and_then(|z| z.get::<_, mlua::Function>("print"))
                                .and_then(|p| p.call::<_, ()>(format!("Plugin Error in {:?}: {}", init_file, e)));
                        }
                    }
                }
            }
        }
    }

    fn load_plugin(&self, path: &PathBuf) -> Result<()> {
        let script = fs::read_to_string(path)?;
        
        let chunk = self.lua.load(&script);
        let plugin_table: mlua::Table = chunk.eval().map_err(|_| anyhow::anyhow!("Script must return a Lua table!"))?;
        
        let name: String = plugin_table.get("name").map_err(|_| anyhow::anyhow!("Plugin is missing 'name' field!"))?;
        let author: String = plugin_table.get("author").map_err(|_| anyhow::anyhow!("Plugin '{}' is missing 'author' field!", name))?;
        let version: String = plugin_table.get("version").map_err(|_| anyhow::anyhow!("Plugin '{}' is missing 'version' field!", name))?;
        
        let on_load: mlua::Function = plugin_table.get("on_load").map_err(|_| anyhow::anyhow!("Plugin '{}' is missing 'on_load' function!", name))?;
        on_load.call::<_, ()>(()).map_err(|e| anyhow::anyhow!("Failed running on_load for '{}': {}", name, e))?;

        let _ = self.lua.globals().get::<_, mlua::Table>("zap")
            .and_then(|z| z.get::<_, mlua::Function>("print"))
            .and_then(|p| p.call::<_, ()>(format!("Successfully loaded plugin: '{}' v{} by {}", name, version, author)));

        Ok(())
    }

    pub fn load_script(&self, script: &str) -> Result<()> {
        self.lua.load(script).exec()?;
        Ok(())
    }
}
