pub trait Queue<T> {
  fn size(&self) -> usize;
  fn add(&mut self, item: T);
  fn remove(&mut self) -> T;
}

#[derive(Default, std::fmt::Debug)]
struct ArrayQueue<T> {
  a: Box<[T]>,
  j: usize,
  n: usize
}

impl<T: Copy + Default + std::fmt::Debug> ArrayQueue<T> {
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
}

impl<T: Copy + Default + std::fmt::Debug> Queue<T> for ArrayQueue<T> {
  fn size(&self) -> usize {
    self.n
  }

  fn add(&mut self, item: T) {
    if self.size() == self.limit() {
      self.resize()
    }
    self.a[(self.j + self.n) % self.limit()] = item;
    self.n += 1
  }

  fn remove(&mut self) -> T {
    let y = self.a[self.j];
    self.a[self.j] = Default::default();
    self.j = (self.j + 1) % self.limit();
    self.n -= 1;

    if self.size() * 3 < self.limit() {
      self.resize()
    }

    y
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_size() {
    let queue = ArrayQueue {
      a: Box::new(["a","b","c","d","e"]),
      n: 5,
      j: 0
    };
    assert_eq!(queue.size(), 5)
  }

  #[test]
  fn test_add_without_resize() {
    let mut queue = ArrayQueue {
      a: Box::new(["","","","","a","b","c","d","e"]),
      n: 5,
      j: 4
    };
    queue.add("f");
    assert_eq!(*queue.a, ["f","","","","a","b","c","d","e"]);
    assert_eq!(queue.n, 6);
    assert_eq!(queue.j, 4);
  }

  #[test]
  fn test_add_with_resize() {
    let mut queue = ArrayQueue {
      a: Box::new(["f","g","h","i","a","b","c","d","e"]),
      n: 9,
      j: 4
    };
    queue.add("j");
    assert_eq!(*queue.a, ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "", "", "", "", "", "", "", ""]);
    assert_eq!(queue.n, 10);
    assert_eq!(queue.j, 0);
  }

  #[test]
  fn test_remove_without_resize() {
    let mut queue = ArrayQueue {
      a: Box::new(["","","","","a","b","c","d","e"]),
      n: 5,
      j: 4
    };
    assert_eq!(queue.remove(), "a");
    assert_eq!(*queue.a, ["","","","","","b","c","d","e"]);
    assert_eq!(queue.n, 4);
    assert_eq!(queue.j, 5);
  }

  #[test]
  fn test_remove_with_resize() {
    let mut queue = ArrayQueue {
      a: Box::new(["","","","","a","b","c","d","e","","","","","","","",""]),
      n: 5,
      j: 4
    };
    queue.remove();
    assert_eq!(*queue.a, ["b","c","d","e","","","",""]);
    assert_eq!(queue.n, 4);
    assert_eq!(queue.j, 0);
  }

  #[test]
  fn test_remove_with_resize2() {
    let mut queue = ArrayQueue {
      a: Box::new(["b","c","d","e","","","","","","","","","","","","","a"]),
      n: 5,
      j: 16
    };
    queue.remove();
    assert_eq!(*queue.a, ["b","c","d","e","","","",""]);
    assert_eq!(queue.n, 4);
    assert_eq!(queue.j, 0);
  }
}
