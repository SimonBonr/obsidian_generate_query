use super::data;
use std::fs;
use std::collections::HashMap;


pub fn parse_lit_files(lit_loc: &String, nested_links: &mut data::Nested_Links) {

    let lit_files = fs::read_dir(lit_loc).expect(&format!("{}{}", "Couldn't open the Litterature folder: ", lit_loc));

    for lit_file in lit_files {
        let filename = lit_file.unwrap().path();
        match fs::read_to_string(&filename) {
            Ok(res) => {parse_lit_content(&res, nested_links);},
            Err(err) => {if (data::DEBUGGING){ println!("Skipping {} due to {}", filename.display(), err)}},
        };
        
    }
}

fn parse_lit_content(content: &String, nested_links: &mut data::Nested_Links) {
    //let mut links: HashMap<String, Option<Box<HashMap<>>>> = HashMap::new();

    let ann_line = content.find("Annotations").unwrap_or(content.len() - 1);
    let mut on_ann_line = true;

    for paragraph in content[ann_line..].split("\n\n") {

        if on_ann_line {
            on_ann_line = false;
            continue;
        }
        //println!("{}", paragraph);
        let mut paragraph_split:Vec<&str> = paragraph.rsplit("” (").collect(); //"[#HIGHLIGHT...][” (Van .... #CITATION) #COMMENT]"
        
        if paragraph_split.len() != 2 {
            
            if paragraph.chars().nth(0).unwrap() == '(' { //Starts with citations as there's no comment
                paragraph_split = vec![paragraph];
            } else {
                println!("---Skipping following paragraph due to non obvious split---\n{}", paragraph);
                continue;
            }
        }
        let paragraph_split2:Vec<&str> = paragraph_split[0].splitn(2, ")").collect(); //[” (Van .... #CITATION)] [#COMMENT]

        if paragraph_split2.len() != 2 {
            println!("---Skipping following comment due to non obvious split---\n{}", paragraph_split[1]);
            continue;
        }
        let my_comment = paragraph_split2[1];

        parse_lit_comment(my_comment, nested_links);

        //println!("{}", my_comment);
    }
}

fn parse_lit_comment(comment: &str, nested_links: &mut data::Nested_Links) {
    

    for link in comment.split("[[") {

        if link.contains(".png") ||  link.contains(".PNG") || !link.contains("]]") {
            continue;
        }

        let mut current_nest_link = &mut nested_links.links;
        let mut tabs = 0;

        
        for header in link.split("#") {

            if header.len() >= 2 {

                let header_clean = match header.find("]]") {
                    Some(res) => {header.split_at(res).0.to_lowercase()},
                    None => header.to_lowercase(),
                };
                if header_clean.len() >= 40 { //Most likely a header towards another paper
                    
                    if data::DEBUGGING {
                        println!("Skipping header: \n{}\nFrom comment: \n{}", header,  comment);
                    }
                    continue;
                }
                let child_links = current_nest_link.entry(header_clean.to_string()).or_insert(Box::new(data::Nested_Links { links: HashMap::new() }));

                //println!("{} {}", str::repeat("\t", tabs), header_clean);

                current_nest_link = &mut child_links.links;
            }
            tabs += 1;
        }
    }

}