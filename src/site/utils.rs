use kuchiki::traits::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn add_target_blank_to_links(html: String) -> String {
    // Parse the HTML document
    let document = kuchiki::parse_html().one(html);

    // Get all `a` elements in the document
    let a_elements = document.select("a").unwrap();
    
    for a in a_elements {
        let mut attrs = a.attributes.borrow_mut();
        // Add the target "_blank" attribute
        attrs.insert("target", "_blank".to_string());
    }
    
    // Serialize the modified document back to a string
    let body = document.select_first("body").unwrap();
    let mut output = vec![];
    for child in body.as_node().children() {
        child.serialize(&mut output).unwrap();
    }
    String::from_utf8(output).unwrap()
}

pub fn read_first_five_news() -> String {
    let file = File::open("content/news.md").expect("Failed to open news.md");
    let reader = io::BufReader::new(file);

    let mut count = 0;
    let mut out_string = String::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            count += 1;
            if count == 5 {
                break;
            }
        }
        out_string.push_str(&line);
        out_string.push_str("\n");
    }
    out_string
}
