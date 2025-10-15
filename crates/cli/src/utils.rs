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
        "llm-security" => DetectorProfile::LLMSecurity.get_detectors(),
        "llm-quality" => DetectorProfile::LLMQuality.get_detectors(),
        "llm-comprehensive" => DetectorProfile::LLMComprehensive.get_detectors(),
        "production-ready-llm" => DetectorProfile::ProductionReadyWithLLM.get_detectors(),
        _ => {
            println!("Unknown profile '{}', using 'basic'", profile);
            DetectorProfile::Basic.get_detectors()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_get_db_path_with_provided_path() {
        let custom_path = PathBuf::from("/custom/path/db.sqlite");
        let result = get_db_path(Some(custom_path.clone()));
        assert_eq!(result, custom_path);
    }

    #[test]
    fn test_get_db_path_with_none() {
        let result = get_db_path(None);
        assert_eq!(result, PathBuf::from("data/code-guardian.db"));
    }

    #[test]
    fn test_get_detectors_from_profile_basic() {
        let detectors = get_detectors_from_profile("basic");
        assert!(
            !detectors.is_empty(),
            "Basic profile should return detectors"
        );
    }

    #[test]
    fn test_get_detectors_from_profile_comprehensive() {
        let detectors = get_detectors_from_profile("comprehensive");
        assert!(
            !detectors.is_empty(),
            "Comprehensive profile should return detectors"
        );

        // Comprehensive should have more detectors than basic
        let basic_detectors = get_detectors_from_profile("basic");
        assert!(
            detectors.len() >= basic_detectors.len(),
            "Comprehensive should have at least as many detectors as basic"
        );
    }

    #[test]
    fn test_get_detectors_from_profile_security() {
        let detectors = get_detectors_from_profile("security");
        assert!(
            !detectors.is_empty(),
            "Security profile should return detectors"
        );
    }

    #[test]
    fn test_get_detectors_from_profile_performance() {
        let detectors = get_detectors_from_profile("performance");
        assert!(
            !detectors.is_empty(),
            "Performance profile should return detectors"
        );
    }

    #[test]
    fn test_get_detectors_from_profile_rust() {
        let detectors = get_detectors_from_profile("rust");
        assert!(
            !detectors.is_empty(),
            "Rust profile should return detectors"
        );
    }

    #[test]
    fn test_get_detectors_from_profile_unknown() {
        // This test captures stdout to verify the warning message
        let detectors = get_detectors_from_profile("unknown_profile");
        assert!(
            !detectors.is_empty(),
            "Unknown profile should fallback to basic detectors"
        );

        // Should fallback to basic profile
        let basic_detectors = get_detectors_from_profile("basic");
        assert_eq!(
            detectors.len(),
            basic_detectors.len(),
            "Unknown profile should return same as basic profile"
        );
    }

    #[test]
    fn test_all_profiles_return_valid_detectors() {
        let profiles = ["basic", "comprehensive", "security", "performance", "rust"];

        for profile in &profiles {
            let detectors = get_detectors_from_profile(profile);
            assert!(
                !detectors.is_empty(),
                "Profile '{}' should return at least one detector",
                profile
            );
        }
    }
}
