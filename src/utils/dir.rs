use std::env;
use std::path::Path;

pub fn pwd() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("current directory is : {}", path.display());

    Ok(())
}