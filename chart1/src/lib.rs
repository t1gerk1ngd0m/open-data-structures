use std::vec::Vec;

pub trait Queue<T> {
    fn add(&mut self, x: T);
    fn remove(&mut self) -> Option<T>;
}

pub trait StackTrait<T> {
    fn push(&mut self, x: T);
    fn pop(&mut self) -> Option<T>;
}

pub trait ListTrait<T> {
    fn size(&self) -> usize;
    fn get(&self, i: usize) -> Option<T>;
    fn set(&mut self, i: usize, x: T) -> Option<T>;
    fn add(&mut self, i: usize, x: T);
    fn remove(&mut self, i: usize) -> Option<T>;
}

pub trait USetTrait<T: PartialEq + Eq> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: &T) -> Option<T>;
    fn find(&self, x: &T) -> Option<T>;
}

struct USet<T> (Vec<T>);

impl<T> USetTrait<T> for USet<T> {
    fn size(&self) -> usize {
        self.len()
    }
    fn add(&mut self, x: T) -> bool {
        if self.into_iter().find(&x) {
            return false
        }
        self.push(x)
    }
    fn remove(&mut self, x: &T) -> Option<T> {
        if let Some(idx) = self.0.into_iter().position(|y| *y == x) {
            let y = self.remove(idx);
            return y
        }
        return None
    }
    fn find(&self, x: &T) -> Option<T> {
        let y = self.0.into_iter().find(&x);
        match y {
            Some(ele) => Some(ele),
            None => None
        }
    }
}

pub trait SSetTrait<T: PartialOrd> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: &T) -> Option<T>;
    fn find(&self, x: &T) -> Option<T>;
}

pub trait BagTrait<T: PartialEq> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: &T) -> Option<T>;
    fn find(&self, x: &T) -> Option<T>;
    fn find_all(&self, x: &T) -> Vec<T>;
}
