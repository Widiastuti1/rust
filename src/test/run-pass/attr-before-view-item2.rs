#![allow(unused_attributes)]

// pretty-expanded FIXME #23616

#![feature(custom_attribute, test)]

mod m {
    #[foo = "bar"]
    extern crate test;
}

pub fn main() {
}
