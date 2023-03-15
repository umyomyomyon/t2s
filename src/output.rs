use std::fs::File;
use std::io::Write;

pub fn make_file(code: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("output.js")?;
    file.write_all(code.as_bytes())?;
    Ok(())
}
