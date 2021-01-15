//#[derive(Debug)]
//pub struct LinkedList<T> {
//    data: T,
//    next: Option<Box<LinkedList<T>>>,
//}
//
//impl<T: std::ops::AddAssign> LinkedList<T> {
//    pub fn add_up(&mut self, n: T) {
//        self.data += n;
//    }
//}
//fn main() {
//    let mut ll = LinkedList {
//        data: 3,
//        next: Some(Box::new(LinkedList {
//            data: 2,
//            next: None,
//        })),
//    };
//    if let Some(ref mut v) = ll.next {
//        v.add_up(10);
//    }
//    println!("{:?}!", ll);
//
//    let mut v = Vec::with_capacity(100);
//    v.push("hello".to_string());
//    v.push("goodbye".to_string());
//
//    println!("v.len {}, capacity = {}", v.len(), v.capacity());
//}
use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd + Ord> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }
    // challenge: add an item in the correct position, if the list is assumed to be sorted
    pub fn insert_sorted(&mut self, data: T) {
        match self.0 {
            Some((ref val, ref mut child)) => match data.partial_cmp(val) {
                Some(Ordering::Less) => self.push_front(data),
                _ => child.insert_sorted(data),
            },
            None => self.push_front(data),
        }
    }
}

fn main() {
    let mut l = LinkedList::new();
    l.push_front(1);
    l.push_back(2);
    l.push_back(4);
    l.insert_sorted(19);
    l.insert_sorted(3);
    l.insert_sorted(5);
    l.insert_sorted(0);
    println!("LinkedList = {:?}", l);
}
