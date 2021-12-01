use core::ptr;
use crate::util::*;

pub struct PackedMemoryArray {
    array: *mut i32,
    capacity: usize,
    len: usize,
    height: usize,
}

impl PackedMemoryArray {
    pub fn new() -> PackedMemoryArray {
        PackedMemoryArray {
            array: ptr::null_mut(),
            capacity: 0,
            len: 0,
            height: 0,
        }
    }

    pub fn find(&self, val: i32, index: *mut isize) -> bool {
        let mut left: isize = 0; 
        let mut right: isize = (self.capacity - 1) as isize;
        unsafe {
            while left <= right {
                let mid = left + (right - left) / 2;
                let mut i = mid;
                while i >= left && self.array.offset(i).is_null() {
                    i -= 1;
                }
                if i < left {
                    left = mid + 1;
                } else {
                    if *self.array.offset(i) == val {
                        return true;
                    } else if *self.array.offset(i) > val {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                }
            } 
            *index = right;
            while *index >= 0 && self.array.offset(*index).is_null() {
                *index -= 1;
            }
        }
        false
    }

    pub fn insert (&mut self, value: i32) -> bool{
        let index: *mut isize = ptr::null_mut();
        if !self.find(value, index) {
            unsafe {self.insert_after(value, *index);}
            return true;
        }
        false
    }

    
    fn insert_after(&mut self, value: i32, mut index: isize) {
        assert!(index < self.capacity as isize);
        unsafe {
            assert!(index >= 0 && !self.array.offset(index).is_null() || index >= -1);
        }

        let mut j = index + 1;
        while j < self.capacity as isize && unsafe {!self.array.offset(j).is_null()} {
            j += 1;
        }

        if j < self.capacity as isize {
            unsafe {
                ptr::copy(self.array.offset(index + 1), self.array.offset(index + 2), (j - index - 1) as usize);
                self.array.offset(index + 1).write(value);
            }
            index += 1;
        } else {
            j = index - 1;
            while j >= 0 && unsafe {!self.array.offset(j).is_null()} {
                j -= 1;
            }
            if j >= 0 {
                unsafe {
                    ptr::copy(self.array.offset(index), self.array.offset(index - 1), (index - j - 1) as usize);
                    self.array.offset(j).write(value);
                }
            }
        }
        self.len += 1;
        self.rebalance(index);
    }

    pub fn delete (&mut self, value: i32) -> bool {
        let index: *mut isize = ptr::null_mut();
        if self.find(value, index) {
            unsafe {self.delete_at(*index);}
            return true;
        }
        false
    }

    fn delete_at(&mut self, index: isize) {
        assert!(index < self.capacity as isize);
        assert!(index >= 0);
        unsafe {
            let mut d_ptr: *mut i32 = self.array.offset(index);
            d_ptr = ptr::null_mut();
        }
        self.len -= 1;
        self.rebalance(index);
    }

    fn rebalance(&self, index: isize) {
        // todo
    }
}