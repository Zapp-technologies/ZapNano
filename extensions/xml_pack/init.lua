return {
    name = "xml_pack",
    version = "1.0.0",
    author = "Zap VIM",
    on_load = function()
        zap.print("Plugin xml_pack loaded fully!")

        zap.register_snippets("xml", {
            { prefix = "tag", body = "<@1>\n    @2\n</@1>", description = "XML Tag" },
        })

        zap.register_highlighter("xml", {
            -- Declarations/Headers
            { pattern = "<\\?[^>]*\\?>", color = "Magenta" },
            -- Comment block
            { pattern = "<!--[\\s\\S]*?-->", color = "DarkGray" },
            -- Tag names (opening/closing tag matches)
            { pattern = "</?[a-zA-Z0-9_-]+", color = "Cyan" },
            -- Attributes
            { pattern = "\\b[a-zA-Z0-9_-]+\\s*=", color = "Green" },
            -- Quoted Attribute Values
            { pattern = "\"[^\"]*\"", color = "Yellow" },
            { pattern = "'[^']*'", color = "Yellow" },
            -- Closers
            { pattern = "/?>", color = "Cyan" },
        })
    end
}
