use crate::built_ins::Integer::Integer;

#[derive(dumpster::Collectable, Clone)]
pub struct StdOut {

}

impl StdOut {

    pub fn _ZF11N8__init__E() -> crate::heap::CellGc<Self> {
        crate::heap::new_gc(
            Self {

            }
        )
    }

    pub fn _ZF9N5printEi(& mut self, i: Integer) {
        print!("{}", i.x)
    }

}