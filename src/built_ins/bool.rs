use crate::built_ins::integer::Integer;
use crate::built_ins::float::Float;
use crate::built_ins::string::String;
use dumpster::Collectable;
use std::fmt::Write;

pub type BoolType = bool;

#[derive(Clone, Copy, Collectable)]
pub struct Bool {
    pub x: BoolType,
}

impl Bool {

    pub fn new(x: BoolType) -> Self {
        Self {
            x,
        }
    }

    pub fn _ZF12N9__float__E(& mut self) -> Float {
        if self.x {
            Float::new(1.0)
        } else {
            Float::new(0.0)
        }
    }

    pub fn _ZF10N7__int__E(& mut self) -> Integer {
        if self.x {
            Integer::new(1)
        } else {
            Integer::new(0)
        }
    }

    pub fn _ZF11N8__bool__E(& mut self) -> Bool {
        return *self
    }

    pub fn _ZF18N12__push_fmt__Eui(& mut self, s: String, _format: Integer) {
        write!(& mut crate::heap::mut_ref_rc(&s.s), "{}", self.x).unwrap()
    }



}
