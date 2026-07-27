return {
    name = "css_pack",
    version = "1.0.0",
    author = "Zap VIM",
    on_load = function()
        zap.print("Plugin css_pack loaded fully!")

        zap.register_snippets("css", {
            { prefix = "flex", body = "display: flex;\njustify-content: @1;\nalign-items: @2;", description = "Flexbox Base" },
            { prefix = "grid", body = "display: grid;\ngrid-template-columns: @1;\ngrid-template-rows: @2;", description = "Grid Base" },
            { prefix = "mq", body = "@media (max-width: @1) {\n    @2\n}", description = "Media Query" },
            { prefix = "reset", body = "* {\n    margin: 0;\n    padding: 0;\n    box-sizing: border-box;\n}", description = "CSS Reset" },
            { prefix = "bg", body = "background-color: @1;", description = "Background Color" },
            { prefix = "posa", body = "position: absolute;\ntop: @1;\nleft: @2;", description = "Absolute Position" },
            { prefix = "posr", body = "position: relative;", description = "Relative Position" },
            { prefix = "font", body = "font-family: @1;\nfont-size: @2;\nfont-weight: @3;", description = "Font Rules" },
            { prefix = "anim", body = "animation: @1 @2s ease-in-out forwards;", description = "Animation Shorthand" },
            { prefix = "kf", body = "@keyframes @1 {\n    0% {\n        @2\n    }\n    100% {\n        @3\n    }\n}", description = "Keyframes" },
        })

        zap.register_highlighter("css", {
            { pattern = "@(media|keyframes|import|charset|font-face|supports)\\b", color = "Magenta" },
            { pattern = "\\.[a-zA-Z0-9_-]+", color = "Yellow" },
            { pattern = "#[a-zA-Z0-9_-]+", color = "Yellow" },
            { pattern = ":[a-zA-Z0-9_-]+", color = "Cyan" },
            { pattern = "[a-zA-Z-]+\\s*(?=\\:)", color = "Cyan" },
            { pattern = "(?<=:)\\s*[^;{}]+", color = "Green" },
            { pattern = "/\\*[\\s\\S]*?\\*/", color = "DarkGray" },
            { pattern = "\\b\\d+(px|em|rem|%|vh|vw|ms|s|deg)?\\b", color = "LightBlue" },
            { pattern = "url\\([^)]*\\)", color = "Yellow" },
            { pattern = "[{}]", color = "White" },
            { pattern = ";", color = "White" },
        })
    end
}
