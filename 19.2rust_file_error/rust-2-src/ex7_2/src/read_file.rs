// read_file.rs
pub mod file_control{
    use std::error::Error;
    use std::fs::File;
    use std::io;

    pub fn readfile(path:String) -> Result<File, io::Error> {
        return File::open(path);
    }
    pub  fn createfile(name:String) -> Result<File, io::Error> {
        return File::create(name);
    }
}
