return {
    name = "js_pack",
    version = "1.0.0",
    author = "Zap VIM",
    on_load = function()
        zap.print("Plugin js_pack loaded fully!")

        zap.register_snippets("js", {
            { prefix = "clg", body = "console.log(@1);", description = "Console Log" },
            { prefix = "fn", body = "function @1(@2) {\n    @3\n}", description = "Function Declaration" },
            { prefix = "afn", body = "const @1 = (@2) => {\n    @3\n};", description = "Arrow Function" },
            { prefix = "import", body = "import { @1 } from '@2';", description = "ES6 Import" },
            { prefix = "export", body = "export const @1 = @2;", description = "ES6 Export" },
            { prefix = "for", body = "for (let i = 0; i < @1; i++) {\n    @2\n}", description = "For Loop" },
            { prefix = "forin", body = "for (const key in @1) {\n    @2\n}", description = "For In Loop" },
            { prefix = "forof", body = "for (const item of @1) {\n    @2\n}", description = "For Of Loop" },
            { prefix = "ife", body = "if (@1) {\n    @2\n} else {\n    @3\n}", description = "If Else Statement" },
            { prefix = "timeout", body = "setTimeout(() => {\n    @1\n}, @2);", description = "Set Timeout" },
            { prefix = "prom", body = "new Promise((resolve, reject) => {\n    @1\n});", description = "New Promise" },
            { prefix = "req", body = "const @1 = require('@2');", description = "CommonJS Require" },
            { prefix = "docsel", body = "document.querySelector('@1');", description = "Document Query Selector" },
        })

        zap.register_snippets("ts", {
            { prefix = "int", body = "interface @1 {\n    @2\n}", description = "TS Interface" },
            { prefix = "type", body = "type @1 = @2;", description = "TS Type Alias" },
        })

        zap.register_highlighter("js", {
            { pattern = "\\b(const|let|var|function|return|if|else|for|while|do|switch|case|default|break|continue|try|catch|finally|throw|new|this|super|class|extends|import|export|from|async|await|yield|in|of|instanceof|typeof|void|delete)\\b", color = "Magenta" },
            { pattern = "\\b(true|false|null|undefined|NaN|Infinity)\\b", color = "Cyan" },
            { pattern = "\\b(console|window|document|Math|JSON|Object|Array|String|Number|Boolean|Promise|Date|Map|Set|Symbol)\\b", color = "Blue" },
            { pattern = "\\b\\d+(\\.\\d+)?\\b", color = "LightBlue" },
            { pattern = "['\"][^'\"]*['\"]", color = "Yellow" },
            { pattern = "`[^`]*`", color = "Yellow" },
            { pattern = "//.*", color = "DarkGray" },
            { pattern = "/\\*[\\s\\S]*?\\*/", color = "DarkGray" },
            { pattern = "[{}()\\[\\]]", color = "White" },
            { pattern = "=>", color = "Magenta" },
            { pattern = "[+\\-*/%<>&|^!~=]", color = "White" },
            { pattern = "\\b[a-zA-Z_]\\w*(?=\\s*\\()", color = "Green" }, -- Function calls
        })

        zap.register_highlighter("ts", {
            { pattern = "\\b(interface|type|readonly|public|private|protected|implements|namespace|declare|any|unknown|never|string|number|boolean|symbol|bigint)\\b", color = "Cyan" },
        })
    end
}
