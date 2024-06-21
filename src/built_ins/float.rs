use crate::built_ins::integer::Integer;
use crate::built_ins::bool::Bool;
use crate::built_ins::string::String;
use dumpster::Collectable;
use std::fmt::Write;

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

    pub fn _ZF18N12__push_fmt__Eui(& mut self, s: String, format: Integer) {
        write!(& mut crate::heap::mut_ref_rc(&s.s), "{}", self.x).unwrap()
    }

}
