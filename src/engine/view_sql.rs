use crate::utils::{escape_ident, escape_string_literal};
pub fn build_create_view_sql(
    safe_name: &str,
    path_str: &str,
    format: &str,
    sheet_name: Option<&str>,
) -> Result<String, String> {
    let escaped_path = escape_string_literal(path_str);
    let safe_name = escape_ident(safe_name);
    match format {
        "csv" => Ok(format!(
            "CREATE OR REPLACE VIEW {name} AS SELECT * FROM read_csv_auto({path}, auto_detect=true);",
            name = safe_name,
            path = escaped_path
        )),
        "parquet" => Ok(format!(
            "CREATE OR REPLACE VIEW {name} AS SELECT * FROM read_parquet({path});",
            name = safe_name,
            path = escaped_path
        )),
        "json" | "jsonl" | "ndjson" => Ok(format!(
            "CREATE OR REPLACE VIEW {name} AS SELECT * FROM read_json_auto({path});",
            name = safe_name,
            path = escaped_path
        )),
        "excel" => {
            let sheet_part = sheet_name
                .map(|s| format!(", sheet = {}", escape_string_literal(s)))
                .unwrap_or_default();
            Ok(format!(
                "CREATE OR REPLACE VIEW {name} AS SELECT * FROM read_xlsx({path}{sheet});",
                name = safe_name,
                path = escaped_path,
                sheet = sheet_part
            ))
        }
        "geojson" | "shp" | "gpkg" => Ok(format!(
            "CREATE OR REPLACE VIEW {name} AS SELECT * FROM ST_Read({path});",
            name = safe_name,
            path = escaped_path
        )),
        _ => Err(format!("Unsupported format: {}", format)),
    }
}
fn stable_identity_hash(identity: &str) -> u64 {
    identity.bytes().fold(0xcbf29ce484222325u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
    })
}
pub fn generate_short_alias(stem: &str) -> String {
    format!("vw_{:016x}", stable_identity_hash(stem))
}
pub fn generate_source_view_name(identity: &str) -> String {
    format!("src_{:016x}", stable_identity_hash(identity))
}
pub fn build_alias_view_sql_for(alias_name: &str, original_name: &str) -> String {
    format!(
        "CREATE OR REPLACE VIEW {alias} AS SELECT * FROM {orig};",
        alias = escape_ident(alias_name),
        orig = escape_ident(original_name),
    )
}
pub fn build_alias_view_sql(original_name: &str) -> Result<(String, String), String> {
    let alias = generate_short_alias(original_name);
    let sql = build_alias_view_sql_for(&alias, original_name);
    Ok((alias, sql))
}
