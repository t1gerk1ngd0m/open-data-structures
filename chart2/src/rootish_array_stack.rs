use crate::array_stack::Stack;
use crate::array_stack::ArrayStack;

pub trait Rootish<T> {
  fn size(&self) -> usize;
  fn get(&self, index: usize) -> T;
  fn set(&mut self, index: usize, item: T) -> T;
  fn add(&mut self, index: usize, item: T);
  fn remove(&mut self, index: usize) -> T;
}

#[derive(Default)]
struct RootishArrayStack<T> {
  blocks: ArrayStack<Box<[T]>>,
  n: usize
}

impl<T: Copy + Default> RootishArrayStack<T> {
  // fn new() -> Self {
  //   Self {
  //     blocks: ArrayStack::new(),
  //     n: 0
  //   }
  // }

  fn i2b(&self, i: usize) -> usize {
    // let i = i as f64;
    let db = (-3.0 + (9.0 + 8.0 * i as f64).sqrt()) / 2.0;
    db.ceil() as usize
  }
}

impl<T: Copy + Default> Rootish<T> for RootishArrayStack<T> {
  fn size(&self) -> usize {
    self.n
  }

  fn get(&self, index: usize) -> T {
    let b = self.i2b(index);
    let j = index - b*(b+1)/2;
    self.blocks.get(b)[j]
  }

  fn set(&mut self, index: usize, item: T) -> T {
    todo!()
  }

  fn add(&mut self, index: usize, item: T) {
    todo!()
  }

  fn remove(&mut self, index: usize) -> T {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get() {

  }
}