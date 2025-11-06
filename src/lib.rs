pub mod entry;

pub use entry::*;

use std::fs::{self, OpenOptions};
use std::io::Read;
use std::path::Path;

/// Prepend a coffee entry to the journal file
/// Creates the file if it doesn't exist, or inserts at the top if it does
pub fn save_entry_to_file<P: AsRef<Path>>(entry: &CoffeeEntry, path: P) -> std::io::Result<()> {
    let path = path.as_ref();
    let new_content = entry.to_plaintext();

    // Read existing content if file exists
    let existing_content = if path.exists() {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents
    } else {
        String::new()
    };

    // Write new entry followed by existing content
    let full_content = format!("{}\n{}", new_content, existing_content);
    fs::write(path, full_content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_save_entry_to_new_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_cofj.txt");

        let timestamp = chrono::Local.with_ymd_and_hms(2025, 1, 15, 10, 0, 0).unwrap();
        let entry = CoffeeEntry::with_timestamp(
            timestamp,
            "Test Brand".to_string(),
            "Test Coffee".to_string(),
            "Test Origin".to_string(),
            ProcessingMethod::Washed,
            Roast::Medium,
            Preparation::Drip,
            7,
        ).unwrap();

        let result = save_entry_to_file(&entry, &file_path);
        assert!(result.is_ok());

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("2025-01-15 10:00:00"));
        assert!(content.contains("Brand: Test Brand"));
        assert!(content.contains("---"));
    }

    #[test]
    fn test_save_entry_prepends_to_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_cofj.txt");

        // Create first entry
        let timestamp1 = chrono::Local.with_ymd_and_hms(2025, 1, 15, 10, 0, 0).unwrap();
        let entry1 = CoffeeEntry::with_timestamp(
            timestamp1,
            "First Brand".to_string(),
            "First Coffee".to_string(),
            "First Origin".to_string(),
            ProcessingMethod::Natural,
            Roast::Light,
            Preparation::Pourover,
            8,
        ).unwrap();

        save_entry_to_file(&entry1, &file_path).unwrap();

        // Create second entry
        let timestamp2 = chrono::Local.with_ymd_and_hms(2025, 1, 15, 14, 0, 0).unwrap();
        let entry2 = CoffeeEntry::with_timestamp(
            timestamp2,
            "Second Brand".to_string(),
            "Second Coffee".to_string(),
            "Second Origin".to_string(),
            ProcessingMethod::Washed,
            Roast::Dark,
            Preparation::Espresso,
            9,
        ).unwrap();

        save_entry_to_file(&entry2, &file_path).unwrap();

        // Read final content
        let content = fs::read_to_string(&file_path).unwrap();

        // Second entry should appear first (newer entries at top)
        let second_pos = content.find("Second Brand").unwrap();
        let first_pos = content.find("First Brand").unwrap();
        assert!(second_pos < first_pos, "Newer entry should appear before older entry");

        // Both entries should be present
        assert!(content.contains("2025-01-15 14:00:00"));
        assert!(content.contains("2025-01-15 10:00:00"));
    }

    #[test]
    fn test_file_created_if_not_exists() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("new_cofj.txt");

        assert!(!file_path.exists());

        let timestamp = chrono::Local.with_ymd_and_hms(2025, 1, 15, 12, 0, 0).unwrap();
        let entry = CoffeeEntry::with_timestamp(
            timestamp,
            "Brand".to_string(),
            "Coffee".to_string(),
            "Origin".to_string(),
            ProcessingMethod::AnerobicFermentation,
            Roast::Medium,
            Preparation::Cortado,
            6,
        ).unwrap();

        save_entry_to_file(&entry, &file_path).unwrap();

        assert!(file_path.exists());
    }
}
