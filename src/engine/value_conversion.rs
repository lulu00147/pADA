const MICROS_PER_DAY: i64 = 86_400_000_000;
const MICROS_PER_SEC: i64 = 1_000_000;
#[cfg(feature = "duckdb-impl")]
pub fn value_to_json(v: &duckdb_crate::types::Value) -> serde_json::Value {
    use duckdb_crate::types::Value;
    match v {
        Value::Null => serde_json::Value::Null,
        Value::Boolean(b) => serde_json::Value::Bool(*b),
        Value::TinyInt(i) => serde_json::json!(i),
        Value::SmallInt(i) => serde_json::json!(i),
        Value::Int(i) => serde_json::json!(i),
        Value::BigInt(i) => serde_json::Value::String(i.to_string()),
        Value::HugeInt(h) => serde_json::Value::String(h.to_string()),
        Value::UTinyInt(u) => serde_json::json!(u),
        Value::USmallInt(u) => serde_json::json!(u),
        Value::UInt(u) => serde_json::json!(u),
        Value::UBigInt(u) => serde_json::Value::String(u.to_string()),
        Value::Float(f) => finite_f32_to_json(*f),
        Value::Double(f) => finite_f64_to_json(*f),
        Value::Decimal(d) => serde_json::Value::String(d.to_string()),
        Value::Text(s) => serde_json::Value::String(s.clone()),
        Value::Blob(b) => serde_json::Value::String(format!("<blob: {} bytes>", b.len())),
        Value::Date32(days) => serde_json::Value::String(epoch_days_to_string(*days)),
        Value::Time64(tu, val) => {
            let micros = tu.to_micros(*val);
            serde_json::Value::String(micros_to_time_string(micros))
        }
        Value::Timestamp(tu, val) => {
            let micros = tu.to_micros(*val);
            let days = micros.div_euclid(MICROS_PER_DAY) as i32;
            let time_micros = micros.rem_euclid(MICROS_PER_DAY);
            let date_str = epoch_days_to_string(days);
            let time_str = micros_to_time_string(time_micros);
            serde_json::Value::String(format!("{} {}", date_str, time_str))
        }
        Value::Interval {
            months,
            days,
            nanos,
        } => {
            let parts: Vec<String> = std::iter::empty()
                .chain(if *months != 0 {
                    Some(format!("{} months", months))
                } else {
                    None
                })
                .chain(if *days != 0 {
                    Some(format!("{} days", days))
                } else {
                    None
                })
                .chain(if *nanos != 0 {
                    Some(format!("{} ns", nanos))
                } else {
                    None
                })
                .collect();
            if parts.is_empty() {
                serde_json::Value::String("0".into())
            } else {
                serde_json::Value::String(parts.join(" "))
            }
        }
        Value::List(vals) => serde_json::Value::Array(vals.iter().map(value_to_json).collect()),
        Value::Array(vals) => serde_json::Value::Array(vals.iter().map(value_to_json).collect()),
        Value::Enum(s) => serde_json::Value::String(s.clone()),
        Value::Struct(fields) => {
            let mut map = serde_json::Map::new();
            for (k, val) in fields.iter() {
                map.insert(k.clone(), value_to_json(val));
            }
            serde_json::Value::Object(map)
        }
        Value::Map(entries) => {
            let mut map = serde_json::Map::new();
            for (k, val) in entries.iter() {
                let k_str = match value_to_json(k) {
                    serde_json::Value::String(s) => s,
                    other => other.to_string(),
                };
                map.insert(k_str, value_to_json(val));
            }
            serde_json::Value::Object(map)
        }
        Value::Union(inner) => value_to_json(inner.as_ref()),
    }
}
#[cfg(feature = "duckdb-impl")]
pub fn duckdb_value_to_json(row: &duckdb_crate::Row, idx: usize) -> serde_json::Value {
    use duckdb_crate::types::Value;
    match row.get::<_, Value>(idx) {
        Ok(val) => value_to_json(&val),
        Err(e) => {
            tracing::warn!("duckdb_value_to_json: row.get({}) failed: {}", idx, e);
            serde_json::Value::String("<error>".into())
        }
    }
}
pub fn epoch_days_to_string(days: i32) -> String {
    let z = days as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let era_days = era.checked_mul(146097).unwrap_or(i64::MAX);
    let doe = (z - era_days) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = (yoe as i64 + era.checked_mul(400).unwrap_or(i64::MAX)) as i32;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    format!("{:04}-{:02}-{:02}", y, m, d)
}
#[cfg(feature = "duckdb-impl")]
fn finite_f32_to_json(value: f32) -> serde_json::Value {
    if value.is_finite() {
        serde_json::from_str(&value.to_string()).expect("finite f32 is valid JSON")
    } else {
        non_finite_float_to_json(value.is_nan(), value.is_sign_positive())
    }
}
#[cfg(feature = "duckdb-impl")]
fn finite_f64_to_json(value: f64) -> serde_json::Value {
    if value.is_finite() {
        serde_json::Number::from_f64(value)
            .map(serde_json::Value::Number)
            .expect("finite f64 is always a JSON number")
    } else {
        non_finite_float_to_json(value.is_nan(), value.is_sign_positive())
    }
}
#[cfg(feature = "duckdb-impl")]
fn non_finite_float_to_json(is_nan: bool, is_sign_positive: bool) -> serde_json::Value {
    if is_nan {
        serde_json::Value::String("NaN".to_owned())
    } else if is_sign_positive {
        serde_json::Value::String("Infinity".to_owned())
    } else {
        serde_json::Value::String("-Infinity".to_owned())
    }
}
pub fn micros_to_time_string(micros: i64) -> String {
    let micros = micros.rem_euclid(MICROS_PER_DAY);
    let total_secs = micros / MICROS_PER_SEC;
    let h = (total_secs / 3600) % 24;
    let m = (total_secs / 60) % 60;
    let s = total_secs % 60;
    let fractional_micros = micros % MICROS_PER_SEC;
    format!("{:02}:{:02}:{:02}.{:06}", h, m, s, fractional_micros)
}
