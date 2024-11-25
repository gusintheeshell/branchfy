// src/daemon.rs
use crate::config::Config;
use crate::watcher::BranchWatcher;
use std::path::Path;

pub struct Daemon {
    config: Config,
}

impl Daemon {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn start(&self) {
        println!("Starting branchfy daemon...");

        // Check if current directory is git repo
        if !self.is_git_repo(Path::new(".")) {
            println!("Current directory is not a git repository");
            return;
        }

        // Create and start watcher for current repo
        if let Ok(mut watcher) = BranchWatcher::new(self.config.clone()) {
            println!("Starting watcher for current repository");
            watcher.start().await;
        } else {
            println!("Failed to create watcher for current repository");
        }
    }

    fn is_git_repo(&self, path: &Path) -> bool {
        path.join(".git").is_dir()
    }
}
