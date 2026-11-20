pub fn escape_ident(name: &str) -> String {
    let mut result = String::new();
    result.push('"');
    for c in name.chars() {
        if c == '"' {
            result.push('"');
        }
        result.push(c);
    }
    result.push('"');
    result
}
pub fn escape_string_literal(value: &str) -> String {
    let mut result = String::new();
    result.push('\'');
    for c in value.chars() {
        if c == '\'' {
            result.push('\'');
        }
        result.push(c);
    }
    result.push('\'');
    result
}
pub fn strip_sql_comments(sql: &str) -> String {
    let mut result = String::with_capacity(sql.len());
    let chars: Vec<char> = sql.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\'' {
            result.push('\'');
            i += 1;
            while i < chars.len() {
                if chars[i] == '\'' {
                    if i + 1 < chars.len() && chars[i + 1] == '\'' {
                        result.push('\'');
                        result.push('\'');
                        i += 2;
                    } else {
                        result.push('\'');
                        i += 1;
                        break;
                    }
                } else {
                    result.push(chars[i]);
                    i += 1;
                }
            }
            continue;
        }
        if chars[i] == '-' && i + 1 < chars.len() && chars[i + 1] == '-' {
            let prev_needs_space = result
                .chars()
                .next_back()
                .map(|c| !c.is_whitespace())
                .unwrap_or(false);
            i += 2;
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            if i < chars.len() {
                i += 1;
            }
            let next_is_nonspace = i < chars.len() && !chars[i].is_whitespace();
            if prev_needs_space && next_is_nonspace {
                result.push(' ');
            }
            continue;
        }
        if chars[i] == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
            let prev_needs_space = result
                .chars()
                .next_back()
                .map(|c| !c.is_whitespace())
                .unwrap_or(false);
            let mut j = i + 2;
            while j + 1 < chars.len() && !(chars[j] == '*' && chars[j + 1] == '/') {
                j += 1;
            }
            let next_is_nonspace = if j + 1 < chars.len() {
                let k = j + 2;
                k < chars.len() && !chars[k].is_whitespace()
            } else {
                false
            };
            if prev_needs_space && next_is_nonspace {
                result.push(' ');
            }
            i = if j + 1 < chars.len() {
                j + 2
            } else {
                chars.len()
            };
            continue;
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}
pub fn lowercase_preserving_strings(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\'' {
            result.push('\'');
            i += 1;
            while i < chars.len() {
                if chars[i] == '\'' {
                    if i + 1 < chars.len() && chars[i + 1] == '\'' {
                        result.push('\'');
                        result.push('\'');
                        i += 2;
                    } else {
                        result.push('\'');
                        i += 1;
                        break;
                    }
                } else {
                    result.push(chars[i]);
                    i += 1;
                }
            }
        } else {
            for lc in chars[i].to_lowercase() {
                result.push(lc);
            }
            i += 1;
        }
    }
    result
}
fn collapse_whitespace_outside_quotes(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i: usize = 0;
    let mut quote: char = '\0';
    let mut pending_space = false;
    while i < chars.len() {
        let c = chars[i];
        if quote != '\0' {
            result.push(c);
            if c == quote {
                if i + 1 < chars.len() && chars[i + 1] == quote {
                    result.push(quote);
                    i += 2;
                    continue;
                }
                quote = '\0';
            }
            i += 1;
            continue;
        }
        if c == '\'' || c == '"' {
            if pending_space {
                result.push(' ');
                pending_space = false;
            }
            quote = c;
            result.push(c);
            i += 1;
            continue;
        }
        if c.is_whitespace() {
            pending_space = true;
            i += 1;
            continue;
        }
        if pending_space {
            result.push(' ');
            pending_space = false;
        }
        result.push(c);
        i += 1;
    }
    result
}
fn normalize_stripped(no_comments: &str) -> String {
    let lowered = lowercase_preserving_strings(no_comments);
    let collapsed = collapse_whitespace_outside_quotes(&lowered);
    collapsed
        .trim()
        .trim_end_matches(|c: char| c == ';' || c.is_whitespace())
        .to_string()
}
pub fn normalize(sql: &str) -> String {
    normalize_stripped(&strip_sql_comments(sql))
}
