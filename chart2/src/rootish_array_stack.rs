use chart1::interface::List;
use crate::array_stack::ArrayStack;

struct RootishArrayStack<T> {
  blocks: ArrayStack<Box<[T]>>,
  n: usize
}

impl<T: Copy + Default> RootishArrayStack<T> {
  fn i2b(&self, i: usize) -> usize {
    // let i = i as f64;
    let db = (-3.0 + (9.0 + 8.0 * i as f64).sqrt()) / 2.0;
    db.ceil() as usize
  }
}

impl<T: Copy + Default> List<T> for RootishArrayStack<T> {
  fn size(&self) -> usize {
    self.n
  }

  fn get(&self, index: usize) -> Option<T> {
    let b = self.i2b(index);
    let j = index - b*(b+1)/2;
    // self.blocks.get(b)[j]
    todo!()
  }

  fn set(&mut self, index: usize, item: T) -> Option<T> {
    todo!()
  }

  fn add(&mut self, index: usize, item: T) {
    todo!()
  }

  fn remove(&mut self, index: usize) -> Option<T> {
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