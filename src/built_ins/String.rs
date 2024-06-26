
use crate::built_ins::Integer::Integer;
use std::fmt::Write;
use dumpster::Collectable;

#[derive(Clone)]
pub struct String {
    pub s: crate::heap::CellRc<std::string::String>,
}

unsafe impl Collectable for String {
    fn accept<V: dumpster::Visitor>(&self, _: &mut V) -> Result<(), ()> {
        Ok(())
    }
}

impl String {
    pub fn new(s: std::string::String) -> Self {
        return Self {
            s: crate::heap::new_rc(s),
        }
    }

    pub fn _ZF10N7__str__E(& mut self) -> String {
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

    pub fn _ZF18N12__push_fmt__Eui(& mut self, s: String, _format: Integer) {
        write!(& mut crate::heap::mut_ref_rc(&s.s), "{}", crate::heap::mut_ref_rc(&self.s).as_str()).unwrap()
    }

}
