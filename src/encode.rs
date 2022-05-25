use crate::types::{Decoded, KyaMap};

pub trait Encodable {
    fn encode(&self) -> String;
}

impl Encodable for Decoded {
    fn encode(&self) -> String {
        match self {
            Decoded::Str(s) => format!("'s{}'", s),
            Decoded::Num(i) => format!("'i{}'", i),
            Decoded::Dict(d) => d.encode(),
            Decoded::List(l) => l.encode(),
        }
    }
}
impl Encodable for KyaMap {
    fn encode(&self) -> String {
        let mut s = String::from('^');
        for i in 0..self.k.len() {
            s = format!("{}{}{}", s, self.k[i].encode(), self.v[i].encode());
        }
        s.push('$');
        s
    }
}

impl<T> Encodable for Box<T>
where T: Encodable {
    fn encode(&self) -> String {
        self.as_ref().encode()
    }
}

impl<T> Encodable for Vec<T>
where T: Encodable {
    fn encode(&self) -> String {
        let mut s = String::from('<');
        for o in self.into_iter() {
            s = format!("{}{}", s, o.encode());
        }
        s.push('>');
        s
    }
}