
mod heap;
mod built_ins;

use dumpster::Collectable;

/* Rust built in functions */

pub fn _ZF17N12print_stringEu(s: crate::built_ins::string::String) {
    println!("{}", crate::heap::mut_ref_rc(&s.s).as_str())
}

/* User code here */

fn main() {
    let mut r = _ZC11N5RangeEiii::_ZF14N8__init__Eiii(built_ins::integer::Integer::new(0), built_ins::integer::Integer::new(10), built_ins::integer::Integer::new(1));
    let mut n = heap::mut_ref_gc(&r)._ZF11N8__next__E();
    let mut n_1 = heap::mut_ref_gc(&r)._ZF11N8__next__E();
    _ZF9N5printEi(n.clone());
    _ZF9N5printEi(n_1.clone());
}
fn _ZF9N5printEi(mut x: built_ins::integer::Integer) {
    _ZF11N6formatEii(x.clone(), built_ins::integer::Integer::new(0));
}
fn _ZF11N6formatEii(mut x: built_ins::integer::Integer, mut conversion: built_ins::integer::Integer) {
    let mut _s = crate::built_ins::string::String::new(std::string::String::from(""));
    x._ZF18N12__push_fmt__Eui(_s.clone(), conversion.clone());
    _ZF17N12print_stringEu(_s.clone());
}
#[derive(Collectable)]
struct _ZC11N5RangeEiii {
    i: built_ins::integer::Integer,
    finish: built_ins::integer::Integer,
    step: built_ins::integer::Integer
}
impl _ZC11N5RangeEiii {
    fn _ZF14N8__init__Eiii(mut start: built_ins::integer::Integer, mut finish: built_ins::integer::Integer, mut step: built_ins::integer::Integer) -> heap::CellGc<Self> {
        let mut _ZVN4self1iEk0 = start.clone();
        let mut _ZVN4self6finishEk0 = finish.clone();
        let mut _ZVN4self4stepEk0 = step.clone();
        heap::new_gc(Self  {
            i: _ZVN4self1iEk0,
            finish: _ZVN4self6finishEk0,
            step: _ZVN4self4stepEk0,
        })
    }
    fn _ZF11N8__next__E(& mut self) -> built_ins::integer::Integer {
        let mut r = self._ZF12N9__get_i__E();
        self._ZF13N9__set_i__Ei(self._ZF12N9__get_i__E()._ZF11N7__add__Ei(self._ZF16N12__get_step__E()));
        return r.clone();
    }
    fn _ZF12N9__get_i__E(& self) -> built_ins::integer::Integer {
        return self.i.clone();
    }
    fn _ZF16N12__get_step__E(& self) -> built_ins::integer::Integer {
        return self.step.clone();
    }
    fn _ZF13N9__set_i__Ei(& mut self, mut i: built_ins::integer::Integer) {
        self.i = i.clone();
    }
}