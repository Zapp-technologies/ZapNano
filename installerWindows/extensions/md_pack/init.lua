return {
    name = "md_pack",
    version = "1.0.0",
    author = "Zap VIM",
    on_load = function()
        zap.print("Plugin md_pack loaded fully!")

        zap.register_snippets("md", {
            { prefix = "link", body = "[@1](@2)", description = "Markdown Link" },
            { prefix = "code", body = "```@1\n@2\n```", description = "Fenced Code Block" },
        })

        zap.register_highlighter("md", {
            -- Headers
            { pattern = "^#+.*", color = "Magenta" },
            -- Bold
            { pattern = "%*%*.-%*%*", color = "Special" },
            -- Italic
            { pattern = "%*.-%*", color = "Special" },
            -- Inline Code
            { pattern = "`.-`", color = "String" },
            -- Lists
            { pattern = "^%s*[%-%*%+]%s+", color = "Operator" },
            { pattern = "^%s*%d+%.%s+", color = "Operator" },
            -- Links
            { pattern = "%[.-%]%(.-%)", color = "Function" },
        })
    end
}
