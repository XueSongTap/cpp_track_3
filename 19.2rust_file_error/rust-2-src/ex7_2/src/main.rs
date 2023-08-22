use std::fs::File;
mod read_file;
use read_file::file_control as file_ctrl;

fn main() {
    let find_file = file_ctrl::readfile("Good_Morning.txt".to_string());
    if find_file.is_err(){
        let make_file = file_ctrl::createfile("Good_Morning.txt".to_string());
        if make_file.is_err(){
            panic!("Something wrong with writing");
        }else{
            println!("File writing is ok {:?}",make_file);
        }
        let find_file = file_ctrl::readfile("Good_Morning.txt".to_string());
        if find_file.is_err() {
            panic!("Something wrong with reading");
        }else{
            println!("File reading is ok {:?}",find_file);
        }
    }
}
 