use notify::{Watcher, RecommendedWatcher, RecursiveMode, Config};
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
use tauri::{AppHandle, Manager, Emitter};

pub fn start_watcher(paths: Vec<String>, app: AppHandle) -> RecommendedWatcher {
    

    let mut watcher = RecommendedWatcher::new(
        move |res: notify::Result<notify::Event>| {
            match res {
                Ok(event) => {
                    // Send an IPC event to the frontend
                    app.emit("rescan_on_change", ()).ok();
                }
                Err(e) => eprintln!("watch error: {:?}", e),
            }
        },
        Config::default().with_poll_interval(Duration::from_millis(500)),
    ).unwrap();
    
    for path in paths {
        let p = Path::new(&path);
        if p.exists() {
            watcher.watch(p, RecursiveMode::Recursive).ok();
        }
    }
    
    watcher
}
