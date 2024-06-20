
mod heap;
mod built_ins;

use dumpster::Collectable;

fn main() {
    let mut x = built_ins::integer::Integer::new(1);
    let mut y = built_ins::bool::Bool::new(true);
    let mut a = { let mut s = crate::built_ins::string::String::new(std::string::String::new()); s.push_slice("hello "); s._ZF12N8push_strEu(x._ZF11N7__add__Ei(built_ins::integer::Integer::new(1))._ZF10N7__str__E()); s.push_slice(" and "); s._ZF12N8push_strEu(y._ZF10N7__str__E()); s };
    println!("{}", a.as_str())
}