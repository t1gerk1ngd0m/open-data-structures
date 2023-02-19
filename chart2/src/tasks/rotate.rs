use crate::array_que::ArrayQue;
use crate::array_deque::ArrayDeque;

trait Rotate {
  fn rotate(&mut self, r: usize);
}

impl<T: Copy + Default> Rotate for ArrayQue<T> {
  fn rotate(&mut self, r: usize) {
    let mut new_array = vec![Default::default(); self.limit()].into_boxed_slice();
    for i in self.j..self.j+self.n {
      new_array[(i+r) % self.limit()] = self.a[i]
    }
    self.a = new_array;
    self.j += r;
  }
}

// 出来ると思ったけどわからん
// rがどんな値になっても要素数分移動させるんじゃないの？？？
impl<T: Copy + Default> Rotate for ArrayDeque<T> {
  fn rotate(&mut self, r: usize) {
    let mut new_array = vec![Default::default(); self.limit()].into_boxed_slice();
    if r < self.n/2 {
      for k in self.j..self.j+self.n {
        new_array[(k+r) % self.limit()] = self.a[k]
      }
    } else {
      for k in self.j..self.j+self.n {
        new_array[(k+(self.n-r)) % self.limit()] = self.a[k]
      }
    }
    self.a = new_array;
  }
}
