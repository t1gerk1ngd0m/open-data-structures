use std::{rc::Rc, cell::RefCell};

use chart1::interface::{Que, Stack};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug, Default)]
pub struct SLList<T> {
  head: Link<T>,
  tail: Link<T>,
  n: usize
}

#[derive(Clone, Debug, Default)]
struct Node<T> {
  x: T,
  next: Link<T>
}

impl<T> Node<T> {
  fn new(x: T) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self { x, next: None }))
  }
}

impl<T> SLList<T> {
  fn new() -> Self {
    Self {
      head: None,
      tail: None,
      n: 0
    }
  }
}

impl<T> Stack<T> for SLList<T> {
  fn push(&mut self, x: T) {
    let new_node = Node::new(x);
    match self.head.take() {
      Some(old) => new_node.borrow_mut().next = Some(old),
      None => self.tail = Some(new_node.clone())
    }
    self.n += 1;
    self.head = Some(new_node);
  }

  fn pop(&mut self) -> Option<T> {
    self.head.take().map(|old| {
      if let Some(next) = old.borrow_mut().next.take() {
        self.head = Some(next);
      } else {
        self.tail.take();
      }
      self.n -= 1;
      Rc::try_unwrap(old).ok().unwrap().into_inner().x
    })
  }
}

impl<T> Que<T> for SLList<T> {
  fn add(&mut self, x: T) {
    let new_node = Node::new(x);
    match self.tail.take() {
      Some(old) => old.borrow_mut().next = Some(new_node.clone()),
      None => self.head = Some(new_node.clone())
    }
    self.n += 1;
    self.tail = Some(new_node);
  }

  fn remove(&mut self) -> Option<T> {
    self.pop()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_push() {
    let mut sllist1: SLList<char> = SLList::new();
    sllist1.push('a');

    let head = sllist1.head.unwrap().borrow().x;
    let tail = sllist1.tail.unwrap().borrow().x;
    assert_eq!(head, 'a');
    assert_eq!(tail, 'a');
    assert_eq!(sllist1.n, 1);

    let mut sllist2: SLList<char> = SLList::new();
    sllist2.push('a');
    sllist2.push('b');
    sllist2.push('c');

    let head = sllist2.head.unwrap();
    let tail = sllist2.tail.unwrap();
    assert_eq!((*head).borrow().x, 'c');
    assert_eq!((*tail).borrow().x, 'a');
    assert_eq!(sllist2.n, 3);
  }

  #[test]
  fn test_pop() {
    let mut sllist1: SLList<char> = SLList::new();
    sllist1.push('a');
    sllist1.pop();

    // assert_eq!(sllist1.head, None);
    // assert_eq!(sllist1.tail, None);
    assert_eq!(sllist1.n, 0);

    let mut sllist2: SLList<char> = SLList::new();
    sllist2.push('a');
    sllist2.push('b');
    sllist2.push('c');
    sllist2.push('d');
    sllist2.push('e');
    sllist2.pop();

    let head = sllist2.head.unwrap();
    let tail = sllist2.tail.unwrap();
    assert_eq!((*head).borrow().x, 'd');
    assert_eq!((*tail).borrow().x, 'a');
    assert_eq!(sllist2.n, 4);
  }

  #[test]
  fn test_add() {
    let mut sllist1: SLList<char> = SLList::new();
    sllist1.add('a');

    let head = sllist1.head.unwrap().borrow().x;
    let tail = sllist1.tail.unwrap().borrow().x;
    assert_eq!(head, 'a');
    assert_eq!(tail, 'a');
    assert_eq!(sllist1.n, 1);

    let mut sllist2: SLList<char> = SLList::new();
    sllist2.add('a');
    sllist2.add('b');
    sllist2.add('c');

    let head = sllist2.head.unwrap().borrow().x;
    let tail = sllist2.tail.unwrap().borrow().x;
    assert_eq!(head, 'a');
    assert_eq!(tail, 'c');
    assert_eq!(sllist2.n, 3);
  }
}
