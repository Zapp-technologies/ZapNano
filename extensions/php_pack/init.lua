-- Zap Nano Plugin: php_pack
return {
    name = "php_pack",
    author = "Zap VIM",
    version = "1.0.0",
    on_load = function()
        zap.register_highlighter("php", {
            -- PHP Tags
            { pattern = "<\\?php", color = "Special" },
            { pattern = "\\?>", color = "Special" },
            -- Single-line and Multi-line comments
            { pattern = "//[^\n]*", color = "DarkGray" },
            { pattern = "/\\*.*?\\*/", color = "DarkGray" },
            -- Controls & Types (Keywords)
            { pattern = "\\b(function|class|public|private|protected|static|extends|implements|namespace|use|if|else|elseif|return|echo|new|true|false|null|array|try|catch|foreach|as|while|do|throw|try|finally)\\b", color = "Keyword" },
            -- Core Built-in functions
            { pattern = "\\b(strlen|str_replace|explode|implode|count|isset|empty|is_array|json_encode|json_decode|die|exit|print_r|var_dump)\\b", color = "Function" },
            -- Numbers
            { pattern = "\\b\\d+(\\.\\d+)?\\b", color = "LightRed" },
            -- Variables
            { pattern = "\\$[a-zA-Z_\\x7f-\\xff][a-zA-Z0-9_\\x7f-\\xff]*", color = "Variable" },
            -- Object / method operators
            { pattern = "->", color = "Operator" },
            { pattern = "=>", color = "Operator" },
            { pattern = "::", color = "Operator" },
            -- Strings
            { pattern = "\"[^\"]*\"", color = "String" },
            { pattern = "'[^']*'", color = "String" },
        })
        zap.print("Plugin php_pack loaded fully!")
    end
}
