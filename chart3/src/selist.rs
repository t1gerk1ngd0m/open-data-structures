use chart1::interface::List;
use chart2::bounded_deque::BoundedDeque as BDeque;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type Wink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Clone, Debug, Default)]
pub struct SEList<T: Clone + Default> {
  head: Link<T>,
  tail: Wink<T>,
  n: usize,
  b: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Node<T> {
  block: BDeque<T>,
  next: Link<T>,
  prev: Wink<T>,
}

#[derive(Clone, Debug, Default)]
pub struct Location<T> {
  u: Link<T>,
  j: usize
}

impl<T> Node<T> {
  fn new(b: usize) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      block: BDeque::new(b + 1),
      next: None,
      prev: None,
    }))
  }
}

impl<T: Default + Clone> SEList<T> {
  pub fn new(b: usize) -> Self {
    let dummy1: Rc<RefCell<Node<T>>> = Default::default();
    let dummy2: Rc<RefCell<Node<T>>> = Default::default();
    dummy1.borrow_mut().next = Some(dummy2.clone());
    dummy2.borrow_mut().prev = Some(Rc::downgrade(&dummy1));
    Self {
      head: Some(dummy1),
      tail: Some(Rc::downgrade(&dummy2)),
      n: 0,
      b,
    }
  }

  fn get_location(&self, mut i: usize) -> Location<T> {
    let mut p: Link<T>;
    if i < self.n / 2 {
      p = self.head.as_ref().and_then(|d| d.borrow().next.clone());
      while i >= p.as_ref().map(|p| p.borrow().block.size()).unwrap() {
        i -= p.as_ref().map(|p| p.borrow().block.size()).unwrap();
        p = p.as_ref().and_then(|p| p.borrow().next.clone())
      }
      Location { u: p, j: i }
    } else {
      let mut idx = self.n;
      p = self.tail.as_ref().and_then(|d| d.upgrade());
      while i < idx {
        p = p.as_ref().and_then(|p| p.borrow().prev.as_ref().and_then(|p| p.upgrade()));
        idx -= p.as_ref().map(|p| p.borrow().block.size()).unwrap();
      }
      Location { u: p, j: i - idx }
    }
  }

  // テストかきたい
  fn add_before(&mut self, w: Link<T>) -> Link<T> {
    let u = Node::new(self.b);
    u.borrow_mut().prev = w.as_ref().and_then(|p| p.borrow().prev.clone());
    if let Some(p) = w.as_ref() {
      p.borrow_mut().prev = Some(Rc::downgrade(&u));
    }
    u.borrow_mut().next = w;
    u.borrow().prev.as_ref()
      .and_then(|p|
        p.upgrade().map(|p|
          p.borrow_mut().next = Some(Rc::clone(&u))
        )
      );
    Some(u)
  }

  // テストかきたい
  fn add_last(&mut self, x: T) {
    let mut last =
      self.tail.as_ref()
        .and_then(|p| p.upgrade())
        .and_then(|p| p.borrow().prev.as_ref().and_then(|p| p.upgrade()));
      if let Some(ref p) = last {
        if p.borrow().prev.is_none() || p.borrow().block.size() == self.b + 1 {
          last = self.add_before(self.tail.as_ref().and_then(|p| p.upgrade()));
        }
        if let Some(p) = last {
          let s = p.borrow().block.size();
          p.borrow_mut().block.add(s, x);
          self.n += 1;
        }
      }
  }

  fn spread(&mut self, u: Link<T>) {
    let mut w = u.clone();
    for _j in 0..self.b {
      w = w.as_ref().and_then(|p| p.borrow().next.clone());
    }
    while !Rc::ptr_eq(w.as_ref().unwrap(), u.as_ref().unwrap()) {
      while w.as_ref().map(|p| p.borrow().block.size()).unwrap() < self.b {
          if let Some(p) = w.as_ref() {
              let l = p.borrow().prev.as_ref().and_then(|p| p.upgrade());
              let s = l.as_ref().map(|p| p.borrow().block.size()).unwrap();
              let x = l.and_then(|p| p.borrow_mut().block.remove(s - 1)).unwrap();
              p.borrow_mut().block.add(0, x);
          }
      }
      w = w.and_then(|p| p.borrow().prev.as_ref().and_then(|p| p.upgrade()));
  }
  }
}

impl<T: Default + Clone> List<T> for SEList<T> {
  fn size(&self) -> usize {
    self.n
  }

  fn get(&self, i: usize) -> Option<T> {
    if self.size() == 0 || i > self.size() {
      None
    } else {
      let location = self.get_location(i);
      location.u.and_then(|d| d.borrow().block.get(location.j))
    }
  }

  fn set(&mut self, i: usize, x: T) -> Option<T> {
    if self.size() > 0 && i < self.size() {
      let location = self.get_location(i);
      location.u.and_then(|d| d.borrow_mut().block.set(location.j, x))
    } else {
      None
    }
  }

  fn add(&mut self, i: usize, x: T) {
    if i == self.size() {
      self.add_last(x);
      return;
    }
    let mut location = self.get_location(i);
    let v = location.u.clone();
    let mut r = 0;
    while r < self.b
        && location.u.as_ref()
            .filter(|p| p.borrow().next.is_some() && p.borrow().prev.is_some())
            .is_some()
        && location.u.as_ref().map(|p| p.borrow().block.size()).unwrap() == self.b + 1
    {
      location.u = location.u.and_then(|p| p.borrow().next.clone());
      r += 1;
    }
    // b+1個要素を含むブロックがb個あった場合
    if r == self.b {
      self.spread(v.clone());
      location.u = v.clone()
    }
    // 末尾まで到達したら新規Nodeを追加する
    if location.u.as_ref()
        .map(|p| p.borrow().next.is_none())
        .filter(|b| b == &true)
        .is_some()
    {
      location.u = self.add_before(location.u);
    }
    //逆方向に要素をシフト
    while !Rc::ptr_eq(location.u.as_ref().unwrap(), v.as_ref().unwrap()) {
      if let Some(p) = location.u.as_ref() {
        let prev = p.borrow().prev.as_ref().and_then(|p| p.upgrade());
        let prev_size = prev.as_ref().map(|p| p.borrow().block.size()).unwrap();
        let x = prev.and_then(|p| p.borrow_mut().block.remove(prev_size-1)).unwrap();
        p.borrow_mut().block.add(0,x);
      }
      location.u = location.u
                    .and_then(|p| p.borrow().prev.clone())
                    .and_then(|p| p.upgrade());
    }
    if let Some(p) = location.u {
      p.borrow_mut().block.add(location.j,x);
    }
    self.n += 1;
  }

  fn remove(&mut self, i: usize) -> Option<T> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_location() {
    let mut selist: SEList<char> = SEList::new(3);

  }
}
