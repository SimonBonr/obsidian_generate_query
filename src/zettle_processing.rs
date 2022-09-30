use crate::data::Nested_Links;

use super::data;
use std::{fs, collections::HashMap};

pub fn modify_zettle_files(zet_loc: &String, mut nested_links:data::Nested_Links) {


    if nested_links.links.len() >= 1 {
        //rec_nested_link(&nested_links, 0);

        for (file, sub_nest_links) in nested_links.links.iter() {

            /*if file != &"eBPF".to_lowercase() { // !Remove me eventually
                continue;
            }*/
            let fileaddr = format!("{}\\{}.md", zet_loc, file);


            match fs::read_to_string(&fileaddr) {
                Ok(res) => {
                    println!("{}", fileaddr);
                    let new_filedata = modify_file(&res, sub_nest_links, file);

                    println!("{}", new_filedata);

                    fs::write(&fileaddr, new_filedata);
                },
                Err(_) => {if data::DEBUGGING {println!("Cannot find file: {}", fileaddr);}}
            }
            
        }
    }
}

fn modify_file(filedata: &String, nested_links: &data::Nested_Links, filename: &str) -> String {


    if filedata.len() == 0 {
        println!("GENERATE FILE FROM TEMPLATE {}", filename);
        return "".to_string();
        //Load
    }

    let ref_line = filedata.rfind("# References").unwrap_or_default();
    let query_line = filedata.rfind("---").unwrap_or_default();

    //println!("{} {}", ref_line, query_line);


    if ref_line > query_line { //Doesn't have query block at end
        let query_data = add_links(1, filename, nested_links, "");

        let toc = generate_toc(nested_links, 0, filename);

        let new_filedata = format!("{}\n\n---\n# ToC\n{}\n\n{}", filedata, toc, query_data);

        return new_filedata;

        //println!("ASDJASKDASKD");
    } else { //Already has query block at end
        
        let query_data = add_links(1, filename, nested_links, filedata.split_at(query_line).1);

        let toc = generate_toc(nested_links, 0, filename);

        let new_filedata = format!("{}---\n# ToC\n{}\n\n{}", filedata.split_at(query_line).0, toc, query_data);
        //println!("{}", new_filedata);
        return new_filedata;
        //println!("{}", new_str);
        //get_query_headers(filedata.split_at(query_line).1, &mut query_headers);


    }
    
}

fn generate_toc(nested_links: &data::Nested_Links, tabs: usize, prev_link: &str) -> String {

    let mut toc = String::new();

    for (header, sub_nest_links) in nested_links.links.iter() {
        let link = format!("{}[[{}#{}|{}]]\n",str::repeat("\t", tabs), prev_link, header, header);
        toc.push_str(&link);
        toc.push_str(&generate_toc(&sub_nest_links, tabs + 1, prev_link));
    }

    return toc;
}

fn add_links(depth: usize, header_combo: &str, nested_links: &data::Nested_Links, existing_data: &str) -> String {

    let mut new_str: String = String::new();

    for (header, sub_nest_links) in nested_links.links.iter() {
        let mut new_header_combo: String = String::new();

        /*if depth == 1 {
            new_header_combo = header.to_string();
        } else{
            
        }*/
        new_header_combo = format!("{}#{}", header_combo, header).to_string();
        
        let header_str = format!("{} {}", str::repeat("#", depth), header);

        match existing_data.find(&header_str) {
            Some(header_i) => {
                //println!("{}, {}", header_i, )
                let header_data = existing_data.split_at(header_i).1;
                let split_str = "```\n";
                
                match header_data.find(split_str) { //The extra split_at is just to skip the first characters, bcs rust strings are weird
                    Some(next_header_i) => {
                        new_str.push_str(header_data.split_at(next_header_i  + split_str.len()).0);

                        println!("----------Test 1: {}----------", header_data.split_at(next_header_i + split_str.len()).0);
                    },
                    None => { //Header was at end of file
                        new_str.push_str(header_data);
                        println!("----------Test 2: {}----------", header_data);
                    }
                }
            },
            None => {
                new_str.push_str(format!("{}\n```query\nblock:(\"[[{}]]\")\n```\n", header_str, new_header_combo).as_str());
            }
        };

        new_str.push_str(&add_links(depth + 1, &new_header_combo, sub_nest_links, existing_data));
    }
    
    return new_str;
}


fn rec_nested_link(nested_links: &data::Nested_Links, tabs: usize) {

    for (header, sub_nest_links) in nested_links.links.iter() {

        println!("{} {}", str::repeat("\t", tabs), header);

        rec_nested_link(&sub_nest_links, tabs + 1)
    }
}

fn add_queries() {

}