use std::{collections::HashMap, hash::Hash};
use std::fmt;
use crate::types::{Decoded, KyaMap, Result};

pub fn decode(text: String) -> Result<Decoded> {
    let mut ns_text = text.replace(" ", "");
    let f = match ns_text.chars().collect::<Vec<char>>()[0] {
        '<' => decode_list,
        '^' => decode_dict,
        _ => decode_literal
    };
    Ok(f(ns_text[1..ns_text.len()].to_string())?.0)
}

pub fn decode_list(text: String) -> Result<(Decoded, usize)> {
    let chs: Vec<char> = text.chars().collect();
    let mut ls: Vec<Box<Decoded>> = Vec::new();
    let mut end: usize = 0;
    let mut i: usize = 0;
    while i<text.len() {
        let ch = *chs.get(i).ok_or("Text is longer than list of characters.")?;
        if ch == '>' {
            break;
        }
        let newm: Decoded;
        let f = match ch {
            '<' => decode_list,
            '^' => decode_dict,
            _ => decode_literal
        };
        let j = f((text[i+1..text.len()].to_string()))?;
        newm = j.0;
        i += j.1+1;
        ls.push(Box::new(newm));
        end = i;
    }
    Ok((Decoded::List(ls), end+1))
}

pub fn decode_dict(text: String) -> Result<(Decoded, usize)> { 
    let chs: Vec<char> = text.chars().collect();
    let mut ls = KyaMap::new();
    let mut end: usize = 0;
    let mut i: usize = 0;
    while i<text.len() {
        let ch = *chs.get(i).ok_or("Text is longer than list of characters.")?;
        if ch == '$' {
            break;
        }
        let newm: Decoded;
        let f = match ch {
            '<' => decode_list,
            '^' => decode_dict,
            _ => decode_literal
        };
        let j = f((text[i+1..text.len()].to_string()))?;
        newm = j.0;
        if ls.k.len() <= ls.v.len() {
            ls.k.push(newm);
        } else {
            ls.v.push(newm);
        }
        i += j.1+1;
        end = i;
    }
    
    Ok((Decoded::Dict(ls), end+1))
 }

pub fn decode_literal(text: String) -> Result<(Decoded, usize)> { 
    let f = text.chars().collect::<Vec<char>>()[0];
    let mut obj = &text[1..text.len()];
    let end = text.find('\'').ok_or("Literal has no end")?;
    obj = &obj[0..obj.find('\'').ok_or("Literal has no end")?];
    match f {
        'i' => return Ok((Decoded::Num(obj.parse::<u64>().expect("Invalid literal")), end+1)),
        _ => return Ok((Decoded::Str(obj.to_string()), end+1))
    }
}