use crate::built_ins::float::Float;
use crate::built_ins::bool::Bool;
use dumpster::Collectable;

pub type IntType = i64;

#[derive(Clone, Copy, Collectable)]
pub struct Integer {
    x: IntType,
}

impl Integer {

    pub fn new(x: IntType) -> Self {
        Self {
            x,
        }
    }

    pub fn _ZF12N9__float__E(& mut self) -> Float {
        return Float::new(self.x as crate::built_ins::float::FloatType)
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

}
