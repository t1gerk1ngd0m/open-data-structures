use std::{rc::Rc, cell::RefCell, borrow::BorrowMut};

use chart1::interface::SSet;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SkiplistSSet<T: PartialOrd + Clone + Default> {
  head: Link<T>,
  height: usize,
  n: usize
}

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Node<T: PartialOrd> {
  x: T,
  next: Vec<Link<T>>
}

impl<T: PartialOrd> Node<T> {
  fn new(x: T, h: usize) -> Rc<RefCell<Node<T>>> {
    Rc::new(RefCell::new(Node {
      x,
      next: vec![None; h+1],
    }))
  }
}

impl<T: PartialOrd + Default + Clone> SkiplistSSet<T> {
  pub fn new() -> Self {
    let sentinel = Node::new(Default::default(), 32);
    Self {
      head: Some(sentinel),
      height: 0,
      n: 0
    }
  }

  fn find_pred_node(&self, x: &T) -> Link<T> {
    match self.head {
      Some(ref sentinel) => {
        let mut n = Rc::clone(sentinel); // sentinelから開始
        for r in (0..=self.height).rev() {
          loop {
            let u = Rc::clone(&n);
            match u.borrow().next[r] {
              // u.nextが引数xよりも小さい場合は右にリストrの中で右に進む
              Some(ref target_x) if u.borrow().x < *x => n = Rc::clone(target_x),
              // u.nextが引数x以上の場合はリストr-1に下がる
              _ => break
            };
          }
        }
        Some(n)
      },
      None => None
    }
  }

  fn pick_height() -> usize {
    let z = rand::random::<usize>();
    let mut k = 0;
    let mut m = 1;
    while (z & m) != 0 {
      k += 1;
      m <<= 1;
    }
    k
  }
}

impl<T: PartialOrd + Default + Clone> SSet<T> for SkiplistSSet<T> {
  fn size(&self) -> usize {
    self.n
  }

  fn add(&mut self, x: T) -> bool {
    match self.head {
      Some(ref sentinel) => {
        let mut stack: Vec<Link<T>> = vec![None; sentinel.borrow().next.len()];
        let mut n = Rc::clone(sentinel);
        for r in (0..=self.height).rev() {
          loop {
            let u = Rc::clone(&n);
            match u.borrow().next[r] {
              // この場合nは右に移動する
              Some(ref u) if u.borrow().x < x => {
                n = Rc::clone(u)
              },
              Some(ref u) if u.borrow().x == x => {
                return false
              },
              // この場合nは下に降りる(位置は不変)
              _ => break
            };
          }
          stack[r] = Some(Rc::clone(&n));
        }
        let new_node = Node::new(x, Self::pick_height());
        let height = new_node.borrow().next.len() - 1;
        // 新しく追加するNodeのheightがsentinelのheightよりも高い場合
        while self.height < height {
          if let Some(sentinel) =
            self.head
                .as_ref()
                .filter(|sentinel|
                  sentinel.borrow().next.len() < new_node.borrow().next.len()
                )
          {
            let mut sentinel_ref_mut = RefCell::borrow_mut(&n);
            sentinel_ref_mut.next.push(None);
          }
          self.height += 1; // 高さを増やす
          if let Some(e) = stack.get_mut(self.height) {
            e.replace(Rc::clone(&sentinel));
          } else {
            stack.push(Some(Rc::clone(sentinel)));
          }
        }
        for (index, item) in stack.iter_mut().enumerate().take(height+1) {
          match item.take() {
            Some(ref u) => {
              let mut u_ref_mut = RefCell::borrow_mut(u);
              let mut new_node_ref_mut = RefCell::borrow_mut(&new_node);
              // 新しくNodeを入れたことでnextの付け替えをする
              new_node_ref_mut.next[index] = u_ref_mut.next[index].take();
              u_ref_mut.next[index] = Some(Rc::clone(&new_node));
            },
            None => break
          }
        }
        self.n += 1;
        true
      },
      None => false
    }
  }

  fn remove(&mut self, x: &T) -> Option<T> {
    match self.head {
      Some(ref sentinel) => {
        let mut n = Rc::clone(sentinel);
        let mut del = None;
        let height = self.height;
        for r in (0..=height).rev() {
          let removed = loop {
            let u = Rc::clone(&n);
            match u.borrow().next[r] {
              Some(ref u) if u.borrow().x < *x => {
                n = Rc::clone(u)
              },
              Some(ref u) if u.borrow().x == *x => {
                break true
              },
              _ => break false
            };
          };
          if removed {
            let mut n_ref_mut = RefCell::borrow_mut(&n);;
            del = n_ref_mut.next[r].take();
            if let Some(del) = del.as_ref() {
              let mut del_ref_mut = RefCell::borrow_mut(&del);
              if let Some(next) = del_ref_mut.next[r].take() {
                n_ref_mut.next[r] = Some(next);
              } else if Rc::ptr_eq(&n, self.head.as_ref().unwrap()) && self.height > 0 {
                self.height -= 1;
              }
            }
          }
        }
        del.map(|del| {
          self.n -= 1;
          Rc::try_unwrap(del).ok().unwrap().into_inner().x
        })
      },
      None => None
    }
  }

  fn find(&self, x: &T) -> Option<T> {
    let pred_node = self.find_pred_node(x);
    match pred_node {
      Some(ref u) if u.borrow().next[0].is_some() => {
        u.borrow()
          .next[0]
          .as_ref()
          .map(|next| next.borrow().x.clone())
      },
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
  }
}
