use std::cmp;

#[inline(always)]
pub fn min3(a: usize, b: usize, c: usize) -> usize {
   return cmp::min(cmp::min(a, b), c);
}

#[inline(always)]
pub fn min4(a: usize, b: usize, c: usize, d: usize) -> usize {
   return cmp::min(cmp::min(cmp::min(a, b), c), d); 
}