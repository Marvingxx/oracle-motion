//! Oracle Motion in the Oracle suite. Window ▸ Oracle Library shows the library (its window is
//! `oracle_library_ui`); Composition ▸ Send Frame To puts the current frame in the library or in
//! another Oracle app's open document; what the other apps send here (their File ▸ Send to…) is
//! imported and added to the open composition. Sending goes through `oracle_suite_link::inbox`: a
//! small request file the receiving app takes on its next frame (started when it isn't running).

use std::path::PathBuf;

use oracle_suite_link::App;
use oracle_suite_link::inbox::{self, Action};
use serde_json::{Value, json};

use crate::OracleMotionApp;

/// How often the inbox is looked at, in seconds (on frames drawn anyway: it never wakes the app).
const EVERY: f64 = 1.0;

/// Characters a file name can't hold on Windows (and `/` anywhere).
fn file_name_safe(name: &str) -> String {
    let cleaned: String = name.chars().map(|c| if matches!(c, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*') || c.is_control() { '-' } else { c }).collect();
    let trimmed = cleaned.trim().trim_end_matches('.').to_string();
    if trimmed.is_empty() { "Frame".into() } else { trimmed }
}

/// The active composition's current frame as a PNG, stored in the library ("Mes créations"), the
/// window shown when `show`: where it is now.
pub fn frame_to_library(app: &mut OracleMotionApp, show: bool) -> Result<PathBuf, String> {
    let id = app.session.active_comp_id().ok_or("no active composition")?;
    let name = app.session.project.item(id).map_or_else(|| "Frame".to_string(), |it| it.name.clone());
    let dir = std::env::temp_dir().join(format!("oracle-motion-frames-{}", std::process::id()));
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let file = dir.join(format!("{}.png", file_name_safe(&name)));
    app.session.execute("comp.saveFrameAs", json!({"path": file.to_string_lossy()})).map_err(|e| e.to_string())?;
    let added = if show { app.oracle_library.add_files(std::slice::from_ref(&file)) } else { app.oracle_library.store_files(std::slice::from_ref(&file)) };
    let _ = std::fs::remove_file(&file); // Copied into the library; a leftover is only a temp file.
    let stored = added.into_iter().next().ok_or("the frame couldn't be stored in the Oracle Library")?;
    app.ui.status = format!("Frame added to the Oracle Library: {}", stored.display());
    Ok(stored)
}

/// Composition ▸ Send Frame To ▸ `target`: the frame placed in `target`'s open document.
pub fn send_frame(app: &mut OracleMotionApp, target: App) -> Result<Value, String> {
    let file = frame_to_library(app, false)?;
    inbox::send(target, Action::Place, &file)?;
    app.ui.status = format!("Sent to {}", target.name());
    Ok(json!({"sentTo": target.id(), "path": file}))
}

/// Takes what the other Oracle apps sent: each file imported, and added to the open composition
/// when it was sent to be placed. Looks at most every second.
pub fn poll_inbox(app: &mut OracleMotionApp, ctx: &egui::Context) {
    // Once: a background thread keeps Oracle Motion marked as running, and wakes it when something
    // is sent.
    let watching = egui::Id::new("oracle-suite-inbox-watch");
    if !ctx.data(|d| d.get_temp::<bool>(watching).unwrap_or(false)) {
        ctx.data_mut(|d| d.insert_temp(watching, true));
        let waker = ctx.clone();
        inbox::watch(App::Motion, move || waker.request_repaint());
    }
    let now = ctx.input(|i| i.time);
    let id = egui::Id::new("oracle-suite-inbox");
    if ctx.data(|d| d.get_temp::<f64>(id)).is_some_and(|last| now - last < EVERY) {
        return;
    }
    ctx.data_mut(|d| d.insert_temp(id, now));
    let requests = inbox::take(App::Motion);
    if requests.is_empty() {
        return;
    }
    for r in requests {
        let into_comp = matches!(r.action, Action::Place) && app.session.active_comp_id().is_some();
        let params = json!({"paths": [r.file.to_string_lossy()], "addToComp": into_comp, "background": true});
        if let Err(e) = crate::menus::invoke(app, ctx, "file.import", params) {
            app.ui.status = e;
        }
    }
    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
}

#[cfg(test)]
mod tests {
    use super::file_name_safe;

    #[test]
    fn names_become_file_names() {
        assert_eq!(file_name_safe("Intro: logo?"), "Intro- logo-");
        assert_eq!(file_name_safe(" . "), "Frame");
    }
}
