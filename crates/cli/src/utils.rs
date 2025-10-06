use code_guardian_core::{DetectorProfile, PatternDetector};
use std::path::PathBuf;

/// Get the database path, defaulting to "data/code-guardian.db" if not provided.
pub fn get_db_path(db: Option<PathBuf>) -> PathBuf {
    db.unwrap_or_else(|| PathBuf::from("data/code-guardian.db"))
}

/// Get detectors based on the profile string.
pub fn get_detectors_from_profile(profile: &str) -> Vec<Box<dyn PatternDetector>> {
    match profile {
        "basic" => DetectorProfile::Basic.get_detectors(),
        "comprehensive" => DetectorProfile::Comprehensive.get_detectors(),
        "security" => DetectorProfile::Security.get_detectors(),
        "performance" => DetectorProfile::Performance.get_detectors(),
        "rust" => DetectorProfile::Rust.get_detectors(),
        _ => {
            println!("Unknown profile '{}', using 'basic'", profile);
            DetectorProfile::Basic.get_detectors()
        }
    }
}

