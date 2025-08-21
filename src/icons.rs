//! Provides functionality for selecting file-specific icons and colors.
//!
//! This module is responsible for mapping file paths to appropriate Nerd Font icons
//! and `colored` crate `Color` enums to enhance the visual output.

use colored::Color;
use std::path::Path;

/// Returns a Nerd Font icon and a display color for a given file path.
///
/// The selection logic first checks for special, well-known filenames. If no
/// special filename matches, it falls back to checking file extensions.
///
/// # Arguments
///
/// * `path` - A reference to the `Path` of the file or directory.
/// * `is_dir` - A boolean indicating if the `path` is a directory.
///
/// # Returns
///
/// A tuple containing:
/// * `String` - The Nerd Font icon character.
/// * `Color` - The `colored::Color` to use for displaying the icon.
pub fn get_icon_for_path(path: &Path, is_dir: bool) -> (String, Color) {
    if is_dir {
        return ("".to_string(), Color::Blue); // Folder icon
    }

    let icon = match path.file_name().and_then(|s| s.to_str()) {
        Some("Cargo.toml") => "",
        Some("Cargo.lock") => "",
        Some(".gitignore") | Some(".gitattributes") => "",
        Some("LICENSE") => "",
        Some("README.md") => "",
        Some("Dockerfile") => "",
        Some("Makefile") | Some("makefile") => "",
        _ => match path.extension().and_then(|s| s.to_str()) {
            Some("rs") => "",
            Some("py") => "",
            Some("js") => "",
            Some("ts") | Some("tsx") => "",
            Some("java") => "",
            Some("html") => "",
            Some("css") | Some("scss") => "",
            Some("toml") => "",
            Some("json") => "",
            Some("yaml") | Some("yml") => "󰗊",
            Some("zip") | Some("gz") | Some("tar") => "",
            Some("md") => "",
            Some("sh") | Some("bash") | Some("zsh") => "",
            _ => "", // Default file icon
        },
    };

    let color = match icon {
        "" | "" => Color::Red,
        "" | "" => Color::Yellow,
        "" => Color::BrightBlack,
        "" | "󰗊" => Color::BrightYellow,
        "" => Color::Yellow,
        _ => Color::White,
    };

    (icon.to_string(), color)
}

// Unit tests for the icon logic
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_directory_icon() {
        let path = Path::new("src");
        let (icon, color) = get_icon_for_path(path, true);
        assert_eq!(icon, "");
        assert_eq!(color, Color::Blue);
    }

    #[test]
    fn test_specific_filename_icon() {
        let path = Path::new("Cargo.toml");
        let (icon, color) = get_icon_for_path(path, false);
        assert_eq!(icon, "");
        assert_eq!(color, Color::BrightYellow);
    }

    #[test]
    fn test_rust_extension_icon() {
        let path = Path::new("main.rs");
        let (icon, color) = get_icon_for_path(path, false);
        assert_eq!(icon, "");
        assert_eq!(color, Color::Red);
    }

    #[test]
    fn test_default_file_icon() {
        let path = Path::new("some_random_file.xyz");
        let (icon, color) = get_icon_for_path(path, false);
        assert_eq!(icon, "");
        assert_eq!(color, Color::White);
    }
}
