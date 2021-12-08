use core::mem::size_of;

pub const T_H: f64 = 0.75;
pub const T_0: f64 = 1.00;
pub const P_H: f64 = 0.50;
pub const P_0: f64 = 0.25;
pub const MAX_SPARSENESS: usize = 4; // 1 / P_0 as usize
pub const LARGEST_EMPTY_SEGMENT: usize = 4; // 1 * MAX_SPARSENESS

pub fn ceil_div(a: usize, b: usize) -> usize {
    1 + (a - 1) / b
}

pub fn ceil_log2(a: usize) -> usize {
    last_bit_set(a - 1)
}

pub fn floor_log2(a: usize) -> usize {
    last_bit_set(a) - 1
}

pub fn ceil_hyper(a: usize) -> usize {
    1 << ceil_log2(a)
}

fn last_bit_set(a: usize) -> usize {
    size_of::<usize>() * 8 - a.leading_zeros() as usize
}

