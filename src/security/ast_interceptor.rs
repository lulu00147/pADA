use regex::Regex;
use sqlparser::ast::{self, ObjectType};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::sync::LazyLock;
static RE_INSTALL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\bINSTALL\b").unwrap());
static RE_LOAD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\bLOAD\b").unwrap());
static RE_BLOCK_COMMENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/").unwrap());
static RE_SPATIAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:ST_READ|ST_READTEXT|ST_READFILE|READFILE|ST_GEOGFROMTEXT|ST_POINT|ST_BUFFER|ST_CONTAINS|ST_INTERSECTS|ST_AREA|ST_DISTANCE|ST_UNION|ST_ASTEXT|ST_ASGEOJSON|ST_MAKEPOINT|ST_COLLECT|ST_EXTENT|ST_CENTROID|ST_TRANSFORM)\b").unwrap()
});
static RE_ATTACH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(?:ATTACH|DETACH)\b").unwrap());
#[derive(Debug, Clone)]
pub struct ExtractedPath {
    pub function_name: String,
    pub file_path: String,
}
pub struct AstInspector;
impl AstInspector {
    pub fn strip_sql_literals(sql: &str) -> String {
        let no_comments = crate::utils::sql_normalize::strip_sql_comments(sql);
        let mut result = String::with_capacity(no_comments.len());
        let mut chars = no_comments.chars().peekable();
        while let Some(ch) = chars.next() {
            result.push(ch);
            if ch == '\'' {
                let mut closed = false;
                while let Some(c) = chars.next() {
                    if c == '\'' {
                        if chars.peek() == Some(&'\'') {
                            chars.next();
                        } else {
                            result.push('\'');
                            closed = true;
                            break;
                        }
                    }
                }
                if !closed {
                    result.push('\'');
                }
            }
        }
        result
    }
    pub fn contains_always_blocked(sql: &str) -> bool {
        let no_block = RE_BLOCK_COMMENT.replace_all(sql, "").to_string();
        let clean = Self::strip_sql_literals(&no_block);
        RE_INSTALL.is_match(&clean) || RE_LOAD.is_match(&clean)
    }
    pub fn contains_attach_detach(sql: &str) -> bool {
        RE_ATTACH.is_match(&Self::strip_sql_literals(sql))
    }
    pub fn contains_dangerous_ddl_from_statements(stmts: &[ast::Statement], _sql: &str) -> bool {
        for stmt in stmts {
            match stmt {
                ast::Statement::CreateTable { .. }
                | ast::Statement::CreateIndex { .. }
                | ast::Statement::CreateSchema { .. }
                | ast::Statement::CreateSequence { .. }
                | ast::Statement::CreateFunction { .. }
                | ast::Statement::CreateMacro { .. }
                | ast::Statement::CreateView { .. }
                | ast::Statement::Drop { .. }
                | ast::Statement::AlterTable { .. }
                | ast::Statement::AlterIndex { .. }
                | ast::Statement::Truncate { .. }
                | ast::Statement::Insert { .. }
                | ast::Statement::Update { .. }
                | ast::Statement::Delete { .. }
                | ast::Statement::Copy { .. }
                | ast::Statement::Merge { .. }
                | ast::Statement::Call { .. } => return true,
                _ => {}
            }
        }
        false
    }
    pub fn contains_spatial_functions(sql: &str) -> bool {
        RE_SPATIAL.is_match(&Self::strip_sql_literals(sql))
    }
    pub(crate) fn extract_file_paths_from_statements<'a>(
        stmts: impl Iterator<Item = &'a ast::Statement>,
    ) -> Vec<ExtractedPath> {
        scan_file_functions(
            &stmts
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; "),
        )
    }
    pub fn extract_file_paths(sql: &str) -> Vec<ExtractedPath> {
        if Parser::parse_sql(&GenericDialect, sql).is_ok() {
            scan_file_functions(sql)
        } else {
            Vec::new()
        }
    }
    pub(crate) fn extract_file_paths_fallback(sql: &str) -> Vec<ExtractedPath> {
        scan_file_functions(sql)
    }
    pub(crate) fn validate_write_statements(stmts: &[ast::Statement]) -> Result<(), String> {
        for statement in stmts {
            let allowed = match statement {
                ast::Statement::Insert { .. }
                | ast::Statement::Update { .. }
                | ast::Statement::Delete { .. }
                | ast::Statement::AlterTable { .. } => true,
                ast::Statement::CreateTable {
                    or_replace,
                    temporary,
                    external,
                    location,
                    like,
                    clone,
                    ..
                } => {
                    !or_replace
                        && !temporary
                        && !external
                        && location.is_none()
                        && like.is_none()
                        && clone.is_none()
                }
                ast::Statement::Drop { object_type, .. } => {
                    matches!(object_type, ObjectType::Table)
                }
                _ => false,
            };
            if !allowed {
                return Err(format!(
                    "Write SQL statement '{}' is not in the scratchpad allowlist",
                    statement
                ));
            }
        }
        if stmts
            .iter()
            .any(|statement| contains_three_part_identifier(&statement.to_string()))
        {
            return Err("Three-part object names are not allowed in scratchpad SQL".into());
        }
        Ok(())
    }
}
fn contains_three_part_identifier(sql: &str) -> bool {
    let bytes = sql.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\'' {
            index += 1;
            while index < bytes.len() {
                if bytes[index] == b'\'' {
                    index += 1;
                    if index < bytes.len() && bytes[index] == b'\'' {
                        index += 1;
                    } else {
                        break;
                    }
                } else {
                    index += 1;
                }
            }
            continue;
        }
        let Some(next) = consume_identifier(bytes, index) else {
            index += 1;
            continue;
        };
        let mut component_count = 1;
        let mut cursor = next;
        loop {
            while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
                cursor += 1;
            }
            if cursor >= bytes.len() || bytes[cursor] != b'.' {
                break;
            }
            cursor += 1;
            while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
                cursor += 1;
            }
            let Some(next_component) = consume_identifier(bytes, cursor) else {
                break;
            };
            component_count += 1;
            if component_count >= 3 {
                return true;
            }
            cursor = next_component;
        }
        index = next;
    }
    false
}
fn consume_identifier(bytes: &[u8], start: usize) -> Option<usize> {
    if start >= bytes.len() {
        return None;
    }
    if bytes[start] == b'"' {
        let mut index = start + 1;
        while index < bytes.len() {
            if bytes[index] == b'"' {
                index += 1;
                if index < bytes.len() && bytes[index] == b'"' {
                    index += 1;
                } else {
                    return Some(index);
                }
            } else {
                index += 1;
            }
        }
        return None;
    }
    if !(bytes[start].is_ascii_alphabetic() || bytes[start] == b'_') {
        return None;
    }
    let mut index = start + 1;
    while index < bytes.len() && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_') {
        index += 1;
    }
    Some(index)
}
fn allowed_file_function(name: &str) -> bool {
    matches!(
        name,
        "READ_CSV"
            | "READ_CSV_AUTO"
            | "READ_PARQUET"
            | "READ_JSON"
            | "READ_JSON_AUTO"
            | "READ_XLSX"
            | "READ_EXCEL"
            | "ST_READ"
            | "ST_READTEXT"
            | "ST_READFILE"
            | "READFILE"
    )
}
fn suspicious_file_function(name: &str) -> bool {
    name.starts_with("READ_")
        || name.starts_with("SCAN_")
        || name.contains("FILE")
        || name.contains("PATH")
        || matches!(name, "PARQUET_SCAN" | "CSV_SCAN" | "JSON_SCAN")
}
fn scan_file_functions(sql: &str) -> Vec<ExtractedPath> {
    let bytes = sql.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_alphabetic() || bytes[i] == b'_' {
            let start = i;
            i += 1;
            while i < bytes.len()
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.')
            {
                i += 1;
            }
            let name = sql[start..i]
                .rsplit('.')
                .next()
                .unwrap()
                .to_ascii_uppercase();
            let mut open = i;
            while open < bytes.len() && bytes[open].is_ascii_whitespace() {
                open += 1;
            }
            if open < bytes.len() && bytes[open] == b'(' {
                if let Some((_end, args)) = parenthesized(sql, open) {
                    if allowed_file_function(&name) {
                        if let Some(path) = first_static_string(args) {
                            out.push(ExtractedPath {
                                function_name: name,
                                file_path: path,
                            });
                        } else {
                            out.push(ExtractedPath {
                                function_name: name,
                                file_path: "/__pada_dynamic_file_loader_argument__".into(),
                            });
                        }
                    } else if suspicious_file_function(&name) {
                        out.push(ExtractedPath {
                            function_name: name,
                            file_path: "/__pada_unapproved_file_function__".into(),
                        });
                    }
                    i = open + 1;
                }
            }
        } else {
            i += 1;
        }
    }
    out
}
fn parenthesized(sql: &str, open: usize) -> Option<(usize, &str)> {
    let b = sql.as_bytes();
    let mut depth = 0;
    let mut quoted = false;
    let mut i = open;
    while i < b.len() {
        if b[i] == b'\'' {
            if quoted && i + 1 < b.len() && b[i + 1] == b'\'' {
                i += 2;
                continue;
            }
            quoted = !quoted;
        } else if !quoted && b[i] == b'(' {
            depth += 1;
        } else if !quoted && b[i] == b')' {
            depth -= 1;
            if depth == 0 {
                return Some((i + 1, &sql[open + 1..i]));
            }
        }
        i += 1;
    }
    None
}
fn first_static_string(args: &str) -> Option<String> {
    let s = args.trim_start();
    if !s.starts_with('\'') {
        return None;
    }
    let mut value = String::new();
    let mut chars = s[1..].chars();
    while let Some(c) = chars.next() {
        if c == '\'' {
            if chars.clone().next() == Some('\'') {
                chars.next();
                value.push('\'');
            } else {
                return Some(value);
            }
        } else {
            value.push(c);
        }
    }
    None
}
