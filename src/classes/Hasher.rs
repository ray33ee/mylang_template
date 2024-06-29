use std::hash::DefaultHasher;
use crate::built_ins::Bytes::Bytes;
use crate::built_ins::Float::Float;
use crate::built_ins::Integer::Integer;
use crate::built_ins::String::String;

#[derive(dumpster::Collectable, Clone)]
pub struct Hasher {
    hasher: DefaultHasher,
}
impl Hasher {
    pub fn _ZF11N8__init__E() -> crate::heap::CellGc<Self> {

        crate::heap::new_gc(Self  {
            hasher: DefaultHasher::new(),
        })
    }
    pub fn _ZF11N8finaliseE(& mut self) -> Integer {
        return Integer::new(std::hash::Hasher::finish(&self.hasher) as i64)
    }

    pub fn _ZF9N5writeEi(& mut self, i: Integer) {
        std::hash::Hasher::write_i64(&mut self.hasher, i.x);
    }

    pub fn _ZF9N5writeEf(& mut self, f: Float) {
        std::hash::Hasher::write(&mut self.hasher, f.x.to_ne_bytes().as_slice());
    }

    pub fn _ZF9N5writeEu(& mut self, s: String) {
        std::hash::Hasher::write(&mut self.hasher, crate::heap::mut_ref_rc(&s.s).as_bytes());
    }

    pub fn _ZF9N5writeEm(& mut self, b: Bytes) {
        std::hash::Hasher::write(&mut self.hasher, crate::heap::mut_ref_rc(&b.b).as_slice());
    }
}
