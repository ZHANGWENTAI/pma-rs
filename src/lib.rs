pub mod util;
pub mod pma;

use std::{ptr, mem};

pub fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3, 4, 5];
    let mut v = mem::ManuallyDrop::new(v);

    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe {
        // Overwrite memory with 4, 5, 6
        for i in 0..len as isize {
            ptr::write(p.offset(i), 4 + i);
        }

        // Put everything back together into a Vec
        let rebuilt = Vec::from_raw_parts(p, len, cap);
        assert_eq!(rebuilt, [4, 5, 6]);
    }
}