use crate::mcp::error::JsonRpcError;
pub fn interpret_duckdb_error(err: &str) -> (String, Option<String>) {
    let lower = err.to_lowercase();
    if lower.contains("ai_scratchpad.ai_scratchpad") {
        return (
            "Scratchpad table name must be unqualified".into(),
            Some("Write connection already uses schema ai_scratchpad. Use CREATE TABLE tm_<16 hex> AS SELECT ... FROM vw_... ; do not write CREATE TABLE ai_scratchpad.tm_... (that becomes a three-part name). Scratchpad tables are write-only; query vw_... with execute_read_sql.".into()),
        );
    }
    if lower.contains("not exist")
        && (lower.contains("table") || lower.contains("view") || lower.contains("relation"))
    {
        return (
            "Table or view not found".into(),
            Some(
                "Use discover_workspace to see available vw_<16 hex> views. \
                  Query views only — never query table names or tm_*. \
                  Never use raw file names as tables."
                    .into(),
            ),
        );
    }
    if lower.contains("column") && lower.contains("not found") {
        return (
            "Column not found in table".into(),
            Some("Use discover_workspace to see available columns. Check for typos or case sensitivity.".into()),
        );
    }
    if lower.contains("syntax error") || lower.contains("parser error") {
        return (
            "SQL syntax error".into(),
            Some("Check for missing commas, unbalanced quotes, or incorrect keyword spelling. Use explain_query to validate complex queries.".into()),
        );
    }
    if lower.contains("cannot cast") || (lower.contains("type") && lower.contains("mismatch")) {
        return (
            "Data type error".into(),
            Some(
                "Use CAST() or :: to convert types. Check column types with discover_workspace."
                    .into(),
            ),
        );
    }
    if lower.contains("already exists") {
        return (
            "Object already exists".into(),
            Some("Use a different name, or DROP the existing object first via execute_scratchpad_sql.".into()),
        );
    }
    if lower.contains("ambiguous") {
        return (
            "Ambiguous column reference".into(),
            Some("Qualify the column with table alias (e.g. t.column_name) when joining tables with overlapping column names.".into()),
        );
    }
    if lower.contains("out of memory") {
        return (
            "Memory exceeded".into(),
            Some("Reduce row count with LIMIT, add WHERE filters, or use bounding-box pre-filtering for spatial queries. Consider splitting into smaller batches.".into()),
        );
    }
    if lower.contains("timeout") || lower.contains("timed out") {
        return (
            "Query timeout".into(),
            Some("Simplify the query, add indexes via CTAS, reduce JOIN scope, or split into steps via execute_pipeline.".into()),
        );
    }
    if lower.contains("binder error") {
        return (
            "Query planning error (Binder)".into(),
            Some("The SQL references something DuckDB cannot resolve. Check table/column names, function availability, or use explain_query to debug.".into()),
        );
    }
    if lower.contains("catalog error") {
        return (
            "Schema/catalog error".into(),
            Some("The referenced schema or database object doesn't exist. Use discover_workspace to see available resources.".into()),
        );
    }
    if lower.contains("cannot open") || lower.contains("no files found") {
        return (
            "File access error".into(),
            Some("The file path may be incorrect or outside the workspace. Register a known workspace path with register_data_source, or upload the file through the HTTP transport.".into()),
        );
    }
    ("Database operation failed".into(), None)
}
pub fn engine_err_to_jsonrpc(err: &dyn std::fmt::Display) -> JsonRpcError {
    let (msg, suggestion) = interpret_duckdb_error(&err.to_string());
    let detail = if let Some(s) = suggestion {
        format!("{}. {}", msg, s)
    } else {
        msg
    };
    JsonRpcError::internal_error(detail)
}
