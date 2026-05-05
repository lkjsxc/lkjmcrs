use serde::Serialize;
use std::{fs, path::Path};
use thiserror::Error;

const DOC_LIMIT: usize = 300;
const SRC_LIMIT: usize = 200;

#[derive(Debug, Error)]
pub enum QualityError {
    #[error("quality check failed:\n{0}")]
    Failed(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[derive(Serialize)]
struct LineReport {
    status: &'static str,
    docs_max: usize,
    src_max: usize,
    violations: Vec<String>,
}

pub fn validate_docs_topology() -> Result<(), QualityError> {
    let mut failures = Vec::new();
    visit_dirs(Path::new("docs"), &mut |dir| {
        let readme = dir.join("README.md");
        if !readme.exists() {
            failures.push(format!("missing README.md: {}", dir.display()));
        }
        let child_count = fs::read_dir(dir)?
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name() != "README.md")
            .count();
        if child_count < 2 {
            failures.push(format!("needs >=2 children: {}", dir.display()));
        }
        Ok(())
    })?;
    finish(failures, "docs topology ok")
}

pub fn check_lines() -> Result<(), QualityError> {
    let mut report = LineReport {
        status: "pass",
        docs_max: 0,
        src_max: 0,
        violations: Vec::new(),
    };
    check_tree(
        Path::new("docs"),
        DOC_LIMIT,
        &mut report.docs_max,
        &mut report.violations,
    )?;
    check_tree(
        Path::new("src"),
        SRC_LIMIT,
        &mut report.src_max,
        &mut report.violations,
    )?;
    if !report.violations.is_empty() {
        report.status = "fail";
    }
    println!("{}", serde_json::to_string(&report)?);
    finish(report.violations, "line limits ok")
}

fn visit_dirs(
    dir: &Path,
    action: &mut dyn FnMut(&Path) -> Result<(), QualityError>,
) -> Result<(), QualityError> {
    action(dir)?;
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            visit_dirs(&path, action)?;
        }
    }
    Ok(())
}

fn check_tree(
    dir: &Path,
    limit: usize,
    max_seen: &mut usize,
    failures: &mut Vec<String>,
) -> Result<(), QualityError> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            check_tree(&path, limit, max_seen, failures)?;
        } else if should_check(&path) {
            let lines = fs::read_to_string(&path)?.lines().count();
            *max_seen = (*max_seen).max(lines);
            if lines > limit {
                failures.push(format!("{}: {}>{}", path.display(), lines, limit));
            }
        }
    }
    Ok(())
}

fn should_check(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("md") | Some("rs")
    )
}

fn finish(failures: Vec<String>, success: &str) -> Result<(), QualityError> {
    if failures.is_empty() {
        println!("{success}");
        Ok(())
    } else {
        Err(QualityError::Failed(failures.join("\n")))
    }
}
