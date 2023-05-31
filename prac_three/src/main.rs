use std::fs::File;
use std::io::Read;
use xmltree::Element;

fn main() {
    let xml_string = read_xml_file("AST.xml");
    let root = parse_xml_string(&xml_string);
    generate_intermediate_code(&root);
}

fn read_xml_file(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Failed to open XML file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read XML file");
    contents
}

fn parse_xml_string(xml_string: &str) -> Element {
    let xml = xml_string.as_bytes();
    Element::parse(xml).expect("Failed to parse XML")
}

fn generate_intermediate_code(node: &Element) {
    match node.name.as_str() {
        "ASSIGN" => generate_assign_code(node),
        // Add more cases for different element types as needed
        _ => (),
    }

    for child in &node.children {
        if let Some(child) = child.as_element() {
            generate_intermediate_code(child);
        }
    }
}

fn generate_assign_code(node: &Element) {
    if let Some(id) = node.attributes.get("id") {
        println!("Assign id: {}", id);
    }

    for child in &node.children {
        println!("{:?}", child.as_element());
        // match child.as_element().unwrap().name.as_str() {
        //     "NUMVAR" => generate_numvar_code(child.as_element().unwrap()),
        //     "NUMEXPR" => generate_numexpr_code(child.as_element().unwrap()),
        //     // Handle other relevant elements
        //     _ => (),
        // }
    }
}

fn generate_numvar_code(node: &Element) {
    if let Some(id) = node.attributes.get("id") {
        println!("Numvar id: {}", id);
    }

    // Generate code for NUMVAR content
    if let Some(content) = node.get_text() {
        println!("Numvar content: {}", content.trim());
    }
}

fn generate_numexpr_code(node: &Element) {
    if let Some(id) = node.attributes.get("id") {
        println!("Numexpr id: {}", id);
    }

    for child in &node.children {
        if let Some(child) = child.as_element() {
            match child.name.as_str() {
                "DECNUM" => generate_decnum_code(child),
                // Handle other relevant elements
                _ => (),
            }
        }
    }
}

fn generate_decnum_code(node: &Element) {
    if let Some(id) = node.attributes.get("id") {
        println!("Decnum id: {}", id);
    }

    // Generate code for DECNUM content
    if let Some(content) = node.get_text() {
        println!("Decnum content: {}", content.trim());
    }
}

fn traverse_tree(node: &Element, indentation: usize) {
    // Print the current element with appropriate indentation
    println!("{:indent$}{}", "", node.name, indent = indentation * 4);

    // Process the text content if it exists
    for child in &node.children {
        if let xmltree::XMLNode::Text(text) = child {
            // Print the text content with appropriate indentation
            println!("{:indent$}{}", "", text, indent = (indentation + 1));
        }
    }

    // Recursively traverse child nodes
    for child in &node.children {
        if let xmltree::XMLNode::Element(element) = child {
            traverse_tree(element, indentation + 1);
        }
    }
}
