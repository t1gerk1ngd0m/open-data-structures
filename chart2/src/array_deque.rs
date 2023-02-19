pub trait Deque<T> {
  fn get(&self, index: usize) -> T;
  fn set(&mut self, index: usize, item: T) -> T;
  fn size(&self) -> usize;
  fn add(&mut self, index: usize, item: T);
  fn remove(&mut self, index: usize) -> T;
}

#[derive(Default)]
pub struct ArrayDeque<T> {
  pub a: Box<[T]>,
  pub j: usize,
  pub n: usize
}

impl<T: Copy + Default > ArrayDeque<T> {
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
}

impl<T: Copy + Default > Deque<T> for ArrayDeque<T> {
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

  fn add(&mut self, index: usize, item: T) {
    if self.n+1 > self.limit() { self.resize() }
    if index < self.n/2 {
      self.j = if self.j == 0 { self.limit() - 1 } else { self.j - 1 };
      for k in 0..index {
        self.a[(self.j+k) % self.limit()] = self.a[(self.j+k+1) % self.limit()]
      }
    } else {
      for k in (index..=self.size()).rev() {
        self.a[(self.j+k) % self.limit()] = self.a[(self.j+k-1) % self.limit()]
      }
    }
    self.a[(self.j+index) % self.limit()] = item;
    self.n += 1
  }

  fn remove(&mut self, index: usize) -> T {
    let y = self.a[(self.j+index) % self.limit()];
    if index < self.n/2 {
      for k in (0..=index).rev() {
        self.a[(self.j+k) % self.limit()] = self.a[(self.j+k-1) % self.limit()];
      }
      self.j = (self.j+1) % self.limit();
    } else {
      for k in index..self.size() {
        self.a[(self.j+k) % self.limit()] = self.a[(self.j+k+1) % self.limit()]
      }
    }

    self.n -= 1;
    if 3 * self.size() < self.limit() { self.resize() }
    y
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_size() {
    let que = ArrayDeque {
      a: Box::new(["a","b","c","d","e"]),
      n: 5,
      j: 0
    };
    assert_eq!(que.size(), 5)
  }

  #[test]
  fn test_add_in_half_left1() {
    let mut que = ArrayDeque {
      a: Box::new(["","a","b","c","d","e","","",""]),
      n: 5,
      j: 1
    };
    que.add(1, "f");
    assert_eq!(*que.a, ["a","f","b","c","d","e","","",""]);
    assert_eq!(que.n, 6);
    assert_eq!(que.j, 0);
  }

  #[test]
  fn test_add_in_half_left2() {
    let mut que = ArrayDeque {
      a: Box::new(["a","b","c","d","e","","","",""]),
      n: 5,
      j: 0
    };
    que.add(1, "f");
    assert_eq!(*que.a, ["f","b","c","d","e","","","","a"]);
    assert_eq!(que.n, 6);
    assert_eq!(que.j, 8);
  }

  #[test]
  fn test_add_in_half_right1() {
    let mut que = ArrayDeque {
      a: Box::new(["","a","b","c","d","e","","",""]),
      n: 5,
      j: 1
    };
    que.add(2, "f");
    assert_eq!(*que.a, ["","a","b","f","c","d","e","",""]);
    assert_eq!(que.n, 6);
    assert_eq!(que.j, 1);
  }

  #[test]
  fn test_add_in_half_right2() {
    let mut que = ArrayDeque {
      a: Box::new(["","","","","a","b","c","d","e"]),
      n: 5,
      j: 4
    };
    que.add(4, "f");
    assert_eq!(*que.a, ["e","","","","a","b","c","d","f"]);
    assert_eq!(que.n, 6);
    assert_eq!(que.j, 4);
  }

  #[test]
  fn test_add_in_half_left_with_resize() {
    let mut que = ArrayDeque {
      a: Box::new(["a","b","c","d","e"]),
      n: 5,
      j: 0
    };
    que.add(1, "f");
    assert_eq!(*que.a, ["f","b","c","d","e","","","","","a"]);
    assert_eq!(que.n, 6);
    assert_eq!(que.j, 9);
  }

  #[test]
  fn test_add_in_half_right_with_resize() {
    let mut que = ArrayDeque {
      a: Box::new(["a","b","c","d","e"]),
      n: 5,
      j: 0
    };
    que.add(3, "f");
    assert_eq!(*que.a, ["a","b","c","f","d","e","","","",""]);
    assert_eq!(que.n, 6);
    assert_eq!(que.j, 0);
  }

  #[test]
  fn test_remove_in_half_left1() {
    let mut que = ArrayDeque {
      a: Box::new(["","a","b","c","d","e","","",""]),
      n: 5,
      j: 1
    };
    que.remove(1);
    assert_eq!(*que.a, ["","","a","c","d","e","","",""]);
    assert_eq!(que.n, 4);
    assert_eq!(que.j, 2);
  }

  #[test]
  fn test_remove_in_half_left2() {
    let mut que = ArrayDeque {
      a: Box::new(["b","c","d","e","","","","","a"]),
      n: 5,
      j: 8
    };
    que.remove(1);
    assert_eq!(*que.a, ["a","c","d","e","","","","",""]);
    assert_eq!(que.n, 4);
    assert_eq!(que.j, 0);
  }

  #[test]
  fn test_remove_in_half_right1() {
    let mut que = ArrayDeque {
      a: Box::new(["","a","b","c","d","e","","",""]),
      n: 5,
      j: 1
    };
    que.remove(2);
    assert_eq!(*que.a, ["","a","b","d","e","","","",""]);
    assert_eq!(que.n, 4);
    assert_eq!(que.j, 1);
  }

  #[test]
  fn test_remove_in_half_right2() {
    let mut que = ArrayDeque {
      a: Box::new(["e","","","","","a","b","c","d"]),
      n: 5,
      j: 5
    };
    que.remove(2);
    assert_eq!(*que.a, ["","","","","","a","b","d","e"]);
    assert_eq!(que.n, 4);
    assert_eq!(que.j, 5);
  }
}
