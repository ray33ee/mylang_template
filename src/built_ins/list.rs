use dumpster::{Collectable, Visitor};
use crate::built_ins::integer::Integer;
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

    pub fn _ZF10N6appendE(& mut self, item: T) {
        heap::mut_ref_gc(&self.l).push(item);
    }

    pub fn _ZF16N11__getitem__Ei(& mut self, index: Integer) -> T {
        return heap::mut_ref_gc(&self.l)[index.x as usize].clone()
    }

}

unsafe impl<T: Clone + Collectable> Collectable for List<T> {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), ()> {
        heap::mut_ref_gc(&self.l).accept(visitor)
    }
}
