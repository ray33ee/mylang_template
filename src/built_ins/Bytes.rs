use dumpster::{Collectable, Visitor};
use crate::built_ins::Integer::{Integer, IntType};
use crate::heap;

#[derive(Clone)]
pub struct Bytes {
    pub b: heap::CellRc<Vec<u8>>,
}

impl Bytes {

    #[allow(dead_code)]
    pub fn new(v: Vec<u8>) -> Self {
        Self {
            b: heap::new_rc(v),
        }
    }

    pub fn _ZF10N6appendEi(& mut self, item: Integer) {
        heap::mut_ref_rc(&self.b).push(item.x as u8);
    }

    pub fn _ZF16N11__getitem__Ei(& mut self, index: Integer) -> Integer {
        return Integer::new(heap::mut_ref_rc(&self.b)[index.x as usize] as IntType)
    }

    pub fn _ZF10N7__len__E(& self) -> Integer {
        return Integer::new(heap::mut_ref_rc(&self.b).len() as IntType)
    }

}

unsafe impl Collectable for Bytes{
    fn accept<V: Visitor>(&self, _: &mut V) -> Result<(), ()> {
        Ok(())
    }
}
