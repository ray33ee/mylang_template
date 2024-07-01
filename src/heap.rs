
use std::cell::UnsafeCell;
use std::rc::Rc;

use dumpster::unsync::Gc;
use dumpster::Collectable;


// Reference counted cell type aliases for improved readability
pub type CellGc<T> = Gc<UnsafeCell<T>>;
pub type CellRc<T> = Rc<UnsafeCell<T>>;

// Functions used to obtain a mutable reference from an unsafe cell for improved readability
#[allow(dead_code)]
pub fn mut_ref_gc<T: Collectable>(t: & CellGc<T>) -> & mut T {
    unsafe { UnsafeCell::<_>::get(t).as_mut().unwrap() }
}

#[allow(dead_code)]
pub fn mut_ref_rc<T>(t: & CellRc<T>) -> & mut T {
    unsafe { UnsafeCell::<_>::get(t).as_mut().unwrap() }
}

// Functions to create new CellGc and CellRc objects for improved readability
#[allow(dead_code)]
pub fn new_gc<T: Collectable>(t: T) -> CellGc<T> {
    Gc::new(UnsafeCell::new(t))
}

#[allow(dead_code)]
pub fn new_rc<T>(t: T) -> CellRc<T> {
    Rc::new(UnsafeCell::new(t))
}

//Get the IDs of gc and rc objects
#[allow(dead_code)]
pub fn gc_id<T: Collectable>(t: & CellGc<T>) -> usize {
    return Gc::<_>::as_ptr(t) as usize;
}

#[allow(dead_code)]
pub fn rc_id<T: Collectable>(t: & CellRc<T>) -> usize {
    return Rc::<_>::as_ptr(t) as usize;
}

pub fn ref_id<T>(t: &T) -> usize {
    return t as * const T as usize
}

#[allow(dead_code)]
pub fn filthy_cast_to_rc<T>(t: & T) -> CellRc<T> {
    filthy_cast_to_rgc::<T, CellRc<T>>(t, 2)
}

#[allow(dead_code)]
pub fn filthy_cast_to_gc<T: Collectable>(t: & T) -> CellGc<T> {
    filthy_cast_to_rgc::<T, CellGc<T>>(t, 1)
}

#[allow(dead_code)]
fn filthy_cast_to_rgc<T, R: Clone>(t: & T, number_of_counts: isize) -> R {
    unsafe {
        // Convert the reference to t into a pointer to t
        let t_pointer = t as * const T;

        // convert this pointer to t into a pointer to bytes (used for offset function)
        let bytes_pointer = t_pointer as * const u8;

        // The RcBox type contains two usize counts (strong and weak counts) then the data.
        // So to go from the data to the beginning of the RcBox struct, we subtract two usizes worth of bytes from the pointer.
        // For the GcBox, we only have one usize (a single strong count) and so we subtract one usize.
        // We supply the number of usizes to subtract as the argument `number_of_counts`
        let box_begin = bytes_pointer.offset(std::mem::size_of::<usize>() as isize * -number_of_counts);

        //Magical jiggery pokery to convert pointer to u8 into a pointer to Rc<...>
        let rc_ref = &box_begin as *const * const u8;

        let rc_pointer = rc_ref as * const R;

        //Get a regular reference to the Rc<...> object
        let rc_ref = &*rc_pointer;

        //Clone the Rc object. This provides a reference to T that respects the reference counting rules
        rc_ref.clone()
    }
}