mod litterature_processing;
mod zettle_processing;
pub mod data;
use std::collections::HashMap;
use std::fs;


fn main() {
    println!("Hello, world!");
    let mut files: HashMap<String, String> = HashMap::new();
    parse_files(&mut files);
    
    let lit_loc = files.get("Litterature").expect("Missing Litterature Key in file");
    let zet_loc = files.get("Zettelkasten").expect("Missing Zettelkasten Key in file");

    let mut nested_links = data::Nested_Links{links: HashMap::new()};    

    litterature_processing::parse_lit_files(lit_loc, &mut nested_links);
    zettle_processing::modify_zettle_files(zet_loc, nested_links);
}


static files_file: &str  = "./resources/folders.txt";
fn parse_files(files: &mut HashMap<String, String>) {
    let contents = fs::read_to_string(files_file).expect(&format!("{}{}", "Could not open file: ", files_file));

    let mut line_i = 0;

    for line in contents.split('\n') {
        
        if !line.contains("#") {
                        
            let key_value: Vec<&str> = line.splitn(2, ":").collect();

            if key_value.len() != 2 {
                println!("Invalid line at {}", line_i);
            } else {
                files.insert(key_value[0].trim().to_string(), key_value[1].trim().to_string());
            }
        }
        line_i += 1;
    }

    if data::DEBUGGING {
        
        println!("Debug print of Keys : Values");
        for (key, value) in files.iter() {
            println!("{} : {}", key, value);
        }
    }
}