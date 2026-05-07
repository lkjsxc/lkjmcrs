use crate::quality::QualityError;
use std::fs;
use std::path::Path;

pub fn check_boundaries(failures: &mut Vec<String>) -> Result<(), QualityError> {
    check_forbidden_imports(
        Path::new("src/protocol"),
        &[
            "crate::player",
            "crate::world",
            "crate::scheduler",
            "crate::session",
        ],
        "protocol import boundary",
        failures,
    )?;
    check_forbidden_imports(
        Path::new("src/probe"),
        &["crate::world"],
        "probe domain import boundary",
        failures,
    )
}

fn check_forbidden_imports(
    dir: &Path,
    forbidden: &[&str],
    label: &str,
    failures: &mut Vec<String>,
) -> Result<(), QualityError> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            check_forbidden_imports(&path, forbidden, label, failures)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            let contents = fs::read_to_string(&path)?;
            for needle in forbidden {
                if contents.contains(needle) {
                    failures.push(format!("{}: {} uses {}", label, path.display(), needle));
                }
            }
        }
    }
    Ok(())
}
