use std::cell::{Ref, RefMut, RefCell};

// # Collection Utilities

pub trait Push<T> {
    fn push(&mut self, value: T);
}

impl<T> Push<T> for Vec<T> {
    fn push(&mut self, value: T) {
        Vec::push(self, value);
    }
}

// # `SegVec`

pub struct SegVecRoot<T> {
    vec_cell: RefCell<Vec<T>>
}

impl<T> Default for SegVecRoot<T> {
    fn default() -> Self {
        SegVecRoot { vec_cell: RefCell::new(Vec::new()) }
    }
}

impl<T> SegVecRoot<T> {
    pub fn extend<'a>(&'a mut self) -> SegVec<'a, T> {
        SegVec::new(&mut self.vec_cell)
    }
}

pub struct SegVec<'a, T> 
{
    vec_cell: &'a RefCell<Vec<T>>,
    begin: usize
}

impl<'a, T> SegVec<'a, T> {

    // Construct

    pub fn new(vec: &'a mut RefCell<Vec<T>>) -> Self {
        let begin = vec.borrow().len();
        Self { vec_cell: vec, begin }
    }
    
    // Mutate
    
    pub fn extend<'b, 'c>(&'b mut self) -> SegVec<'c, T> 
    where 'a: 'b, 'b: 'c 
    {
        let begin = self.vec_cell.borrow().len();
        SegVec { vec_cell: self.vec_cell, begin }
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut vec = self.vec_cell.borrow_mut();
        if self.begin < vec.len() { return vec.pop(); }
        return None;
    }

    pub fn as_mut_slice<'b, 'c>(&'b self) -> RefMut<'c, [T]>
    where 'a: 'b, 'b: 'c
    {
        let begin = self.begin;
        RefMut::map(self.vec_cell.borrow_mut(), 
            |r| &mut r.as_mut_slice()[begin..])
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        let mut vec = self.vec_cell.borrow_mut();
        let removed = vec.swap_remove(self.begin + index);
        return removed;
    }    

    // Inspect

    pub fn len(&self) -> usize { self.as_slice().len() }

    pub fn as_slice<'b, 'c>(&'b self) -> Ref<'c, [T]>
    where 'a: 'b, 'b: 'c
    {
        let begin = self.begin;
        Ref::map(self.vec_cell.borrow(), |r| &r.as_slice()[begin..])
    }

    pub fn is_empty(&self) -> bool { self.as_slice().is_empty() }    
}

impl<'a, T> Drop for SegVec<'a, T> {
    fn drop(&mut self) {
        self.vec_cell.borrow_mut().truncate(self.begin);
    }
}

impl<'a, T> Push<T> for SegVec<'a, T> {
    fn push(&mut self, value: T) {
        self.vec_cell.borrow_mut().push(value);
    }
}

// # Control Flow Utilities

#[macro_export]
macro_rules! assert_matches {
    ($e:expr, $p:pat) => {
        let $p = $e 
        else { panic!("{:?} did not match pattern {}", $e, core::stringify!($p)); };
    }
}

// # IO utilities

pub fn read_u32_le(stream: &mut impl std::io::Read) -> std::io::Result<u32> {
    let mut buf: [u8; 4] = [0; 4];
    stream.read_exact(&mut buf)?;
    let value = u32::from_le_bytes(buf);
    return Ok(value);
}

pub struct Dispose;

impl std::io::Write for Dispose {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { Ok(buf.len()) }

    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}
