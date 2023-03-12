use chart1::interface::Que;
use rand::Rng;

struct RandomQueue<T> {
  a: Box<[T]>,
  j: usize,
  n: usize
}

impl<T: Copy + Default> RandomQueue<T> {
  fn limit(&self) -> usize {
    self.a.len()
  }

  fn resize(&mut self) {
    let new_limit = self.n * 2;
    let mut new_array = vec![Default::default(); new_limit].into_boxed_slice();
    for i in 0..self.n {
      new_array[i] = self.a[(self.j + i) % self.limit()]
    }
    self.a = new_array;
    self.j = 0
  }

  fn random_index(&mut self) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..self.limit()) + self.j
  }

  fn size(&self) -> usize {
    self.n
  }

  fn get(&self, index: usize) -> T {
    self.a[(self.j + index) % self.limit()]
  }

  fn set(&mut self, index: usize, item: T) -> T {
    let y = self.a[(self.j + index) % self.limit()];
    self.a[(self.j + index) % self.limit()] = item;
    y
  }
}

impl<T: Copy + Default> Que<T> for RandomQueue<T> {
  fn add(&mut self, item: T) {
    if self.size() == self.limit() {
      self.resize()
    }
    self.a[(self.j + self.n) % self.limit()] = item;
    self.n += 1
  }

  fn remove(&mut self) -> Option<T> {
    let target_index = self.random_index();
    let y = self.a[target_index];
    self.a[target_index] = self.a[self.j];
    self.j = (self.j + 1) % self.limit();
    self.n -= 1;

    if self.size() * 3 < self.limit() {
      self.resize()
    }

    Some(y)
  }
}
