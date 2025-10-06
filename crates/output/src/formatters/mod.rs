use code_guardian_core::Match;

/// Trait for formatting a list of matches into a string representation.
/// Implementors should define how to convert matches into various output formats.
pub trait Formatter {
    /// Formats the given matches into a string.
    /// Returns the formatted string.
    fn format(&self, matches: &[Match]) -> String;
}

pub mod csv;
pub mod html;
pub mod json;
pub mod markdown;
pub mod text;

pub use csv::CsvFormatter;
pub use html::HtmlFormatter;
pub use json::JsonFormatter;
pub use markdown::MarkdownFormatter;
pub use text::TextFormatter;
