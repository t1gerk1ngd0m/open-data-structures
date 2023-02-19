// 1つずつaddする方法だと計算量は Collection cの要素数 * n-i となる
// 下記の実装であれば計算量は Collection cの要素数 + i となる

use crate::array_stack::ArrayStack;

trait AddAll<T: Copy + Default> {
  fn add_all(&mut self, i: usize, x:Box<[T]>);
}

impl<T: Copy + Default> AddAll<T> for ArrayStack<T> {
  fn add_all(&mut self, i: usize, x: Box<[T]>) {
    let mut new_array = vec![Default::default(); self.n].into_boxed_slice();
    for j in 0..i {
      new_array[j] = self.a[j];
    }
    for j in i..x.len()+i {
      new_array[j] = x[j-i];
    }
    for j in i..self.n {
      new_array[j] = self.a[j];
    }
    self.a = new_array;
  }
}
