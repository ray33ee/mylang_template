
use crate::built_ins::integer::Integer;
use std::fmt::Write;
use dumpster::Collectable;

#[derive(Clone)]
pub struct String {
    pub s: crate::heap::CellRc<std::string::String>,
}

unsafe impl Collectable for String {
    fn accept<V: dumpster::Visitor>(&self, visitor: &mut V) -> Result<(), ()> {
        Ok(())
    }
}

impl String {
    pub fn new(s: std::string::String) -> Self {
        return Self {
            s: crate::heap::new_rc(s),
        }
    }

    pub fn _ZF10N7__str__E(& mut self) -> Self {
        self.clone()
    }

    pub fn push_slice(& mut self, s: & str) {
        crate::heap::mut_ref_rc(&self.s).push_str(s)
    }

    pub fn _ZF12N8push_strEu(& mut self, s: String) {
        self.push_slice(crate::heap::mut_ref_rc(&s.s).as_str())
    }

    pub fn as_str(& mut self) -> & str {
        crate::heap::mut_ref_rc(&self.s).as_str()
    }

    pub fn _ZF18N12__push_fmt__Eui(& mut self, s: String, format: Integer) {
        write!(& mut crate::heap::mut_ref_rc(&s.s), "{}", crate::heap::mut_ref_rc(&self.s).as_str()).unwrap()
    }

}
