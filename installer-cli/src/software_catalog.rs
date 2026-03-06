use installer_core::catalog::Catalog;

#[allow(dead_code)]
pub fn load_s_tier() -> Catalog {
    Catalog::load_s_tier().unwrap_or_default()
}

pub fn load_full() -> Catalog {
    Catalog::load_full().unwrap_or_default()
}

#[allow(dead_code)]
pub fn load_languages() -> Catalog {
    Catalog::load_languages().unwrap_or_default()
}
