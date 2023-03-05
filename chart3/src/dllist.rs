use std::{rc::{Rc, Weak}, cell::RefCell};
use chart1::interface::List;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type Wink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug, Default)]
pub struct DLList<T: Default> {
  head: Link<T>,
  tail: Wink<T>,
  n: usize
}

#[derive(Debug, Default)]
struct Node<T> {
  x: T,
  next: Link<T>,
  prev: Wink<T>
}

impl<T> Node<T> {
  fn new(x: T) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self { x, next: None, prev: None }))
  }
}

impl<T: Default> DLList<T> {
  pub fn new() -> Self {
    let dummy1: Rc<RefCell<Node<T>>> = Default::default();
    let dummy2: Rc<RefCell<Node<T>>> = Default::default();
    dummy1.borrow_mut().next = Some(Rc::clone(&dummy2));
    dummy2.borrow_mut().prev = Some(Rc::downgrade(&dummy1));
    Self {
      head: Some(dummy1),
      tail: Some(Rc::downgrade(&dummy2)),
      n: 0
    }
  }

  fn get_node(&self, i: usize) -> Link<T> {
    let mut p: Link<T>;
    if i < self.n / 2 {
      p = self.head.as_ref().and_then(|d| d.borrow().next.clone());
      for _j in 0..i {
        // ここのテストから
        p = p.and_then(|p| p.borrow().next.clone())
      }
    } else {
      p = self.tail.as_ref().and_then(|d| d.upgrade());
      for _j in (i+1..=self.n).rev() {
        p = p.and_then(|p| p.borrow().prev.as_ref().and_then(|p| p.upgrade()))
      }
    }
    p
  }

  fn add_before(&mut self, w: Link<T>, x: T) {
    let u = Node::new(x);
    u.borrow_mut().prev = w.as_ref().and_then(|p| p.borrow().prev.clone());
    if let Some(p) = w.as_ref() {
      p.borrow_mut().prev = Some(Rc::downgrade(&u));
    }
    u.borrow_mut().next = w;
    u.borrow()
      .prev
      .as_ref()
      .and_then(|p| p.upgrade().map(|p| p.borrow_mut().next = Some(Rc::clone(&u))));
    self.n += 1;
  }
}

impl<T: Clone + Default> List<T> for DLList<T> {
  fn size(&self) -> usize {
    self.n
  }

  fn get(&self, i: usize) -> Option<T> {
    if self.size() == 0 {
      None
    } else {
      self.get_node(i).map(|p| p.borrow().x.clone())
    }
  }

  fn set(&mut self, i: usize, x: T) -> Option<T> {
    if self.size() == 0 {
      None
    } else {
      self.get_node(i).map(|p| {
        let target = p.borrow().x.clone();
        p.borrow_mut().x = x;
        target
      })
    }
  }

  fn add(&mut self, i: usize, x: T) {
    self.add_before(self.get_node(i), x)
  }

  fn remove(&mut self, i: usize) -> Option<T> {
    if self.size() == 0 {
      None
    } else {
      let target = self.get_node(i);
      let prev = target.as_ref().and_then(|p| p.borrow().prev.clone());
      let next = target.as_ref().and_then(|p| p.borrow().next.clone());
      prev.as_ref()
          .and_then(|p| p.upgrade().map(|p| p.borrow_mut().next = next.clone()));
      if let Some(p) = next {
        p.borrow_mut().prev = prev
      }
      self.n -= 1;
      match target {
        Some(w) => Some(w.borrow().x.clone()),
        None => None
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_node() {
    let mut dllist: DLList<char> = DLList::new();
    dllist.add(0, 'a');
    dllist.add(1, 'b');
    dllist.add(2, 'c');

    let actual = dllist.get_node(1).unwrap().borrow().x;
    assert_eq!(actual, 'b')
  }

  #[test]
  fn test_add_before() {
    let mut dllist: DLList<char> = DLList::new();
    dllist.add(0, 'a');
    dllist.add(1, 'b');
    dllist.add(2, 'c');
    dllist.add(3, 'd');
    assert_eq!(dllist.size(), 4);

    let target = dllist.get_node(2);
    dllist.add_before(target, 'x');

    let actual = dllist.get_node(2).unwrap().borrow().x;
    assert_eq!(actual, 'x');
    assert_eq!(dllist.size(), 5);
  }

  #[test]
  fn test_remove() {
    let mut dllist: DLList<char> = DLList::new();
    dllist.add(0, 'a');
    dllist.add(1, 'b');
    dllist.add(2, 'c');
    dllist.add(3, 'd');
    assert_eq!(dllist.size(), 4);

    assert_eq!(dllist.remove(2), Some('c'));
    assert_eq!(dllist.size(), 3);
    assert_eq!(dllist.get(2), Some('d'));
  }
}
