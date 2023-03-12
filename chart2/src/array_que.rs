use chart1::interface::Que;

#[derive(Default)]
pub struct ArrayQue<T> {
  pub a: Box<[T]>,
  pub j: usize,
  pub n: usize
}

impl<T: Copy + Default> ArrayQue<T> {
  pub fn limit(&self) -> usize {
    self.a.len()
  }

  pub fn resize(&mut self) {
    let new_limit = self.n * 2;
    let mut new_array = vec![Default::default(); new_limit].into_boxed_slice();
    for i in 0..self.n {
      new_array[i] = self.a[(self.j + i) % self.limit()]
    }
    self.a = new_array;
    self.j = 0
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

impl<T: Copy + Default> Que<T> for ArrayQue<T> {
  fn add(&mut self, item: T) {
    if self.size() == self.limit() {
      self.resize()
    }
    self.a[(self.j + self.n) % self.limit()] = item;
    self.n += 1
  }

  fn remove(&mut self) -> Option<T> {
    let y = self.a[self.j];
    self.a[self.j] = Default::default();
    self.j = (self.j + 1) % self.limit();
    self.n -= 1;

    if self.size() * 3 < self.limit() {
      self.resize()
    }

    Some(y)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_size() {
    let que = ArrayQue {
      a: Box::new(["a","b","c","d","e"]),
      n: 5,
      j: 0
    };
    assert_eq!(que.size(), 5)
  }

  #[test]
  fn test_add_without_resize() {
    let mut que = ArrayQue {
      a: Box::new(["","","","","a","b","c","d","e"]),
      n: 5,
      j: 4
    };
    que.add("f");
    assert_eq!(*que.a, ["f","","","","a","b","c","d","e"]);
    assert_eq!(que.n, 6);
    assert_eq!(que.j, 4);
  }

  #[test]
  fn test_add_with_resize() {
    let mut que = ArrayQue {
      a: Box::new(["f","g","h","i","a","b","c","d","e"]),
      n: 9,
      j: 4
    };
    que.add("j");
    assert_eq!(*que.a, ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "", "", "", "", "", "", "", ""]);
    assert_eq!(que.n, 10);
    assert_eq!(que.j, 0);
  }

  #[test]
  fn test_remove_without_resize() {
    let mut que = ArrayQue {
      a: Box::new(["","","","","a","b","c","d","e"]),
      n: 5,
      j: 4
    };
    assert_eq!(que.remove().unwrap(), "a");
    assert_eq!(*que.a, ["","","","","","b","c","d","e"]);
    assert_eq!(que.n, 4);
    assert_eq!(que.j, 5);
  }

  #[test]
  fn test_remove_with_resize() {
    let mut que = ArrayQue {
      a: Box::new(["","","","","a","b","c","d","e","","","","","","","",""]),
      n: 5,
      j: 4
    };
    que.remove();
    assert_eq!(*que.a, ["b","c","d","e","","","",""]);
    assert_eq!(que.n, 4);
    assert_eq!(que.j, 0);
  }

  #[test]
  fn test_remove_with_resize2() {
    let mut que = ArrayQue {
      a: Box::new(["b","c","d","e","","","","","","","","","","","","","a"]),
      n: 5,
      j: 16
    };
    que.remove();
    assert_eq!(*que.a, ["b","c","d","e","","","",""]);
    assert_eq!(que.n, 4);
    assert_eq!(que.j, 0);
  }
}
