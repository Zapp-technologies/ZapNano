use crate::settings::IndentPairConfig;

#[derive(Debug, PartialEq, Clone)]
pub struct MoveCloseDetail {
    pub close_bracket: String,
    pub close_line_indent: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndentDecision {
    pub new_line_indent: String,
    pub move_close: Option<MoveCloseDetail>,
}

fn get_leading_whitespace(s: &str) -> &str {
    let mut end = 0;
    for c in s.chars() {
        if c == ' ' || c == '\t' {
            end += c.len_utf8();
        } else {
            break;
        }
    }
    &s[..end]
}

pub fn determine_indent(
    text_before: &str,
    text_after: &str,
    config: &IndentPairConfig,
    default_tab_size: usize,
) -> IndentDecision {
    let current_line_before = match text_before.rfind('\n') {
        Some(idx) => &text_before[idx + 1..],
        None => text_before,
    };

    let current_line_after = match text_after.find('\n') {
        Some(idx) => &text_after[..idx],
        None => text_after,
    };

    let base_indent = get_leading_whitespace(current_line_before);

    for pair in &config.pairs {
        if current_line_before.ends_with(&pair.open)
            && current_line_after.starts_with(&pair.close)
        {
            let indent_size = pair.indent_size.unwrap_or(default_tab_size);
            let extra_indent = " ".repeat(indent_size);
            return IndentDecision {
                new_line_indent: format!("{}{}", base_indent, extra_indent),
                move_close: Some(MoveCloseDetail {
                    close_bracket: pair.close.clone(),
                    close_line_indent: base_indent.to_string(),
                }),
            };
        }
    }

    let trimmed_before = current_line_before.trim_end();
    for pair in &config.pairs {
        if trimmed_before.ends_with(&pair.open) {
            let indent_size = pair.indent_size.unwrap_or(default_tab_size);
            let extra_indent = " ".repeat(indent_size);
            return IndentDecision {
                new_line_indent: format!("{}{}", base_indent, extra_indent),
                move_close: None,
            };
        }
    }

    IndentDecision {
        new_line_indent: base_indent.to_string(),
        move_close: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::BracketPair;

    fn test_config() -> IndentPairConfig {
        IndentPairConfig {
            pairs: vec![
                BracketPair {
                    open: "{".to_string(),
                    close: "}".to_string(),
                    indent_size: None,
                },
                BracketPair {
                    open: "(".to_string(),
                    close: ")".to_string(),
                    indent_size: None,
                },
                BracketPair {
                    open: "[".to_string(),
                    close: "]".to_string(),
                    indent_size: None,
                },
            ],
        }
    }

    #[test]
    fn test_no_pair() {
        let config = test_config();
        let decision = determine_indent("  let x = 5;", "", &config, 4);
        assert_eq!(
            decision,
            IndentDecision {
                new_line_indent: "  ".to_string(),
                move_close: None,
            }
        );
    }

    #[test]
    fn test_between_brackets() {
        let config = test_config();
        
        // {...|}
        let decision = determine_indent("    {", "}", &config, 4);
        assert_eq!(
            decision,
            IndentDecision {
                new_line_indent: "        ".to_string(),
                move_close: Some(MoveCloseDetail {
                    close_bracket: "}".to_string(),
                    close_line_indent: "    ".to_string(),
                }),
            }
        );

        // (...|)
        let decision = determine_indent(" (", ")", &config, 4);
        assert_eq!(
            decision,
            IndentDecision {
                new_line_indent: "     ".to_string(),
                move_close: Some(MoveCloseDetail {
                    close_bracket: ")".to_string(),
                    close_line_indent: " ".to_string(),
                }),
            }
        );
    }

    #[test]
    fn test_increase_indent_no_pair() {
        let config = test_config();

        let decision = determine_indent("  fn test() {", "", &config, 4);
        assert_eq!(
            decision,
            IndentDecision {
                new_line_indent: "      ".to_string(),
                move_close: None,
            }
        );

        let decision = determine_indent("  fn test() {  ", "", &config, 4);
        assert_eq!(
            decision,
            IndentDecision {
                new_line_indent: "      ".to_string(),
                move_close: None,
            }
        );
    }
}
