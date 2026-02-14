/*
    Common utilities and types used across examples
*/

// Third-party dependencies
use colored::Colorize;
    
/// Print a colored section header
pub fn print_header(text: &str) {
    println!("\n{}", text.bright_cyan().bold());
    println!("{}", "=".repeat(text.len()).bright_cyan());
}

/// Print a success message
pub fn print_success(text: &str) {
    println!("{} {}", "✓".green(), text);
}

/// Print an info message
pub fn print_info(text: &str) {
    println!("{} {}", "ℹ".blue(), text);
}

/// Print a warning message
pub fn print_warning(text: &str) {
    println!("{} {}", "⚠".yellow(), text);
}
