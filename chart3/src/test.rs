#[derive(Debug)]
struct Node<T> {
  next: Box<Option<T>>,
  n: usize
}

fn main() {
  let node_1: Node<()> = Node { next: Box::new(None), n: 1 };
  let node_2 = Node { next: Box::new(Some(node_1)), n: 1 };

  println!("{:?}", node_2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_main() {
    main()
  }
}