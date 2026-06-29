mod app;
mod editor;
mod input;
mod ui;
pub mod plugins;
mod settings;

use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;

fn create_plugin_template(name: &str) -> Result<()> {
    let dir = PathBuf::from("extensions").join(name);
    fs::create_dir_all(&dir)?;

    let init_path = dir.join("init.lua");
    let template = format!(
r#"-- Zap Nano Plugin: {name}
return {{
    name = "{name}",
    author = "Your Name",
    version = "1.0.0",
    
    on_load = function()
        -- Example of registering a basic syntax highlight regex:
        -- zap.register_highlighter("lua", {{
        --     {{ pattern = "TODO", color = "LightRed" }},
        -- }})
        
        -- Example of printing to the debug registry:
        -- zap.print("Plugin {name} loaded successfully!")
    end
}}
"#,
        name = name
    );

    fs::write(&init_path, template)?;
    println!("Successfully scaffolding Lua plugin: {:?}", init_path);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.iter().any(|arg| arg == "--version" || arg == "-v") {
        println!("ZapNano v0.1.0");
        return Ok(());
    }

    if args.len() == 3 && args[1] == "new" {
        return create_plugin_template(&args[2]);
    }

    let filepath = if args.len() >= 2 {
        Some(PathBuf::from(&args[1]))
    } else {
        None
    };

    let settings = settings::Settings::load();
    let mut app = app::App::new(filepath, settings)?;
    app.run()?;

    Ok(())
}
