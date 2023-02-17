use chart1::interface::List;

#[derive(Default)]
pub struct ArrayStack<T> {
  a: Box<[T]>,
  n: usize
}

impl<T: Copy + Default> ArrayStack<T> {
  fn resize(&mut self) {
    let new_limit = self.n * 2;
    let mut new_array = vec![Default::default(); new_limit].into_boxed_slice();
    for i in 0..self.n {
      new_array[i] = self.a[i];
    }
    self.a = new_array
  }

  fn limit(&self) -> usize {
    self.a.len()
  }
}

impl<T: Copy + Default> List<T> for ArrayStack<T> {
  fn size(&self) -> usize {
    self.n
  }

  fn get(&self, index: usize) -> T {
    self.a[index]
  }

  fn set(&mut self, index: usize, item: T) -> T {
    let y = self.a[index];
    self.a[index] = item;
    y
  }

  fn add(&mut self, index: usize, item: T) {
    if self.size() == self.limit() {
      self.resize();
    }
    for j in (index+1..self.size()+1).rev() {
      self.a[j] = self.a[j-1];
    }
    self.n += 1;
    self.a[index] = item;
  }

  fn remove(&mut self, index: usize) -> T {
    let y = self.a[index];
    self.n -= 1;

    for j in index..self.size() {
      self.a[j] = self.a[j+1];
    }

    if self.size() * 3 < self.limit() {
      self.resize();
    } else {
      self.a[self.size()] = Default::default();
    }

    y
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_resize_up() {
    let mut stack = ArrayStack {
      a: Box::new([0,1,2,3,4]),
      n: 5
    };
    stack.resize();
    assert_eq!(stack.a.len(), 10);
  }

  #[test]
  fn test_resize_down() {
    let mut stack = ArrayStack {
      a: Box::new([0,1,2,3,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]),
      n: 5
    };
    stack.resize();
    assert_eq!(stack.a.len(), 10);
  }

  #[test]
  fn test_size() {
    let stack = ArrayStack {
      a: Box::new([0,1,2,3,4]),
      n: 5
    };
    assert_eq!(stack.size(), 5)
  }

  #[test]
  fn test_get() {
    let stack = ArrayStack {
      a: Box::new([0,1,2,3,4]),
      n: 5
    };
    assert_eq!(stack.get(1), 1)
  }

  #[test]
  fn test_set() {
    let mut stack = ArrayStack {
      a: Box::new([0,1,2,3,4]),
      n: 5
    };
    assert_eq!(stack.set(1, 10), 1);
    assert_eq!(*stack.a, [0,10,2,3,4]);
  }

  #[test]
  fn test_add() {
    let mut stack = ArrayStack {
      a: Box::new([0,1,2,3,4]),
      n: 5
    };
    stack.add(1, 10);
    assert_eq!(*stack.a, [0,10,1,2,3,4,0,0,0,0]);
    assert_eq!(stack.n, 6);
  }

  #[test]
  fn test_remove() {
    let mut stack = ArrayStack {
      a: Box::new([0,1,2,3,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]),
      n: 5
    };
    assert_eq!(stack.remove(1), 1);
    assert_eq!(*stack.a, [0,2,3,4,0,0,0,0]);
    assert_eq!(stack.n, 4);
  }

  #[test]
  fn test_remove2() {
    let mut stack = ArrayStack {
      a: Box::new([0,1,2,3,4]),
      n: 5
    };
    assert_eq!(stack.remove(1), 1);
    assert_eq!(*stack.a, [0,2,3,4,0]);
    assert_eq!(stack.n, 4);
  }
}
