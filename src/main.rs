use anyhow::Error;
use std::path::PathBuf;

fn main() {
    if let Err(e) = doit() {
        println!("{:?}", e);
    }
}

fn doit() -> Result<(), Error> {
    let new_repo = PathBuf::from("index-clone");

    let mut opts = git2::RepositoryInitOptions::new();
    opts.external_template(false);
    let repo = git2::Repository::init_opts(&new_repo, &opts)?;

    let refspecs = vec!["HEAD:refs/remotes/origin/HEAD".to_string()];
    let mut fetch_opts = git2::FetchOptions::new();
    let _res = repo
        .remote_anonymous("index/.git")?
        .fetch(&refspecs, Some(&mut fetch_opts), None)?;

    Ok(())
}
