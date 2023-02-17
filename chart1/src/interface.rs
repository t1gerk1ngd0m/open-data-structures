pub trait Queue<T> {
    fn add(&mut self, x: T);
    fn remove(&mut self) -> T;
}

pub trait Stack<T> {
    fn push(&mut self, x: T);
    fn pop(&mut self) -> T;
}

pub trait List<T: Clone> {
    fn size(&self) -> usize;
    fn get(&self, i: usize) -> T;
    fn set(&mut self, i: usize, x: T) -> T;
    fn add(&mut self, i: usize, x: T);
    fn remove(&mut self, i: usize) -> T;
}

pub trait USet<T: PartialEq + Clone> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: &T) -> T;
    fn find(&self, x: &T) -> T;
}

pub trait SSet<T: PartialOrd + Clone> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: &T) -> T;
    fn find(&self, x: &T) -> T;
}

pub trait Graph {
    fn add_edge(&mut self, i: usize, j: usize);
    fn remove_edge(&mut self, i: usize, j: usize);
    fn has_edge(&self, i: usize, j: usize) -> bool;
    fn out_edges(&self, i: usize) -> Vec<usize>;
    fn in_edges(&self, i: usize) -> Vec<usize>;
}
