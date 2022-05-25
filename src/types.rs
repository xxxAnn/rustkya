use std::{collections::HashMap, hash::Hash};
use std::fmt;

#[derive(Clone)]
pub enum Decoded {
    Str(String),
    Num(u64),
    Dict(KyaMap),
    List(Vec<Box<Decoded>>)
}

impl fmt::Debug for Decoded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Decoded::Str(s) => s.fmt(f),
            Decoded::Num(i) => i.fmt(f),
            Decoded::Dict(d) => d.fmt(f),
            Decoded::List(l) => l.fmt(f),
        }
    }
}


#[derive(Clone)]
pub struct KyaMap {
    pub k: Vec<Decoded>,
    pub v: Vec<Decoded>
}

impl fmt::Debug for KyaMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.veccify().into_iter()).finish()
    }
}

impl KyaMap {
    pub fn new() -> Self {
        KyaMap {
            k: Vec::new(),
            v: Vec::new()
        }
    }
}

impl KyaMap {
    fn veccify(&self) -> Vec<(Decoded, Decoded)> {
        let mut v = Vec::new();
        for i in 0..self.k.len() {
            v.push((self.k[i].clone(), self.v[i].clone()))
        }
        v
    }
}