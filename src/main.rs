// Mylang translator's mangler creates names that does not conform with rust's regulations, so we suppress these warnings
#![allow(non_snake_case)]

// Sometimes we set certain variables as mutable when we dont need to, so we suppress these warnings
#![allow(unused_mut)]

mod heap;
mod built_ins;
mod classes;

/* Rust built in functions */

pub fn _ZF17N12print_stringEu(s: crate::built_ins::String::String) {
    println!("{}", crate::heap::mut_ref_rc(&s.s).as_str())
}

pub fn _ZF17N12panic_stringEu(s: crate::built_ins::String::String) {
    panic!("{}", crate::heap::mut_ref_rc(&s.s).as_str())
}


/* User code here */

