use chart1::interface::List;

#[derive(Clone, Debug, Default)]
pub struct BoundedDeque<T> {
  a: Box<[Option<T>]>,
  j: usize,
  n: usize
}

impl<T> BoundedDeque<T> {
  pub fn limit(&self) -> usize {
    self.a.len()
  }

  pub fn new(b: usize) -> Self {
    Self {
      a: Self::allocate_in_heap(b),
      j: 0,
      n: 0
    }
  }

  fn allocate_in_heap(size: usize) -> Box<[Option<T>]> {
    std::iter::repeat_with(Default::default)
      .take(size)
      .collect::<Vec<_>>()
      .into_boxed_slice()
  }
}

impl<T: Clone> List<T> for BoundedDeque<T> {
  fn size(&self) -> usize {
    self.n
  }

  fn get(&self, index: usize) -> Option<T> {
    self.a.get((self.j + index) % self.limit())?.as_ref().cloned()
  }

  fn set(&mut self, index: usize, item: T) -> Option<T> {
    self.a.get_mut((self.j + index) % self.limit())?.replace(item)
  }

  fn add(&mut self, index: usize, item: T) {
    if index < self.n/2 {
      self.j = if self.j == 0 { self.limit() - 1 } else { self.j - 1 };
      for k in 0..index {
        self.a[(self.j+k) % self.limit()] = self.a[(self.j+k+1) % self.limit()].take()
      }
    } else {
      for k in (index..=self.size()).rev() {
        self.a[(self.j+k) % self.limit()] = self.a[(self.j+k-1) % self.limit()].take()
      }
    }
    self.a[(self.j+index) % self.limit()] = Some(item);
    self.n += 1
  }

  fn remove(&mut self, index: usize) -> Option<T> {
    if index >= self.n {
      None
  } else {
      let x = self.a[(self.j + index) % self.limit()].take();
      if index < self.n / 2 {
        for k in (1..=index).rev() {
          self.a[(self.j + k) % self.limit()] =
            self.a[(self.j + k - 1) % self.limit()].take();
        }
        self.j = (self.j + 1) % self.limit();
      } else {
        for k in index..self.n - 1 {
          self.a[(self.j + k) % self.limit()] =
            self.a[(self.j + k + 1) % self.limit()].take();
        }
      }
      self.n -= 1;
      x
  }
  }
}
