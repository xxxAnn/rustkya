use std::collections::HashMap;

#[derive(Debug)]
pub enum Decoded {
    Str(String),
    Num(u64),
    Dict(HashMap<Box<Decoded>, Box<Decoded>>),
    List(Vec<Box<Decoded>>)
}

pub fn decode(text: String) -> (Decoded, usize) {
    let mut ns_text = text.replace(" ", "");
    let f = match ns_text.chars().collect::<Vec<char>>()[0] {
        '<' => decode_list,
        '$' => decode_dict,
        _ => decode_literal
    };
    f(ns_text[1..ns_text.len()].to_string())
}

pub fn decode_list(text: String) -> (Decoded, usize) {
    let chs: Vec<char> = text.chars().collect();
    let mut ls: Vec<Box<Decoded>> = Vec::new();
    let mut end: usize = 0;
    let mut i: usize = 0;
    while i<text.len() {
        let ch = chs[i];
        if ch == '>' {
            break;
        }
        let newm: Decoded;
        let f = match ch {
            '<' => decode_list,
            '$' => decode_dict,
            _ => decode_literal
        };
        let j = f((text[i+1..text.len()].to_string()));
        newm = j.0;
        i += j.1+1;
        ls.push(Box::new(newm));
        end = i;
    }
    (Decoded::List(ls), end+1)
}

pub fn decode_dict(text: String) -> (Decoded, usize) { todo!() }

pub fn decode_literal(text: String) -> (Decoded, usize) { 
    let f = text.chars().collect::<Vec<char>>()[0];
    let mut obj = &text[1..text.len()];
    let end = text.find('\'').unwrap();
    obj = &obj[0..obj.find('\'').unwrap()];
    match f {
        'i' => return (Decoded::Num(obj.parse::<u64>().expect("Invalid literal")), end+1),
        _ => return (Decoded::Str(obj.to_string()), end+1)
    }
}