use crate::built_ins::Integer::Integer;
use crate::built_ins::Float::Float;
use crate::built_ins::String::String;
use dumpster::Collectable;
use std::fmt::Write;
use crate::built_ins::ID::ID;

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

    pub fn _ZF9N6__id__E(& mut self) -> ID {
        return ID::new(crate::heap::ref_id(self));
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

    pub fn get_bool(& self) -> bool {
        return self.x
    }


}
