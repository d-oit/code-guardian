use crate::detectors::*;
use crate::enhanced_config::{DetectorType, EnhancedScanConfig};
use crate::llm_detectors::*;
use crate::PatternDetector;
use anyhow::Result;

/// Factory for creating pattern detectors based on configuration
pub struct DetectorFactory;

impl DetectorFactory {
    /// Create all enabled detectors from configuration
    pub fn create_detectors(config: &EnhancedScanConfig) -> Vec<Box<dyn PatternDetector>> {
        let mut detectors = Vec::new();
        for detector_type in &config.enabled_detectors {
            match Self::create_detector(detector_type, Some(config)) {
                Ok(Some(detector)) => detectors.push(detector),
                Ok(None) => {} // Detector type not supported or disabled
                Err(e) => eprintln!(
                    "Warning: Failed to create detector for {:?}: {}",
                    detector_type, e
                ),
            }
        }
        detectors
    }

    /// Create a default set of detectors (backwards compatibility)
    pub fn create_default_detectors() -> Vec<Box<dyn PatternDetector>> {
        vec![Box::new(TodoDetector), Box::new(FixmeDetector)]
    }

    /// Create an extended set of detectors for comprehensive scanning
    pub fn create_comprehensive_detectors() -> Vec<Box<dyn PatternDetector>> {
        vec![
            // Comment patterns
            Box::new(TodoDetector),
            Box::new(FixmeDetector),
            Box::new(HackDetector),
            Box::new(BugDetector),
            Box::new(XxxDetector),
            Box::new(NoteDetector),
            Box::new(WarningDetector),
            // Rust-specific patterns
            Box::new(PanicDetector),
            Box::new(UnwrapDetector),
            Box::new(ExpectDetector),
            Box::new(UnimplementedDetector),
            Box::new(UnreachableDetector),
            // Performance patterns
            Box::new(CloneDetector),
            Box::new(ToStringDetector),
            // Security patterns
            Box::new(UnsafeDetector),
            // Development/Phase patterns
            Box::new(DevDetector),
            Box::new(DebugDetector),
            Box::new(TestDetector),
            Box::new(PhaseDetector),
            Box::new(StagingDetector),
            // Non-production code patterns
            Box::new(ConsoleLogDetector),
            Box::new(PrintDetector),
            Box::new(AlertDetector),
            Box::new(DebuggerDetector),
            Box::new(UnusedVarDetector),
            Box::new(DeadCodeDetector),
            Box::new(ExperimentalDetector),
        ]
    }

    /// Create detectors specifically for finding non-production code
    pub fn create_production_ready_detectors() -> Vec<Box<dyn PatternDetector>> {
        vec![
            // Development/Phase patterns
            Box::new(DevDetector),
            Box::new(DebugDetector),
            Box::new(TestDetector),
            Box::new(PhaseDetector),
            Box::new(StagingDetector),
            // Non-production code patterns
            Box::new(ConsoleLogDetector),
            Box::new(PrintDetector),
            Box::new(AlertDetector),
            Box::new(DebuggerDetector),
            Box::new(UnusedVarDetector),
            Box::new(DeadCodeDetector),
            Box::new(ExperimentalDetector),
            // Critical issues that shouldn't be in production
            Box::new(PanicDetector),
            Box::new(UnwrapDetector),
            Box::new(UnsafeDetector),
        ]
    }

    /// Create security-focused detectors
    pub fn create_security_detectors() -> Vec<Box<dyn PatternDetector>> {
        vec![
            Box::new(UnsafeDetector),
            Box::new(PanicDetector),
            Box::new(UnwrapDetector),
            Box::new(ExpectDetector),
        ]
    }

    /// Create LLM-specific vulnerability detectors
    pub fn create_llm_security_detectors() -> Vec<Box<dyn PatternDetector>> {
        vec![
            Box::new(HallucinatedApiDetector),
            Box::new(LLMSQLInjectionDetector),
            Box::new(InsecureRandomDetector),
            Box::new(HardcodedCredentialsDetector),
            Box::new(RustMemorySafetyDetector),
            Box::new(CryptoAntipatternDetector),
            Box::new(XSSInjectionDetector),
            Box::new(FilesystemSecurityDetector),
            Box::new(ContextConfusionDetector),
        ]
    }

    /// Create comprehensive LLM detectors (all LLM-related patterns)
    pub fn create_llm_comprehensive_detectors() -> Vec<Box<dyn PatternDetector>> {
        vec![Box::new(ComprehensiveLLMDetector::new())]
    }

    /// Create LLM performance and quality detectors
    pub fn create_llm_quality_detectors() -> Vec<Box<dyn PatternDetector>> {
        vec![
            Box::new(AsyncAntipatternDetector),
            Box::new(PerformanceAntipatternDetector),
            Box::new(ErrorHandlingDetector),
            Box::new(OverengineeringDetector),
            Box::new(ConfigAntipatternDetector),
            Box::new(DatabaseAntipatternDetector),
            Box::new(JSLLMIssuesDetector),
            Box::new(PythonLLMIssuesDetector),
        ]
    }

    /// Create detectors for production-ready scanning including LLM issues
    pub fn create_production_ready_with_llm_detectors() -> Vec<Box<dyn PatternDetector>> {
        let mut detectors = Self::create_production_ready_detectors();
        detectors.extend(Self::create_llm_security_detectors());
        detectors.extend(Self::create_llm_quality_detectors());
        detectors.push(Box::new(LLMGeneratedCommentsDetector));
        detectors
    }

    /// Create performance-focused detectors
    pub fn create_performance_detectors() -> Vec<Box<dyn PatternDetector>> {
        vec![
            Box::new(CloneDetector),
            Box::new(ToStringDetector),
            Box::new(UnwrapDetector), // Can cause performance issues
        ]
    }

    /// Create a single detector by type
    fn create_detector(
        detector_type: &DetectorType,
        config: Option<&EnhancedScanConfig>,
    ) -> Result<Option<Box<dyn PatternDetector>>> {
        match detector_type {
            DetectorType::Todo => Ok(Some(Box::new(TodoDetector))),
            DetectorType::Fixme => Ok(Some(Box::new(FixmeDetector))),
            DetectorType::Hack => Ok(Some(Box::new(HackDetector))),
            DetectorType::Bug => Ok(Some(Box::new(BugDetector))),
            DetectorType::Xxx => Ok(Some(Box::new(XxxDetector))),
            DetectorType::Note => Ok(Some(Box::new(NoteDetector))),
            DetectorType::Warning => Ok(Some(Box::new(WarningDetector))),
            DetectorType::Panic => Ok(Some(Box::new(PanicDetector))),
            DetectorType::Unwrap => Ok(Some(Box::new(UnwrapDetector))),
            DetectorType::Expect => Ok(Some(Box::new(ExpectDetector))),
            DetectorType::Unimplemented => Ok(Some(Box::new(UnimplementedDetector))),
            DetectorType::Unreachable => Ok(Some(Box::new(UnreachableDetector))),
            DetectorType::Clone => Ok(Some(Box::new(CloneDetector))),
            DetectorType::ToString => Ok(Some(Box::new(ToStringDetector))),
            DetectorType::Unsafe => Ok(Some(Box::new(UnsafeDetector))),

            // Development/Phase patterns
            DetectorType::Dev => Ok(Some(Box::new(DevDetector))),
            DetectorType::Debug => Ok(Some(Box::new(DebugDetector))),
            DetectorType::Test => Ok(Some(Box::new(TestDetector))),
            DetectorType::Phase => Ok(Some(Box::new(PhaseDetector))),
            DetectorType::Staging => Ok(Some(Box::new(StagingDetector))),

            // Non-production code patterns
            DetectorType::ConsoleLog => Ok(Some(Box::new(ConsoleLogDetector))),
            DetectorType::Print => Ok(Some(Box::new(PrintDetector))),
            DetectorType::Alert => Ok(Some(Box::new(AlertDetector))),
            DetectorType::Debugger => Ok(Some(Box::new(DebuggerDetector))),
            DetectorType::UnusedVar => Ok(Some(Box::new(UnusedVarDetector))),
            DetectorType::DeadCode => Ok(Some(Box::new(DeadCodeDetector))),
            DetectorType::Experimental => Ok(Some(Box::new(ExperimentalDetector))),

            // LLM-specific security patterns
            DetectorType::LLMHallucinatedApi => Ok(Some(Box::new(HallucinatedApiDetector))),
            DetectorType::LLMSQLInjection => Ok(Some(Box::new(LLMSQLInjectionDetector))),
            DetectorType::LLMInsecureRandom => Ok(Some(Box::new(InsecureRandomDetector))),
            DetectorType::LLMHardcodedCredentials => {
                Ok(Some(Box::new(HardcodedCredentialsDetector)))
            }
            DetectorType::LLMRustMemorySafety => Ok(Some(Box::new(RustMemorySafetyDetector))),
            DetectorType::LLMCryptoAntipattern => Ok(Some(Box::new(CryptoAntipatternDetector))),
            DetectorType::LLMXSSInjection => Ok(Some(Box::new(XSSInjectionDetector))),
            DetectorType::LLMFilesystemSecurity => Ok(Some(Box::new(FilesystemSecurityDetector))),
            DetectorType::LLMContextConfusion => Ok(Some(Box::new(ContextConfusionDetector))),

            // LLM-specific quality patterns
            DetectorType::LLMAsyncAntipattern => Ok(Some(Box::new(AsyncAntipatternDetector))),
            DetectorType::LLMPerformanceIssue => Ok(Some(Box::new(PerformanceAntipatternDetector))),
            DetectorType::LLMErrorHandling => Ok(Some(Box::new(ErrorHandlingDetector))),
            DetectorType::LLMOverengineering => Ok(Some(Box::new(OverengineeringDetector))),
            DetectorType::LLMConfigAntipattern => Ok(Some(Box::new(ConfigAntipatternDetector))),
            DetectorType::LLMDatabaseAntipattern => Ok(Some(Box::new(DatabaseAntipatternDetector))),
            DetectorType::LLMJSIssues => Ok(Some(Box::new(JSLLMIssuesDetector))),
            DetectorType::LLMPythonIssues => Ok(Some(Box::new(PythonLLMIssuesDetector))),
            DetectorType::LLMGeneratedComments => Ok(Some(Box::new(LLMGeneratedCommentsDetector))),

            // Advanced LLM-specific patterns
            DetectorType::LLMAIModelHallucination => {
                Ok(Some(Box::new(AIModelHallucinationDetector)))
            }
            DetectorType::LLMIncorrectAsync => Ok(Some(Box::new(IncorrectAsyncDetector))),
            DetectorType::LLMSecurityAntipattern => {
                Ok(Some(Box::new(LLMSecurityAntipatternDetector)))
            }
            DetectorType::LLMDBAntipattern => Ok(Some(Box::new(LLMDBAntipatternDetector))),
            DetectorType::LLMErrorHandlingMistake => {
                Ok(Some(Box::new(LLMErrorHandlingMistakesDetector)))
            }
            DetectorType::LLMPerformanceMistake => {
                Ok(Some(Box::new(LLMPerformanceMistakesDetector)))
            }
            DetectorType::LLMTypeMistake => Ok(Some(Box::new(LLMTypeMistakesDetector))),

            // Comprehensive LLM detector
            DetectorType::LLMComprehensive => Ok(Some(Box::new(ComprehensiveLLMDetector::new()))),

            DetectorType::Custom(name) => {
                if let Some(config) = config {
                    if let Some(pattern) = config.custom_patterns.get(name) {
                        let detector = CustomPatternDetector::new(name, pattern)?;
                        Ok(Some(Box::new(detector)))
                    } else {
                        Ok(None) // Pattern not found in config
                    }
                } else {
                    Ok(None) // No config provided
                }
            }
        }
    }
}

/// Predefined detector profiles for common use cases
pub enum DetectorProfile {
    /// Basic TODO/FIXME detection
    Basic,
    /// All available detectors
    Comprehensive,
    /// Security-focused scanning
    Security,
    /// Performance-focused scanning
    Performance,
    /// Rust-specific patterns only
    Rust,
    /// Production-readiness scanning (finds non-production code)
    ProductionReady,
    /// LLM security vulnerabilities only
    LLMSecurity,
    /// LLM quality issues only
    LLMQuality,
    /// All LLM-related patterns
    LLMComprehensive,
    /// Production-ready with LLM detection
    ProductionReadyWithLLM,
    /// Custom configuration
    Custom(Box<EnhancedScanConfig>),
}

impl DetectorProfile {
    /// Get detectors for the specified profile
    pub fn get_detectors(&self) -> Vec<Box<dyn PatternDetector>> {
        match self {
            DetectorProfile::Basic => DetectorFactory::create_default_detectors(),
            DetectorProfile::Comprehensive => DetectorFactory::create_comprehensive_detectors(),
            DetectorProfile::Security => DetectorFactory::create_security_detectors(),
            DetectorProfile::Performance => DetectorFactory::create_performance_detectors(),
            DetectorProfile::ProductionReady => {
                DetectorFactory::create_production_ready_detectors()
            }
            DetectorProfile::LLMSecurity => DetectorFactory::create_llm_security_detectors(),
            DetectorProfile::LLMQuality => DetectorFactory::create_llm_quality_detectors(),
            DetectorProfile::LLMComprehensive => {
                DetectorFactory::create_llm_comprehensive_detectors()
            }
            DetectorProfile::ProductionReadyWithLLM => {
                DetectorFactory::create_production_ready_with_llm_detectors()
            }
            DetectorProfile::Rust => vec![
                Box::new(PanicDetector),
                Box::new(UnwrapDetector),
                Box::new(ExpectDetector),
                Box::new(UnimplementedDetector),
                Box::new(UnreachableDetector),
                Box::new(CloneDetector),
                Box::new(ToStringDetector),
                Box::new(UnsafeDetector),
            ],
            DetectorProfile::Custom(config) => DetectorFactory::create_detectors(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_detectors() {
        let detectors = DetectorFactory::create_default_detectors();
        assert_eq!(detectors.len(), 2);
    }

    #[test]
    fn test_comprehensive_detectors() {
        let detectors = DetectorFactory::create_comprehensive_detectors();
        assert!(detectors.len() > 10);
    }

    #[test]
    fn test_security_detectors() {
        let detectors = DetectorFactory::create_security_detectors();
        assert!(detectors.len() >= 4);
    }

    #[test]
    fn test_detector_profiles() {
        let basic = DetectorProfile::Basic.get_detectors();
        let comprehensive = DetectorProfile::Comprehensive.get_detectors();

        assert!(comprehensive.len() > basic.len());
    }

    #[test]
    fn test_factory_with_custom_detectors() {
        let mut config = EnhancedScanConfig::default();
        config
            .custom_patterns
            .insert("MY_PATTERN".to_string(), r"custom".to_string());
        config
            .enabled_detectors
            .push(DetectorType::Custom("MY_PATTERN".to_string()));

        let detectors = DetectorFactory::create_detectors(&config);
        assert!(!detectors.is_empty());
        // The default config has 2 detectors, plus our custom one
        assert!(detectors.len() >= 3);
    }

    #[test]
    fn test_custom_detector_creation_success() {
        let mut config = EnhancedScanConfig::default();
        config
            .custom_patterns
            .insert("TEST".to_string(), r"test".to_string());

        let result = DetectorFactory::create_detector(
            &DetectorType::Custom("TEST".to_string()),
            Some(&config),
        );
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_custom_detector_creation_missing_pattern() {
        let config = EnhancedScanConfig::default();

        let result = DetectorFactory::create_detector(
            &DetectorType::Custom("MISSING".to_string()),
            Some(&config),
        );
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_custom_detector_creation_no_config() {
        let result =
            DetectorFactory::create_detector(&DetectorType::Custom("TEST".to_string()), None);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_custom_detector_invalid_regex() {
        let mut config = EnhancedScanConfig::default();
        config
            .custom_patterns
            .insert("INVALID".to_string(), r"[invalid".to_string());
        config
            .enabled_detectors
            .push(DetectorType::Custom("INVALID".to_string()));

        let detectors = DetectorFactory::create_detectors(&config);
        // Should have default detectors but not the invalid custom one
        assert_eq!(detectors.len(), 2); // default has 2
    }
}
