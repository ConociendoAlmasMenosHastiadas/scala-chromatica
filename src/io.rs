//! ColorMap Save/Load System
//!
//! This module handles persistent storage of ColorMaps using JSON files.
//! It supports both built-in default colormaps (embedded at compile time)
//! and user-created custom colormaps (stored in platform-specific directories).
//!
//! # Architecture
//! - Built-in colormaps are embedded in the binary using `include_str!`
//! - Custom colormaps are stored in OS-appropriate config directories
//! - Automatic directory creation and error handling
//!
//! # Usage
//! ```
//! use scala_chromatica::io;
//! use scala_chromatica::ColorMap;
//!
//! // Load a built-in colormap
//! let fire = io::load_builtin_colormap("Fire").unwrap();
//!
//! // Create and save a custom colormap
//! let custom = ColorMap::new("MyCustom");
//! // io::save_colormap(&custom).unwrap(); // Commented - would write to disk
//!
//! // Load a custom colormap
//! // let custom = io::load_custom_colormap("MyCustom").unwrap();
//!
//! // List all available colormaps
//! let all = io::list_available_colormaps().unwrap();
//! ```

use crate::colormap::ColorMap;
use crate::error::{ColorMapError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

/// Macro to define builtin colormaps with automatic list generation
macro_rules! define_builtin_colormaps {
    ($($name:literal => $const_name:ident => $file:literal),* $(,)?) => {
        $(
            const $const_name: &str = include_str!($file);
        )*

        /// Get list of all builtin colormap names
        fn get_builtin_colormap_names() -> &'static [&'static str] {
            &[$($name),*]
        }

        /// Load a builtin colormap by name
        fn load_builtin_impl(name: &str) -> Option<&'static str> {
            match name {
                $($name => Some($const_name),)*
                _ => None,
            }
        }

        /// Check if a colormap name is builtin
        fn is_builtin_impl(name: &str) -> bool {
            matches!(name, $($name)|*)
        }
    };
}

// Define all builtin colormaps in one place
define_builtin_colormaps! {
    "Default" => DEFAULT_COLORMAP_JSON => "colormaps/default.json",
    "Fire" => FIRE_COLORMAP_JSON => "colormaps/fire.json",
    "Ocean" => OCEAN_COLORMAP_JSON => "colormaps/ocean.json",
    "Grayscale" => GRAYSCALE_COLORMAP_JSON => "colormaps/grayscale.json",
    "Rainbow" => RAINBOW_COLORMAP_JSON => "colormaps/rainbow.json",
    "Academic" => ACADEMIC_COLORMAP_JSON => "colormaps/academic.json",
    "Twilight Garden" => TWILIGHT_GARDEN_COLORMAP_JSON => "colormaps/twilight_garden.json",
    "Coral Sunset" => CORAL_SUNSET_COLORMAP_JSON => "colormaps/coral_sunset.json",
    "Olive Symmetry" => OLIVE_SYMMETRY_COLORMAP_JSON => "colormaps/olive_symmetry.json",
    "Orchid Garden" => ORCHID_GARDEN_COLORMAP_JSON => "colormaps/orchid_garden.json",
    "Frozen Amaranth" => FROZEN_AMARANTH_COLORMAP_JSON => "colormaps/frozen_amaranth.json",
    "Electric Neon" => ELECTRIC_NEON_COLORMAP_JSON => "colormaps/electric_neon.json",
    "Cosmic Dawn" => COSMIC_DAWN_COLORMAP_JSON => "colormaps/cosmic_dawn.json",
    "Vintage Lavender" => VINTAGE_LAVENDER_COLORMAP_JSON => "colormaps/vintage_lavender.json",
    "Spring Meadow" => SPRING_MEADOW_COLORMAP_JSON => "colormaps/spring_meadow.json",
}

/// Get the directory where custom colormaps are stored
/// Returns platform-specific config directory:
/// - Windows: %APPDATA%\scala-chromatica\colormaps\
/// - Linux: ~/.config/scala-chromatica/colormaps/
/// - macOS: ~/Library/Application Support/scala-chromatica/colormaps/
pub fn get_colormaps_directory() -> Result<PathBuf> {
    let base_dir = directories::ProjectDirs::from("", "", "scala-chromatica")
        .ok_or(ColorMapError::NoConfigDirectory)?;

    let colormaps_dir = base_dir.config_dir().join("colormaps");

    // Create directory if it doesn't exist
    if !colormaps_dir.exists() {
        fs::create_dir_all(&colormaps_dir)?;
    }

    Ok(colormaps_dir)
}

/// Load a built-in colormap by name
///
/// Available built-in colormaps:
/// - Default, Fire, Ocean, Grayscale, Rainbow
/// - Academic, Twilight Garden, Coral Sunset
/// - Olive Symmetry, Orchid Garden, Frozen Amaranth
/// - Electric Neon, Cosmic Dawn, Vintage Lavender
pub fn load_builtin_colormap(name: &str) -> Result<ColorMap> {
    let json_str =
        load_builtin_impl(name).ok_or_else(|| ColorMapError::NotFound(name.to_string()))?;

    let colormap: ColorMap = serde_json::from_str(json_str)?;
    Ok(colormap)
}

/// Check if a colormap is a built-in default
pub fn is_builtin_colormap(name: &str) -> bool {
    is_builtin_impl(name)
}

/// Save a colormap to the custom colormaps directory
/// This will create a JSON file named "{colormap.name}.json"
///
/// Returns the path to the saved file
pub fn save_colormap(colormap: &ColorMap) -> Result<PathBuf> {
    let dir = get_colormaps_directory()?;
    let filename = format!("{}.json", colormap.name);
    let filepath = dir.join(&filename);

    let json = serde_json::to_string_pretty(colormap)?;
    fs::write(&filepath, json)?;

    Ok(filepath)
}

/// Load a custom colormap from the colormaps directory
pub fn load_custom_colormap(name: &str) -> Result<ColorMap> {
    let dir = get_colormaps_directory()?;
    let filename = format!("{}.json", name);
    let filepath = dir.join(&filename);

    if !filepath.exists() {
        return Err(ColorMapError::NotFound(name.to_string()));
    }

    let json = fs::read_to_string(&filepath)?;
    let colormap: ColorMap = serde_json::from_str(&json)?;

    Ok(colormap)
}

/// Load a colormap by name, checking built-ins first, then custom colormaps
pub fn load_colormap(name: &str) -> Result<ColorMap> {
    // Try built-in first
    if is_builtin_colormap(name) {
        return load_builtin_colormap(name);
    }

    // Try custom
    load_custom_colormap(name)
}

/// Delete a custom colormap
/// Note: Built-in colormaps cannot be deleted
pub fn delete_custom_colormap(name: &str) -> Result<()> {
    if is_builtin_colormap(name) {
        return Err(ColorMapError::IoError(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Cannot delete built-in colormaps",
        )));
    }

    let dir = get_colormaps_directory()?;
    let filename = format!("{}.json", name);
    let filepath = dir.join(&filename);

    if !filepath.exists() {
        return Err(ColorMapError::NotFound(name.to_string()));
    }

    fs::remove_file(&filepath)?;
    Ok(())
}

/// Information about an available colormap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorMapInfo {
    pub name: String,
    pub is_builtin: bool,
    pub filepath: Option<PathBuf>,
}

/// List all available colormaps (built-in + custom)
pub fn list_available_colormaps() -> Result<Vec<ColorMapInfo>> {
    let mut colormaps = Vec::new();

    // Add built-in colormaps
    for name in get_builtin_colormap_names() {
        colormaps.push(ColorMapInfo {
            name: name.to_string(),
            is_builtin: true,
            filepath: None,
        });
    }

    // Add custom colormaps
    let dir = get_colormaps_directory()?;
    if dir.exists() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    // Skip if it has the same name as a built-in (built-ins take precedence)
                    if !is_builtin_colormap(stem) {
                        colormaps.push(ColorMapInfo {
                            name: stem.to_string(),
                            is_builtin: false,
                            filepath: Some(path),
                        });
                    }
                }
            }
        }
    }

    Ok(colormaps)
}

/// Export a built-in colormap to the custom colormaps directory
/// This allows users to create modified versions of built-in colormaps
pub fn export_builtin_colormap(name: &str) -> Result<PathBuf> {
    let colormap = load_builtin_colormap(name)?;
    save_colormap(&colormap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_builtin_colormaps() {
        // Test loading each built-in colormap
        for name in &[
            "Default",
            "Fire",
            "Ocean",
            "Grayscale",
            "Rainbow",
            "Academic",
            "Twilight Garden",
            "Coral Sunset",
            "Olive Symmetry",
            "Orchid Garden",
        ] {
            let result = load_builtin_colormap(name);
            assert!(result.is_ok(), "Failed to load {}: {:?}", name, result);

            let colormap = result.unwrap();
            assert_eq!(colormap.name, *name);
            assert!(!colormap.stops.is_empty());
        }
    }

    #[test]
    fn test_load_nonexistent_builtin() {
        let result = load_builtin_colormap("NonExistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_builtin_colormap() {
        assert!(is_builtin_colormap("Fire"));
        assert!(is_builtin_colormap("Ocean"));
        assert!(is_builtin_colormap("Academic"));
        assert!(is_builtin_colormap("Orchid Garden"));
        assert!(!is_builtin_colormap("MyCustom"));
        assert!(!is_builtin_colormap(""));
    }
}
