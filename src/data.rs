use std::collections::HashMap;

pub static DEBUGGING:bool = false;

pub struct Nested_Links {
    pub links: HashMap<String, Box<Nested_Links>>,
}