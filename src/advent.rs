use std::{fs::File, path::Path, io::{self, Read}};

pub fn read(input: &str) -> Result<String, io::Error> {
  let path_string = format!("./inputs/{}", input);
  let path = Path::new(path_string.as_str());
  
  let file = File::open(path);
  let mut string = String::new();

  match file {
    Ok(mut f) => match f.read_to_string(&mut string) {
      Ok(_) => Ok(string),
      Err(e) => Err(e)
    },
    Err(e) => Err(e)
  }
}

pub fn read_lines(input: &str) -> Result<Vec<String>, io::Error> {
  match read(input) {
    Ok(file) => Ok(file.lines().map(String::from).collect()),
    Err(e) => Err(e) 
  }
}