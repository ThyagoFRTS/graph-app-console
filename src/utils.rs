use std::collections::HashMap;
use std::io::{prelude::*, self};
use std::fs::File;
use std::path::Path;

pub fn remove_letters(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c.is_numeric() || c == ',' {
            result.push(c);
        }
    }
    result
}

pub fn create_mapper(v: Vec<usize>) -> impl Fn(usize) -> Option<usize> {
    let mut mapper = HashMap::new();
    for (i, &x) in v.iter().enumerate() {
        mapper.insert(x, i);
    }
    move |value| mapper.get(&value).cloned()
}

pub fn verify_file_exist(file_path: &str) -> bool {
    Path::new(file_path).exists()
} 

pub fn create_file(content: String) {
    let mut data_file = File::create("graphviz.dot").expect("creation failed");
    // Write contents to the file
    data_file.write(content.as_str().as_bytes()).expect("write failed");
}

pub fn get_clean_selection_input(msg: &str, opt: &Vec<String>) -> usize {
    
    loop {
        let mut value = String::new();

        print!("{}", msg);
        
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut value)
            .expect("Falha ao ler a entrada");

        let value = value.trim();

        if value.is_empty() {
            println!("Digite um valor válido. (v0, v1..., vn)");
            continue;
        }

        if !opt.contains(&value.to_string()) {
            println!("Valor inválido. Tente novamente.");
            continue;
        }

        return value.replace("v", "").parse::<usize>().unwrap();
    }
}

