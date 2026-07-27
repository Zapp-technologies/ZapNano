return {
    name = "toml_pack",
    version = "1.0.0",
    author = "Zap VIM",
    on_load = function()
        zap.print("Plugin toml_pack loaded fully!")

        zap.register_snippets("toml", {
            { prefix = "sec", body = "[@1]\n@2 = \"@3\"", description = "TOML Section" },
        })

        zap.register_highlighter("toml", {
            -- Comments
            { pattern = "#.*", color = "DarkGray" },
            -- Section headers
            { pattern = "\\[[a-zA-Z0-9_\\.-]+\\]", color = "Special" },
            -- Keys
            { pattern = "\\b[a-zA-Z0-9_-]+\\s*=", color = "Keyword" },
            -- String values
            { pattern = "\"[^\"]*\"", color = "String" },
            { pattern = "'[^']*'", color = "String" },
            -- Numbers
            { pattern = "\\b\\d+(?:\\.\\d+)?\\b", color = "LightRed" },
            -- Booleans
            { pattern = "\\b(true|false)\\b", color = "Special" },
        })
    end
}
