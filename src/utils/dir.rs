use std::env;
use std::path::Path;

pub fn pwd() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("current directory is : {}", path.display());

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::utils::dir::pwd;

    #[test]
    fn test_pwd() {
        match pwd() {
            Ok(..) => { println!("ok") }
            Err(e) => { println!("error: {}", e) }
        }
    }
}