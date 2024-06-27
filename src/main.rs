// Mylang translator's mangler creates names that does not conform with rust's regulations, so we suppress these warnings
#![allow(non_snake_case)]

// Sometimes we set certain variables as mutable when we dont need to, so we suppress these warnings
#![allow(unused_mut)]

mod heap;
mod built_ins;

/* Rust built in functions */

pub fn _ZF17N12print_stringEu(s: crate::built_ins::String::String) {
    println!("{}", crate::heap::mut_ref_rc(&s.s).as_str())
}

pub fn _ZF17N12panic_stringEu(s: crate::built_ins::String::String) {
    panic!("{}", crate::heap::mut_ref_rc(&s.s).as_str())
}

#[derive(dumpster::Collectable)]
struct _ZC11N7_HasherEi {
    digest: built_ins::Integer::Integer
}
impl _ZC11N7_HasherEi {
    fn _ZF11N8__init__E() -> heap::CellGc<Self> {
        let mut _ZVN4self6digestEk0 = built_ins::Integer::Integer::new(0);
        heap::new_gc(Self  {
            digest: _ZVN4self6digestEk0,
        })
    }
    fn _ZF11N8finaliseE(& mut self) -> built_ins::Integer::Integer {
        return self._ZF18N14__get_digest__E();
    }
    fn _ZF18N14__get_digest__E(& mut self) -> built_ins::Integer::Integer {
        return self.digest.clone();
    }
    pub fn _ZF9N5writeEi(& mut self, i: built_ins::Integer::Integer) {
        let mut t = self._ZF18N14__get_digest__E()._ZF11N7__xor__Ei(i.clone());
        self._ZF19N14__set_digest__Ei(t.clone());
    }

    fn _ZF19N14__set_digest__Ei(& mut self, mut d: built_ins::Integer::Integer) {
        self.digest = d.clone();
    }
}


/* User code here */

