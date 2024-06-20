


pub struct String {
    s: crate::heap::CellRc<std::string::String>,
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

    //todo Add push_int, push_float, etc for all built in types that do not create allocations
}

impl Clone for String {
    fn clone(&self) -> Self {
        return String {
            s: self.s.clone(),
        }
    }
}
