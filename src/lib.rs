#![no_std]

extern crate alloc;

pub mod binary {
    mod instructions;
    pub mod modules;
    mod parser;
    mod types;
    mod values;
}
