use crate::built_ins::integer::Integer;
use crate::built_ins::float::Float;
use crate::built_ins::string::String;
use dumpster::Collectable;

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

    pub fn _ZF10N7__str__E(& mut self) -> String {
        return String::new(if self.x {std::string::String::from("True")} else {std::string::String::from("False")})
    }



}
