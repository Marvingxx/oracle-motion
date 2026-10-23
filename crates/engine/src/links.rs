//! Project links (shown in Help ▸, the About dialog and the start screen). The community link opens
//! Oracle Suite, the launcher of every Oracle app.

pub const APP_NAME: &str = "Oracle Motion";
pub const APP_SLUG: &str = "oracle-motion";
/// Oracle Suite: the launcher that installs and opens every Oracle app (Help ▸ Oracle Suite).
pub const DISCORD: &str = "https://github.com/Marvingxx/oracle-suite";
/// Oracle on GitHub: every Oracle app's source.
pub const WEBSITE: &str = "https://github.com/Marvingxx";
pub const APP_PAGE: &str = "https://github.com/Marvingxx/oracle-motion";
pub const GITHUB: &str = "https://github.com/Marvingxx/oracle-motion";
pub const ISSUES: &str = "https://github.com/Marvingxx/oracle-motion/issues";

/// The other Oracle apps: (name, GitHub repository).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("Oracle Edit", "oracle-edit"),
    ("Oracle Vector", "oracle-vector"),
    ("Oracle Brand Guide", "oracle-guide"),
    ("Oracle Brand Kit", "oracle-brand"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/Marvingxx/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://github.com/Marvingxx/{slug}")
}
