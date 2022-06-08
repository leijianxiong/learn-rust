
use std::collections::BTreeMap;
use core::cmp::{Ord};

#[derive(Debug, Ord, Eq, PartialEq)]
struct Name {
    pub name: String,
    pub flags: u32,
}

impl Name {
    pub fn new(name: impl AsRef<str>, flags: u32) -> Self {
        Self {
            name: name.as_ref().to_string(),
            flags,
        }
    }
}

// impl PartialOrd<Name> for Name {
//     fn partial_cmp(&self, other: &Name) -> Option<std::cmp::Ordering> {
//         Some((self.flags, &self.name).cmp(&(other.flags, &other.name)))
//     }
// }

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((&self.name, self.flags).cmp(&(&other.name, other.flags)))
        // match self.name.partial_cmp(&other.name) {
        //     Some(core::cmp::Ordering::Equal) => {}
        //     ord => return ord,
        // }
        // self.flags.partial_cmp(&other.flags)
    }
}


fn main() {
    let mut map = BTreeMap::new();
    map.insert(Name::new("/etc/password", 0x1), 12);
    map.insert(Name::new("/etc/hosts", 0x1), 4);
    map.insert(Name::new("/home/tchen", 0x0), 28);
    
    for item in map.iter() {
        println!("{:?}", item);
    }
}
