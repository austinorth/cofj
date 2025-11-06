use chrono::{DateTime, Local};
use std::fmt;

/// Processing methods for coffee beans
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingMethod {
    Washed,
    Natural,
    PulpedNatural,
    AnerobicFermentation,
}

/// Roast levels
#[derive(Debug, Clone, PartialEq)]
pub enum Roast {
    Light,
    Medium,
    Dark,
}

/// Preparation methods (coffee drinks)
#[derive(Debug, Clone, PartialEq)]
pub enum Preparation {
    Americano,
    Cortado,
    Drip,
    Espresso,
    Latte,
    Pourover,
}

/// A coffee journal entry
#[derive(Debug, Clone, PartialEq)]
pub struct CoffeeEntry {
    pub timestamp: DateTime<Local>,
    pub brand: String,
    pub name: String,
    pub origin: String,
    pub processing: ProcessingMethod,
    pub roast: Roast,
    pub preparation: Preparation,
    pub rating: u8,
}

impl CoffeeEntry {
    /// Create a new coffee entry with the current timestamp
    pub fn new(
        brand: String,
        name: String,
        origin: String,
        processing: ProcessingMethod,
        roast: Roast,
        preparation: Preparation,
        rating: u8,
    ) -> Result<Self, String> {
        if !(1..=10).contains(&rating) {
            return Err(format!("Rating must be between 1 and 10, got {}", rating));
        }

        Ok(CoffeeEntry {
            timestamp: Local::now(),
            brand,
            name,
            origin,
            processing,
            roast,
            preparation,
            rating,
        })
    }

    /// Create an entry with a specific timestamp (useful for testing)
    /// This is a builder-style method that takes the parameters and timestamp separately
    #[allow(clippy::too_many_arguments)]
    pub fn with_timestamp(
        timestamp: DateTime<Local>,
        brand: String,
        name: String,
        origin: String,
        processing: ProcessingMethod,
        roast: Roast,
        preparation: Preparation,
        rating: u8,
    ) -> Result<Self, String> {
        // Use the existing validation logic from new()
        if !(1..=10).contains(&rating) {
            return Err(format!("Rating must be between 1 and 10, got {}", rating));
        }

        Ok(CoffeeEntry {
            timestamp,
            brand,
            name,
            origin,
            processing,
            roast,
            preparation,
            rating,
        })
    }

    /// Format the entry as compact plaintext
    pub fn to_plaintext(&self) -> String {
        format!(
            "{}\n\
             Brand: {}\n\
             Name: {}\n\
             Origin: {}\n\
             Processing: {}\n\
             Roast: {}\n\
             Preparation: {}\n\
             Rating: {}/10\n\
             ---\n",
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
            self.brand,
            self.name,
            self.origin,
            self.processing,
            self.roast,
            self.preparation,
            self.rating
        )
    }
}

// Display implementations for enums
impl fmt::Display for ProcessingMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessingMethod::Washed => write!(f, "Washed"),
            ProcessingMethod::Natural => write!(f, "Natural"),
            ProcessingMethod::PulpedNatural => write!(f, "Pulped Natural"),
            ProcessingMethod::AnerobicFermentation => write!(f, "Anerobic Fermentation"),
        }
    }
}

impl fmt::Display for Roast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Roast::Light => write!(f, "Light"),
            Roast::Medium => write!(f, "Medium"),
            Roast::Dark => write!(f, "Dark"),
        }
    }
}

impl fmt::Display for Preparation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Preparation::Americano => write!(f, "Americano"),
            Preparation::Cortado => write!(f, "Cortado"),
            Preparation::Drip => write!(f, "Drip"),
            Preparation::Espresso => write!(f, "Espresso"),
            Preparation::Latte => write!(f, "Latte"),
            Preparation::Pourover => write!(f, "Pourover"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_coffee_entry_creation() {
        let entry = CoffeeEntry::new(
            "Blue Bottle".to_string(),
            "Ethiopia Yirgacheffe".to_string(),
            "Ethiopia".to_string(),
            ProcessingMethod::Washed,
            Roast::Light,
            Preparation::Pourover,
            8,
        );

        assert!(entry.is_ok());
        let entry = entry.unwrap();
        assert_eq!(entry.brand, "Blue Bottle");
        assert_eq!(entry.name, "Ethiopia Yirgacheffe");
        assert_eq!(entry.origin, "Ethiopia");
        assert_eq!(entry.processing, ProcessingMethod::Washed);
        assert_eq!(entry.roast, Roast::Light);
        assert_eq!(entry.preparation, Preparation::Pourover);
        assert_eq!(entry.rating, 8);
    }

    #[test]
    fn test_rating_validation_too_low() {
        let entry = CoffeeEntry::new(
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            ProcessingMethod::Washed,
            Roast::Medium,
            Preparation::Espresso,
            0,
        );

        assert!(entry.is_err());
        assert_eq!(entry.unwrap_err(), "Rating must be between 1 and 10, got 0");
    }

    #[test]
    fn test_rating_validation_too_high() {
        let entry = CoffeeEntry::new(
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            ProcessingMethod::Natural,
            Roast::Dark,
            Preparation::Latte,
            11,
        );

        assert!(entry.is_err());
        assert_eq!(entry.unwrap_err(), "Rating must be between 1 and 10, got 11");
    }

    #[test]
    fn test_rating_validation_boundaries() {
        let entry1 = CoffeeEntry::new(
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            ProcessingMethod::Washed,
            Roast::Medium,
            Preparation::Drip,
            1,
        );
        assert!(entry1.is_ok());

        let entry10 = CoffeeEntry::new(
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            ProcessingMethod::Washed,
            Roast::Medium,
            Preparation::Drip,
            10,
        );
        assert!(entry10.is_ok());
    }

    #[test]
    fn test_plaintext_formatting() {
        let timestamp = Local.with_ymd_and_hms(2025, 1, 15, 14, 30, 0).unwrap();

        let entry = CoffeeEntry::with_timestamp(
            timestamp,
            "Intelligentsia".to_string(),
            "Black Cat".to_string(),
            "Colombia".to_string(),
            ProcessingMethod::PulpedNatural,
            Roast::Medium,
            Preparation::Espresso,
            9,
        ).unwrap();

        let plaintext = entry.to_plaintext();

        assert!(plaintext.contains("2025-01-15 14:30:00"));
        assert!(plaintext.contains("Brand: Intelligentsia"));
        assert!(plaintext.contains("Name: Black Cat"));
        assert!(plaintext.contains("Origin: Colombia"));
        assert!(plaintext.contains("Processing: Pulped Natural"));
        assert!(plaintext.contains("Roast: Medium"));
        assert!(plaintext.contains("Preparation: Espresso"));
        assert!(plaintext.contains("Rating: 9/10"));
        assert!(plaintext.ends_with("---\n"));
    }

    #[test]
    fn test_processing_method_display() {
        assert_eq!(ProcessingMethod::Washed.to_string(), "Washed");
        assert_eq!(ProcessingMethod::Natural.to_string(), "Natural");
        assert_eq!(ProcessingMethod::PulpedNatural.to_string(), "Pulped Natural");
        assert_eq!(ProcessingMethod::AnerobicFermentation.to_string(), "Anerobic Fermentation");
    }

    #[test]
    fn test_roast_display() {
        assert_eq!(Roast::Light.to_string(), "Light");
        assert_eq!(Roast::Medium.to_string(), "Medium");
        assert_eq!(Roast::Dark.to_string(), "Dark");
    }

    #[test]
    fn test_preparation_display() {
        assert_eq!(Preparation::Americano.to_string(), "Americano");
        assert_eq!(Preparation::Cortado.to_string(), "Cortado");
        assert_eq!(Preparation::Drip.to_string(), "Drip");
        assert_eq!(Preparation::Espresso.to_string(), "Espresso");
        assert_eq!(Preparation::Latte.to_string(), "Latte");
        assert_eq!(Preparation::Pourover.to_string(), "Pourover");
    }
}
