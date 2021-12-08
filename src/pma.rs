use crate::util::*;
use core::alloc::Layout;
use core::mem::size_of;
use core::ptr;
use std::alloc::{alloc, realloc};

pub struct PackedMemoryArray {
    array: *mut i32,
    capacity: usize,
    len: usize,
    layout: Layout,
    height: usize,
    segment_size: usize,
    num_segments: usize,
    delta_t: f64,
    delta_p: f64,
}

impl PackedMemoryArray {
    pub fn new() -> PackedMemoryArray {
        PackedMemoryArray {
            array: unsafe {
                alloc(Layout::array::<i32>(LARGEST_EMPTY_SEGMENT).unwrap()) as *mut i32
            },
            capacity: 1 << LARGEST_EMPTY_SEGMENT,
            len: 0,
            layout: Layout::array::<i32>(LARGEST_EMPTY_SEGMENT).unwrap(),
            height: 1,
            segment_size: LARGEST_EMPTY_SEGMENT,
            num_segments: (1 << LARGEST_EMPTY_SEGMENT) / LARGEST_EMPTY_SEGMENT,
            delta_t: (T_0 - T_H) as f64,
            delta_p: (P_H - P_0) as f64,
        }
    }

    pub fn find(&self, val: i32, index: *mut isize) -> bool {
        let mut left: isize = 0;
        let mut right: isize = (self.capacity - 1) as isize;
        unsafe {
            while left <= right {
                let mid = left + (right - left) / 2;
                let mut i = mid;
                while i >= left && *self.array.offset(i) == 0 {
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
            while *index >= 0 && self.is_empty_at(*index) {
                *index -= 1;
            }
        }
        false
    }

    pub fn insert(&mut self, value: i32) -> bool {
        let index: *mut isize = ptr::null_mut();
        if !self.find(value, index) {
            unsafe {
                self.insert_after(value, *index);
            }
            return true;
        }
        false
    }

    fn insert_after(&mut self, value: i32, mut index: isize) {
        assert!(index < self.capacity as isize);
        assert!(index >= 0 && self.is_empty_at(index) || index >= -1);

        let mut j = index + 1;
        while j < self.capacity as isize && self.is_empty_at(index) {
            j += 1;
        }

        if j < self.capacity as isize {
            unsafe {
                ptr::copy(
                    self.array.offset(index + 1),
                    self.array.offset(index + 2),
                    (j - index - 1) as usize,
                );
                self.array.offset(index + 1).write(value);
            }
            index += 1;
        } else {
            j = index - 1;
            while j >= 0 && self.is_empty_at(j) {
                j -= 1;
            }
            if j >= 0 {
                unsafe {
                    ptr::copy(
                        self.array.offset(index),
                        self.array.offset(index - 1),
                        (index - j - 1) as usize,
                    );
                    self.array.offset(j).write(value);
                }
            }
        }
        self.len += 1;
        self.rebalance(index);
    }

    pub fn delete(&mut self, value: i32) -> bool {
        let index: *mut isize = ptr::null_mut();
        if self.find(value, index) {
            unsafe {
                self.delete_at(*index);
            }
            return true;
        }
        false
    }

    fn delete_at(&mut self, index: isize) {
        assert!(index < self.capacity as isize);
        assert!(index >= 0);
        self.clear_at(index);
        self.len -= 1;
        self.rebalance(index);
    }

    fn is_empty_at(&self, index: isize) -> bool {
        assert!(index < self.capacity as isize);
        assert!(index >= 0);
        unsafe {
            return *self.array.offset(index) == 0;
        }
    }

    fn clear_at(&mut self, index: isize) {
        assert!(index < self.capacity as isize);
        assert!(index >= 0);
        unsafe {
            *self.array.offset(index) = 0;
        }
    }

    fn rebalance(&mut self, index: isize) {
        let mut window_start: isize;
        let mut window_end: isize;
        let mut height: usize = 0;
        let mut occupancy: usize = if self.is_empty_at(index) { 0 } else { 1 };
        let mut left_index: isize = index - 1;
        let mut right_index: isize = index + 1;
        let mut density: f64;
        let mut t_height: f64;
        let mut p_height: f64;

        loop {
            let window_size: usize = self.len * (1 << height);
            let window = index / (window_size as isize);
            window_start = window * window_size as isize;
            window_end = window_start + window_size as isize;
            while left_index >= window_start {
                if self.is_empty_at(left_index) {
                    occupancy += 1;
                }
                left_index -= 1;
            }
            while right_index < window_end {
                if self.is_empty_at(right_index) {
                    occupancy += 1;
                }
                right_index += 1;
            }
            density = occupancy as f64 / (window_size as f64);
            t_height = T_0 - (height as f64 * self.delta_t);
            p_height = P_0 + (height as f64 * self.delta_p);
            height += 1;

            if (density < p_height || density >= t_height) && height < self.height {
                break;
            }
        }

        // found tha within the threshold
        if density >= p_height && density < t_height {
            self.pack(window_start, window_end, occupancy);
            self.spread(window_start, window_end, occupancy);
        } else {
            self.resize();
        }
    }

    // [from, to)
    fn pack(&mut self, from: isize, to: isize, n: usize) {
        assert!(from < to);
        let mut read_index: isize = from;
        let mut write_index: isize = from;
        while read_index < to {
            if !self.is_empty_at(read_index) {
                if read_index > write_index {
                    unsafe {
                        ptr::copy(
                            self.array.offset(read_index),
                            self.array.offset(write_index),
                            1,
                        );
                    }
                    self.clear_at(read_index);
                }
                write_index += 1;
            }
            read_index += 1;
        }
        assert!(n == (write_index - from) as usize);
    }

    fn spread(&mut self, from: isize, to: isize, n: usize) {
        assert!(from < to);
        let capacity = to - from;
        let frequency = (capacity << 8) / n as isize;
        let mut read_index: isize = from + n as isize - 1;
        let mut write_index: isize = (to << 8) - frequency;
        while write_index >> 8 > read_index {
            unsafe {
                ptr::copy(
                    self.array.offset(read_index),
                    self.array.offset(write_index),
                    1,
                );
            }
            self.clear_at(read_index);
            read_index -= 1;
            write_index -= frequency;
        }
    }

    fn resize(&mut self) {
        self.pack(0, self.capacity as isize, self.len);
        self.compute_capacity();
        self.height = floor_log2(self.num_segments) + 1;
        self.delta_t = (T_0 - T_H) / (self.height as f64);
        self.delta_p = (P_H - P_0) / (self.height as f64);

        self.array = unsafe {
            realloc(
                self.array as *mut u8,
                self.layout,
                self.capacity * size_of::<i32>(),
            ) as *mut i32
        };
        self.spread(0, self.capacity as isize, self.len);
    }

    fn compute_capacity(&mut self) {
        self.segment_size = ceil_log2(self.len);
        self.num_segments = ceil_div(self.len, self.segment_size);
        self.num_segments = ceil_hyper(self.num_segments);
        self.segment_size = ceil_div(self.len, self.num_segments) * MAX_SPARSENESS;
        self.capacity = self.segment_size * self.num_segments * MAX_SPARSENESS;

        assert!(self.capacity < usize::MAX);
        assert!(self.capacity > self.len);
    }
}
