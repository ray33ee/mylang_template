use crate::built_ins::Integer::Integer;
use crate::built_ins::Bool::Bool;
use crate::built_ins::String::String;
use dumpster::Collectable;
use std::fmt::Write;
use crate::built_ins::Bytes::Bytes;
use crate::built_ins::ID::ID;
use crate::heap::CellGc;

pub type FloatType = f64;

#[derive(Clone, Copy, Collectable)]
pub struct Float {
    pub x: FloatType,
}

impl Float {

    pub fn new(x: FloatType) -> Self {
        Self {
            x,
        }
    }

    pub fn _ZF9N6__id__E(& mut self) -> ID {
        return ID::new(crate::heap::ref_id(self));
    }

    pub fn _ZF12N9__float__E(& mut self) -> Float {
        return *self
    }

    pub fn _ZF10N7__int__E(& mut self) -> Integer {
        return Integer::new(self.x as crate::built_ins::Integer::IntType)
    }

    pub fn _ZF11N7__add__Ef(& mut self, other: Float) -> Float {
        return Float::new(self.x + other.x)
    }

    pub fn _ZF11N7__add__Ei(& mut self, other: Integer) -> Float {
        return Float::new(self.x + other.x as FloatType)
    }

    pub fn _ZF11N8__bool__E(& mut self) -> Bool {
        return Bool::new(self.x != 0.0)
    }

    pub fn _ZF11N8__real__E(& mut self) -> Float {
        return *self
    }

    pub fn _ZF11N8__imag__E(& mut self) -> Float {
        return Float::new(0.0)
    }

    pub fn _ZF10N7__str__E(& mut self) -> String {
        return String::new(self.x.to_string())
    }

    pub fn _ZF18N12__push_fmt__Eui(& mut self, s: String, _format: Integer) {
        write!(& mut crate::heap::mut_ref_rc(&s.s), "{}", self.x).unwrap()
    }

    pub fn _ZF22N8__hash__EC9N6HasherE(& mut self, h: CellGc<crate::classes::Hasher::Hasher>) {
        crate::heap::mut_ref_gc(&h)._ZF9N5writeEf(self.clone());
    }

}
