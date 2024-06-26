
use crate::built_ins::Bool::Bool;

use dumpster::Collectable;

#[derive(Clone)]
pub struct Option<O: Collectable + Clone + 'static> {
    pub s: crate::heap::CellGc<std::option::Option<T>>,
}

unsafe impl<O: Collectable + Clone> Collectable for Option<O> {
    fn accept<V: dumpster::Visitor>(&self, visitor: &mut V) -> Result<(), ()> {
        match crate::heap::mut_ref_gc(&self.s) {
            Some(x) => {
                x.accept(visitor)
            },
            None => {
                Ok(())
            }
        }
    }
}

impl<O: Collectable + Clone> Option<O> /*START PARSE HERE*/ {
    pub fn new(item: std::option::Option<O>) -> Option<O> {
        Self {
            s: crate::heap::new_gc(item),
        }
    }

    pub fn _ZF10N7is_noneE(& self) -> Bool {
        return Bool::new(crate::heap::mut_ref_gc(&self.s).is_none())
    }

    pub fn _ZF10N7is_someE(& self) -> Bool {
        return Bool::new(crate::heap::mut_ref_gc(&self.s).is_some())
    }

    pub fn _ZF9N6unwrapE(& self) -> O {
        return crate::heap::mut_ref_gc(&self.s).as_ref().unwrap().clone();
    }
}