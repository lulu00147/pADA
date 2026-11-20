#[allow(clippy::manual_range_contains)]
fn contains_case_insensitive(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() || haystack.len() < needle.len() {
        return false;
    }
    let hb = haystack.as_bytes();
    let nb = needle.as_bytes();
    let mut i = 0;
    while i + nb.len() <= hb.len() {
        let mut j = 0;
        let mut ok = true;
        while j < nb.len() {
            let hc = hb[i + j];
            let nc = nb[j];
            let hl = if hc >= b'A' && hc <= b'Z' {
                hc + 32
            } else {
                hc
            };
            let nl = if nc >= b'A' && nc <= b'Z' {
                nc + 32
            } else {
                nc
            };
            if hl != nl {
                ok = false;
                break;
            }
            j += 1;
        }
        if ok {
            return true;
        }
        i += 1;
    }
    false
}
#[allow(clippy::manual_range_contains)]
fn contains_base_paren(haystack: &str, base: &str) -> bool {
    if base.is_empty() || haystack.len() < base.len() {
        return false;
    }
    let hb = haystack.as_bytes();
    let bb = base.as_bytes();
    let mut i = 0;
    while i + bb.len() <= hb.len() {
        let mut j = 0;
        let mut ok = true;
        while j < bb.len() {
            let hc = hb[i + j];
            let nc = bb[j];
            let hl = if hc >= b'A' && hc <= b'Z' {
                hc + 32
            } else {
                hc
            };
            let nl = if nc >= b'A' && nc <= b'Z' {
                nc + 32
            } else {
                nc
            };
            if hl != nl {
                ok = false;
                break;
            }
            j += 1;
        }
        if ok {
            let mut k = i + bb.len();
            while k < hb.len() && (hb[k] == b'"' || hb[k] == b' ') {
                k += 1;
            }
            if k < hb.len() && hb[k] == b'(' {
                return true;
            }
        }
        i += 1;
    }
    false
}
pub fn is_cacheable(sql: &str) -> bool {
    const VOLATILE_BASES: &[&str] = &[
        "random",
        "uuid",
        "gen_random_uuid",
        "now",
        "today",
        "transaction_timestamp",
        "statement_timestamp",
        "clock_timestamp",
    ];
    const VOLATILE_TOKENS: &[&str] = &[
        "current_date",
        "current_time",
        "current_timestamp",
        "current_localtimestamp",
        "localdate",
        "localtime",
        "localtimestamp",
        "duckdb_",
        "information_schema",
        "pg_catalog",
        "pg_",
        "sqlite_",
        "pragma ",
        "pragma_",
        "show ",
        "describe ",
        "summarize ",
    ];
    for base in VOLATILE_BASES {
        if contains_base_paren(sql, base) {
            return false;
        }
    }
    for term in VOLATILE_TOKENS {
        if contains_case_insensitive(sql, term) {
            return false;
        }
    }
    true
}
