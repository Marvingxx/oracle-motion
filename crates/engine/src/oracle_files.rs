//! Documents of the other Oracle apps (an Oracle Vector logo, an Oracle Edit picture, an Oracle
//! Brand Guide): imported as their drawing, made by the app that owns them (`oracle_suite_link`), at
//! a path that stays the same from one version of the document to the next. The footage watch draws
//! it again when the document was saved ([`refresh`]), so a logo saved in Oracle Vector reloads in
//! the composition by itself; Edit Original opens the document in its app ([`edit_original`]).
//! The web app has no other apps: there, every file is itself.

use std::path::Path;

/// The file to import for `path`: the drawing of another Oracle app's document, else `path` itself.
pub fn import_path(path: &str) -> Result<String, String> {
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(format) = oracle_suite_link::oracle_format(Path::new(path)) {
        let preview = format.previews.first().copied().unwrap_or(oracle_suite_link::Preview::Png);
        return oracle_suite_link::render_stable(Path::new(path), preview).map(|d| d.to_string_lossy().into_owned()).map_err(|e| format!("{path}: {e}"));
    }
    Ok(path.to_string())
}

/// Draws again, among `paths` (the watched footage files), the drawings whose documents were saved
/// since: called before their stamps are taken. Cheap for the others (a path comparison).
pub fn refresh<'a>(paths: impl IntoIterator<Item = &'a String>) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(live) = oracle_suite_link::cache_dir().map(|d| d.join("live")) else { return };
        for path in paths {
            let path = Path::new(path);
            if path.starts_with(&live)
                && let Err(e) = oracle_suite_link::refresh_stable(path)
            {
                log::warn!("{}: {e}", path.display());
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    let _ = paths.into_iter();
}

/// Edit Original of a drawing made from another Oracle app's document: that document opened in its
/// app (the app's name), or why it couldn't be. `None` for any other file.
pub fn edit_original(path: &Path) -> Option<Result<&'static str, String>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let document = oracle_suite_link::stable_source(path)?;
        Some(oracle_suite_link::edit_original(&document).map(|app| app.name()))
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = path;
        None
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// Any other file imports as itself.
    #[test]
    fn other_files_are_themselves() {
        assert_eq!(super::import_path("C:/clips/a.mov").unwrap(), "C:/clips/a.mov");
        assert!(super::edit_original(Path::new("C:/clips/a.mov")).is_none());
    }
}
