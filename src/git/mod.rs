use git2::Repository;

pub fn get_current_branch() -> Result<String, git2::Error> {
    let repo = Repository::open(".")?;
    let head = repo.head()?;
    Ok(head.shorthand().unwrap_or("").to_string())
}
