use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::model::software::{SoftwareCategory, Tier};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Program {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tier: Tier,
    pub packages: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub recommended: bool,
    pub reasoning: Option<String>,
    pub post_install: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subcategory {
    pub name: String,
    pub description: String,
    pub programs: Vec<Program>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "name")]
    pub id: SoftwareCategory,
    pub display_name: String,
    pub description: String,
    pub icon: Option<String>,
    pub subcategories: Vec<Subcategory>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Catalog {
    pub categories: Vec<Category>,
}

impl Catalog {
    pub fn load_s_tier() -> anyhow::Result<Self> {
        let toml_str = include_str!("../../../resources/catalog/s-tier_catalog.toml");
        let catalog: Catalog = toml::from_str(toml_str)?;
        Ok(catalog)
    }

    pub fn load_full() -> anyhow::Result<Self> {
        let toml_str = include_str!("../../../resources/catalog/full_catalog.toml");
        let catalog: Catalog = toml::from_str(toml_str)?;
        Ok(catalog)
    }

    pub fn load_languages() -> anyhow::Result<Self> {
        let toml_str = include_str!("../../../resources/catalog/programming_languages.toml");
        let catalog: Catalog = toml::from_str(toml_str)?;
        Ok(catalog)
    }
}

pub fn curated_catalog() -> Catalog {
    Catalog::load_s_tier().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_s_tier_catalog() {
        let catalog = Catalog::load_s_tier().expect("Failed to load S-tier catalog");
        assert!(!catalog.categories.is_empty());
    }

    #[test]
    fn can_load_full_catalog() {
        let catalog = Catalog::load_full().expect("Failed to load full catalog");
        assert!(!catalog.categories.is_empty());
    }

    #[test]
    fn can_load_languages_catalog() {
        let catalog = Catalog::load_languages().expect("Failed to load languages catalog");
        assert!(!catalog.categories.is_empty());
    }

    #[test]
    fn enforce_driver_mappings_s_tier() {
        let catalog = Catalog::load_s_tier().expect("Failed to load S-tier catalog");
        let required_drivers = ["arch", "debian", "fedora"];
        let mut missing = Vec::new();

        for cat in &catalog.categories {
            for sub in &cat.subcategories {
                for prog in &sub.programs {
                    for driver in &required_drivers {
                        if !prog.packages.contains_key(*driver) {
                            missing.push(format!(
                                "{}::{}::{} missing mapping for '{}'",
                                cat.display_name, sub.name, prog.name, driver
                            ));
                        } else if prog.packages.get(*driver).unwrap().is_empty() {
                            missing.push(format!(
                                "{}::{}::{} has empty mapping for '{}'",
                                cat.display_name, sub.name, prog.name, driver
                            ));
                        }
                    }
                }
            }
        }

        if !missing.is_empty() {
            panic!("S-Tier Catalog Validation Failed:\n{}", missing.join("\n"));
        }
    }
}
