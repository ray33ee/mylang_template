use crate::built_ins::integer::Integer;
use crate::built_ins::bool::Bool;
use dumpster::Collectable;

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

    pub fn _ZF12N9__float__E(& mut self) -> Float {
        return *self
    }

    pub fn _ZF10N7__int__E(& mut self) -> Integer {
        return Integer::new(self.x as crate::built_ins::integer::IntType)
    }

    pub fn _ZF11N7__add__Ef(& mut self, other: Float) -> Float {
        return Float::new(self.x + other.x)
    }

    pub fn _ZF11N8__bool__E(& mut self) -> Bool {
        return Bool::new(self.x != 0.0)
    }

}
