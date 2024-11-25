use std::env;

pub fn get_current_branch() -> Result<String, git2::Error> {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    let repo = git2::Repository::open(current_dir)?;

    let head = repo.head()?;
    Ok(head.shorthand().unwrap_or("").to_string())
}
