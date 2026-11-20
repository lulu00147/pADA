use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;
const ENCODING_SCAN_SIZE: u64 = 256 * 1024;
pub fn is_valid_utf8(path: &Path) -> Result<bool, String> {
    let mut buf = Vec::with_capacity(ENCODING_SCAN_SIZE as usize);
    let file =
        fs::File::open(path).map_err(|e| format!("Cannot open file for encoding check: {}", e))?;
    file.take(ENCODING_SCAN_SIZE)
        .read_to_end(&mut buf)
        .map_err(|e| format!("Cannot read file for encoding check: {}", e))?;
    let window_ok = match std::str::from_utf8(&buf) {
        Ok(_) => true,
        Err(error) if error.error_len().is_none() => true,
        Err(_) => false,
    };
    if !window_ok {
        return Ok(false);
    }
    Ok(first_invalid_utf8_offset(path)?.is_none())
}
fn first_invalid_utf8_offset(path: &Path) -> Result<Option<u64>, String> {
    const CHUNK: usize = 1024 * 1024;
    let file = fs::File::open(path)
        .map_err(|e| format!("Cannot open file for full UTF-8 check: {}", e))?;
    let mut reader = std::io::BufReader::with_capacity(CHUNK, file);
    let mut buf = vec![0u8; CHUNK];
    let mut remainder = Vec::with_capacity(3);
    let mut remainder_offset = 0u64;
    let mut total_read = 0u64;
    loop {
        let n = reader
            .read(&mut buf)
            .map_err(|e| format!("Read error during UTF-8 check: {}", e))?;
        if n == 0 {
            if remainder.is_empty() {
                return Ok(None);
            }
            let valid_up_to = std::str::from_utf8(&remainder)
                .err()
                .map(|error| error.valid_up_to())
                .unwrap_or(0);
            return Ok(Some(remainder_offset + valid_up_to as u64));
        }
        remainder.extend_from_slice(&buf[..n]);
        total_read += n as u64;
        match std::str::from_utf8(&remainder) {
            Ok(_) => {
                remainder.clear();
                remainder_offset = total_read;
            }
            Err(error) if error.error_len().is_none() => {
                let valid_up_to = error.valid_up_to();
                remainder.drain(..valid_up_to);
                remainder_offset += valid_up_to as u64;
            }
            Err(error) => return Ok(Some(remainder_offset + error.valid_up_to() as u64)),
        }
    }
}
fn read_first_n(path: &Path, n: usize) -> Vec<u8> {
    let mut buf = Vec::with_capacity(n);
    if let Ok(mut f) = fs::File::open(path) {
        let _ = std::io::Read::take(&mut f, n as u64).read_to_end(&mut buf);
    }
    buf
}
pub fn ensure_utf8_inplace(path: &Path) -> Result<(), String> {
    if is_valid_utf8(path)? {
        return Ok(());
    }
    let max_bytes: u64 = 500 * 1024 * 1024;
    let meta = fs::metadata(path).map_err(|e| format!("Cannot stat file: {}", e))?;
    if meta.len() > max_bytes {
        return Err(format!(
            "File '{}' is too large for in-place encoding conversion ({} bytes, max {} bytes). \
             Use a tool like `iconv` offline to pre-convert it.",
            path.display(),
            meta.len(),
            max_bytes
        ));
    }
    let contents = fs::read(path).map_err(|e| format!("Cannot read file: {}", e))?;
    let cleaned =
        if contents.len() >= 3 && contents[0] == 0xEF && contents[1] == 0xBB && contents[2] == 0xBF
        {
            &contents[3..]
        } else {
            &contents
        };
    let (decoded, _, had_errors) = encoding_rs::GBK.decode(cleaned);
    if had_errors {
        let invalid_offset = first_invalid_utf8_offset(path)
            .ok()
            .flatten()
            .map(|offset| offset.to_string())
            .unwrap_or_else(|| "unknown".into());
        return Err(format!(
            "File '{}' is not valid UTF-8 nor GBK (unsupported encoding). \
             First invalid UTF-8 byte offset: {}. File start (first 64 bytes): {:02x?}. \
             Please wait until copying finishes or convert to UTF-8 before registering.",
            path.display(),
            invalid_offset,
            read_first_n(path, 64)
        ));
    }
    let temp_path = path.with_extension("utf8.tmp");
    let mut temp_file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp_path)
        .map_err(|e| {
            format!(
                "Cannot create temp file '{}': {} (remove it if it is a leftover from an \
                 interrupted conversion)",
                temp_path.display(),
                e
            )
        })?;
    temp_file
        .write_all(decoded.as_bytes())
        .map_err(|e| format!("Cannot write temp file '{}': {}", temp_path.display(), e))?;
    temp_file
        .sync_all()
        .map_err(|e| format!("Cannot sync temp file '{}': {}", temp_path.display(), e))?;
    drop(temp_file);
    if let Err(e) = fs::rename(&temp_path, path) {
        let _ = fs::remove_file(&temp_path);
        return Err(format!("Cannot replace file '{}': {}", path.display(), e));
    }
    tracing::info!(
        "Converted '{}' from GBK to UTF-8 (in-place)",
        path.display()
    );
    Ok(())
}
