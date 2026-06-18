#![no_std]

extern crate alloc;

mod binary {
    mod instructions;
    mod modules;
    mod parser;
    mod types;
    mod values;
}

#[deprecated]
enum Todo {}
