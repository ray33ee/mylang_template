use crate::built_ins::Float::{Float, FloatType};
use crate::built_ins::Bool::Bool;
use crate::built_ins::String::String;
use dumpster::Collectable;
use std::fmt::Write;
use std::hash::Hasher;
use std::process::ExitCode;
use crate::built_ins::Bytes::Bytes;
use crate::built_ins::ID::ID;
use crate::heap::{CellGc, CellRc};

pub type IntType = i64;

#[derive(Clone, Copy, Collectable)]
pub struct Integer {
    pub x: IntType,
}

impl Integer {

    pub fn new(x: IntType) -> Self {
        Self {
            x,
        }
    }

    pub fn _ZF9N6__id__E(& mut self) -> ID {
        return ID::new(crate::heap::ref_id(self));
    }

    pub fn _ZF12N9__float__E(& mut self) -> Float {
        return Float::new(self.x as crate::built_ins::Float::FloatType)
    }

    pub fn _ZF10N7__int__E(& mut self) -> Integer {
        return *self
    }

    pub fn _ZF11N7__add__Ei(& mut self, other: Integer) -> Integer {
        return Integer::new(self.x + other.x)
    }

    pub fn _ZF11N8__bool__E(& mut self) -> Bool {
        return Bool::new(self.x != 0)
    }

    pub fn _ZF11N8__real__E(& mut self) -> Integer {
        return *self
    }

    pub fn _ZF11N8__imag__E(& mut self) -> Integer {
        return Integer::new(0)
    }

    pub fn _ZF18N12__push_fmt__Eui(& mut self, s: String, _format: Integer) {
        write!(& mut crate::heap::mut_ref_rc(&s.s), "{}", self.x).unwrap()
    }

    pub fn _ZF10N6__eq__Ei(& self, other: Integer) -> Bool {
        return Bool::new(self.x == other.x)
    }

    pub fn _ZF10N6__ne__Ei(& self, other: Integer) -> Bool {
        return Bool::new(self.x != other.x)
    }

    pub fn _ZF10N6__ge__Ei(& self, other: Integer) -> Bool {
        return Bool::new(self.x >= other.x)
    }

    pub fn _ZF10N6__gt__Ei(& self, other: Integer) -> Bool {
        return Bool::new(self.x > other.x)
    }

    pub fn _ZF10N6__le__Ei(& self, other: Integer) -> Bool {
        return Bool::new(self.x <= other.x)
    }

    pub fn _ZF10N6__lt__Ei(& self, other: Integer) -> Bool {
        return Bool::new(self.x < other.x)
    }



    pub fn _ZF10N6__ge__Ef(& self, other: Float) -> Bool {
        return Bool::new(self.x as FloatType >= other.x)
    }

    pub fn _ZF10N7__one__E(& self) -> Integer {
        return Integer::new(1);
    }

    pub fn _ZF11N7__mul__Ei(&self, other: Integer) -> Integer {
        return Integer::new(self.x * other.x)
    }

    pub fn _ZF11N7__sub__Ei(&self, other: Integer) -> Integer {
        return Integer::new(self.x - other.x)
    }

    pub fn _ZF11N7__mod__Ei(&self, other: Integer) -> Integer {
        return Integer::new(self.x % other.x)
    }

    pub fn _ZF11N7__xor__Ei(&self, other: Integer) -> Integer {
        return Integer::new(self.x ^ other.x)
    }

    pub fn _ZF17N12__floordiv__Ei(&self, other: Integer) -> Integer {
        return Integer::new(self.x / other.x)
    }

    pub fn _ZF19N14__push_bytes__Em(& mut self, b: Bytes) {
        crate::heap::mut_ref_rc(&b.b).extend_from_slice(self.x.to_ne_bytes().as_slice())
    }

    pub fn _ZF22N8__hash__EC9N6HasherE(& mut self, h: CellGc<crate::classes::Hasher::Hasher>) {
        crate::heap::mut_ref_gc(&h)._ZF9N5writeEi(self.clone());
    }

}

impl std::process::Termination for Integer {
    fn report(self) -> ExitCode {
        ExitCode::from(self.x as u8)
    }
}

/*impl std::hash::Hash for Integer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self._ZF22N8__hash__EC9N6HasherE()
    }
}*/
