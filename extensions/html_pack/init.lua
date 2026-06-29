return {
    name = "html_pack",
    version = "1.0.0",
    author = "Zap VIM",
    on_load = function()
        zap.print("Plugin html_pack loaded fully!")

        zap.register_snippets("html", {
            { prefix = "!", body = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <title>@1</title>\n</head>\n<body>\n    @2\n</body>\n</html>", description = "HTML5 Boilerplate" },
            { prefix = "div", body = "<div class=\"@1\">\n    @2\n</div>", description = "Div Element" },
            { prefix = "span", body = "<span class=\"@1\">@2</span>", description = "Span Element" },
            { prefix = "a", body = "<a href=\"@1\">@2</a>", description = "Anchor Tag" },
            { prefix = "img", body = "<img src=\"@1\" alt=\"@2\" />", description = "Image Tag" },
            { prefix = "form", body = "<form action=\"@1\" method=\"@2\">\n    @3\n</form>", description = "Form Element" },
            { prefix = "input", body = "<input type=\"@1\" name=\"@2\" value=\"@3\" />", description = "Input Field" },
            { prefix = "button", body = "<button type=\"@1\">@2</button>", description = "Button" },
            { prefix = "ul", body = "<ul>\n    <li>@1</li>\n</ul>", description = "Unordered List" },
            { prefix = "table", body = "<table>\n    <thead>\n        <tr>\n            <th>@1</th>\n        </tr>\n    </thead>\n    <tbody>\n        <tr>\n            <td>@2</td>\n        </tr>\n    </tbody>\n</table>", description = "Table Structure" },
            { prefix = "link", body = "<link rel=\"stylesheet\" href=\"@1\">", description = "CSS Link" },
            { prefix = "script", body = "<script src=\"@1\"></script>", description = "JS Script Link" },
        })

        zap.register_highlighter("html", {
            { pattern = "<!DOCTYPE[^>]*>", color = "Magenta" },
            { pattern = "</?[a-zA-Z0-9:-]+", color = "Cyan" },
            { pattern = "\\s*[a-zA-Z0-9:-]+(?=\\=)", color = "Green" },
            { pattern = "\"[^\"]*\"", color = "Yellow" },
            { pattern = "'[^']*'", color = "Yellow" },
            { pattern = "/>", color = "Cyan" },
            { pattern = ">", color = "Cyan" },
            { pattern = "<!--[\\s\\S]*?-->", color = "DarkGray" },
            { pattern = "&[a-zA-Z0-9#]+;", color = "Red" },
            -- Inline CSS Properties and JS Keywords
            { pattern = "\\b(background-color|background|color|margin|padding|display|width|height|font-family|font-size|font-weight|border|border-radius|position|top|left|right|bottom|justify-content|align-items|flex-direction|flex|grid|gap|box-sizing|transition|transform|opacity|overflow|z-index)(?=\\s*:)", color = "Cyan" },
            { pattern = "(?<=:\\s*)(red|blue|green|yellow|black|white|grey|gray|flex|grid|block|inline|absolute|relative|fixed|none|auto|pointer|transparent)\\b", color = "Green" },
            { pattern = "\\b(const|let|var|function|return|if|else|for|while|do|switch|case|default|break|continue|try|catch|finally|throw|new|this|class|import|export|async|await)\\b", color = "Magenta" },
            { pattern = "\\b(console|window|document|Math|JSON|Promise|Object|Array)\\b", color = "Blue" },
            { pattern = "\\b[a-zA-Z_]\\w*(?=\\s*\\()", color = "Green" },
            { pattern = "\\b\\d+(px|em|rem|%|vh|vw|ms|s|deg)?\\b", color = "LightBlue" },
        })
    end
}
