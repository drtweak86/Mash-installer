use anyhow::Result;
use installer_core::catalog::Catalog;

pub fn catalog_to_text(catalog: &Catalog) -> String {
    let mut output = String::new();
    for category in &catalog.categories {
        output.push_str(&format!(
            "Category: {} ({})\n",
            category.display_name, category.name
        ));
        if let Some(desc) = &category.description.split('\n').next() {
            output.push_str(&format!("  {}\n", desc));
        }

        for subcategory in &category.subcategories {
            output.push_str(&format!("  Subcategory: {}\n", subcategory.name));
            for program in &subcategory.programs {
                let rec_marker = if program.recommended { " [REC]" } else { "" };
                output.push_str(&format!(
                    "    - {} ({}): {}{}\n",
                    program.name, program.id, program.tier, rec_marker
                ));
                output.push_str(&format!("      {}\n", program.description));
            }
        }
        output.push('\n');
    }
    output
}

pub fn catalog_to_json(catalog: &Catalog) -> serde_json::Result<String> {
    serde_json::to_string_pretty(catalog)
}

pub fn print_catalog(catalog: &Catalog, json_output: bool) -> Result<()> {
    if json_output {
        println!("{}", catalog_to_json(catalog)?);
    } else {
        print!("{}", catalog_to_text(catalog));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::catalog_to_json;
    use super::catalog_to_text;
    use installer_core::catalog::Catalog;

    #[test]
    fn catalog_output_includes_categories() {
        let catalog = Catalog::load_s_tier().unwrap_or_default();
        if catalog.categories.is_empty() {
            return;
        }
        let text = catalog_to_text(&catalog);
        assert!(text.contains(&catalog.categories[0].display_name));
    }

    #[test]
    fn catalog_json_is_valid() {
        let catalog = Catalog::load_s_tier().unwrap_or_default();
        let json = catalog_to_json(&catalog).expect("json");
        assert!(serde_json::from_str::<serde_json::Value>(&json).is_ok());
    }
}
