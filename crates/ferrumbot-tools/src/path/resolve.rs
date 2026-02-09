use std::path::{Component, Path, PathBuf};

use anyhow::{Result, anyhow};

pub(crate) fn resolve_path(path: &str, allowed_dir: Option<&Path>) -> Result<PathBuf> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let expanded = ferrumbot_config::expand_tilde(path);
    let absolute = if expanded.is_absolute() {
        expanded
    } else {
        cwd.join(expanded)
    };
    let resolved = canonicalize_with_fallback(&absolute)?;

    if let Some(allowed_dir) = allowed_dir {
        let absolute_allowed = if allowed_dir.is_absolute() {
            allowed_dir.to_path_buf()
        } else {
            cwd.join(allowed_dir)
        };
        let allowed = canonicalize_with_fallback(&absolute_allowed)?;
        if !resolved.starts_with(&allowed) {
            return Err(anyhow!(
                "Path {path} is outside allowed directory {}",
                allowed.display()
            ));
        }
    }

    Ok(resolved)
}

fn canonicalize_with_fallback(path: &Path) -> Result<PathBuf> {
    if path.exists() {
        return Ok(path.canonicalize().unwrap_or_else(|_| normalize_path(path)));
    }

    let mut existing = path.to_path_buf();
    let mut tail = Vec::new();

    while !existing.exists() {
        let Some(name) = existing.file_name().map(|v| v.to_os_string()) else {
            return Ok(normalize_path(path));
        };
        tail.push(name);
        if !existing.pop() {
            return Ok(normalize_path(path));
        }
    }

    let mut resolved = existing
        .canonicalize()
        .unwrap_or_else(|_| normalize_path(&existing));

    for segment in tail.iter().rev() {
        resolved.push(segment);
    }

    Ok(normalize_path(&resolved))
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::resolve_path;

    fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock drift")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{}-{now}", std::process::id()))
    }

    #[test]
    fn resolve_path_blocks_escape_for_missing_targets() {
        let root = unique_temp_dir("ferrum-path-test");
        let allowed = root.join("allowed");
        fs::create_dir_all(&allowed).expect("create allowed dir");

        let escaped = allowed.join("..").join("outside.txt");
        let err = resolve_path(
            escaped.to_str().expect("utf8 path"),
            Some(allowed.as_path()),
        )
        .expect_err("expected outside allowed-dir error");

        assert!(err.to_string().contains("outside allowed directory"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn resolve_path_allows_missing_targets_inside_allowed_dir() {
        let root = unique_temp_dir("ferrum-path-test");
        let allowed = root.join("allowed");
        fs::create_dir_all(&allowed).expect("create allowed dir");

        let inside = allowed.join("nested").join("file.txt");
        let resolved = resolve_path(inside.to_str().expect("utf8 path"), Some(allowed.as_path()))
            .expect("path inside allowed dir should resolve");

        let canonical_allowed =
            super::canonicalize_with_fallback(&allowed).expect("canonical allowed dir");
        assert!(resolved.starts_with(canonical_allowed));
        let _ = fs::remove_dir_all(&root);
    }
}
