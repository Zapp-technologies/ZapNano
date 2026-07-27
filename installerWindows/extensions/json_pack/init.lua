return {
    name = "json_pack",
    version = "1.0.0",
    author = "Zap VIM",
    on_load = function()
        zap.print("Plugin json_pack loaded fully!")

        zap.register_snippets("json", {
            { prefix = "kv", body = "\"@1\": \"@2\"", description = "Key-Value pair" },
        })

        zap.register_highlighter("json", {
            -- Keys
            { pattern = "\"[^\"]*\"%s*:", color = "Keyword" },
            -- String Values
            { pattern = ":%s*\"[^\"]*\"", color = "String" },
            -- Numbers
            { pattern = "%-?%d+%.?%d*", color = "LightRed" },
            -- Booleans / Null
            { pattern = "%f[%a](true|false|null)%f[%A]", color = "Special" },
            -- Structural brackets
            { pattern = "[%{%}%[%]]", color = "Operator" },
        })
    end
}
