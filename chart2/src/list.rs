pub trait List<T> {
  fn size(&self) -> usize;
  fn get(&self, index: usize) -> T;
  fn set(&mut self, index: usize, item: T) -> T;
  fn add(&mut self, index: usize, item: T);
  fn remove(&mut self, index: usize) -> T;
}
