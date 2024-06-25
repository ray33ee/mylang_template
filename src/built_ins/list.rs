use dumpster::{Collectable, Visitor};
use crate::built_ins::integer::{Integer, IntType};
use crate::heap;

#[derive(Clone)]
pub struct List<T: Collectable + Clone + 'static> {
    l: heap::CellGc<Vec<T>>,
}

impl<T: Clone + Collectable> List<T> {

    #[allow(dead_code)]
    pub fn new(v: Vec<T>) -> Self {
        Self {
            l: heap::new_gc(v),
        }
    }

    pub fn append(& mut self, item: T) {
        heap::mut_ref_gc(&self.l).push(item);
    }

    pub fn _ZF16N11__getitem__Ei(& mut self, index: Integer) -> T {
        return heap::mut_ref_gc(&self.l)[index.x as usize].clone()
    }

    pub fn _ZF10N7__len__E(& self) -> Integer {
        return Integer::new(heap::mut_ref_gc(&self.l).len() as IntType)
    }

}

unsafe impl<T: Clone + Collectable> Collectable for List<T> {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), ()> {
        heap::mut_ref_gc(&self.l).accept(visitor)
    }
}

