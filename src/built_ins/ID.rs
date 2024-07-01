use crate::built_ins::Bool::Bool;
use crate::built_ins::Float::Float;
use crate::built_ins::Integer::Integer;
use crate::built_ins::String::String;
use std::fmt::Write;

#[derive(Clone, Copy, dumpster::Collectable)]
pub struct ID {
    pub x: usize,
}

impl ID {
    pub fn new(x: usize) -> Self {
        Self {
            x,
        }
    }



    pub fn _ZF18N12__push_fmt__Eui(& mut self, s: String, _format: Integer) {
        write!(& mut crate::heap::mut_ref_rc(&s.s), "{}", self.x).unwrap()
    }

    pub fn _ZF10N6__eq__Ei(& self, other: ID) -> Bool {
        return Bool::new(self.x == other.x)
    }

    pub fn _ZF10N6__ne__Ei(& self, other: ID) -> Bool {
        return Bool::new(self.x != other.x)
    }

    pub fn _ZF12N9__float__E(& mut self) -> Float {
        return Float::new(self.x as crate::built_ins::Float::FloatType)
    }

    pub fn _ZF10N7__int__E(& mut self) -> Integer {
        return Integer::new(self.x as crate::built_ins::Integer::IntType);
    }


}