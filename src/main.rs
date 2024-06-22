
mod heap;
mod built_ins;

use dumpster::Collectable;

/* Rust built in functions */

pub fn _ZF17N12print_stringEu(s: crate::built_ins::string::String) {
    println!("{}", crate::heap::mut_ref_rc(&s.s).as_str())
}

/* User code here */
