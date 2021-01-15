use std::cell::RefCell;
use std::rc::{Rc, Weak}; // rc used as a reference counter, weak can be dropped while references still exist

#[derive(Debug)]
pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>,
}
#[derive(Debug)]
pub struct DbList<T> {
    // a refcell is dropped when the last reference to the item is dropped
    // It appears immutable to the outside, but can be mutable interior, when borrowed once
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> DbList<T> {
    pub fn new() -> Self {
        DbList {
            last: None,
            first: None,
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                // create new back object
                let new_back = Rc::new(RefCell::new(DbNode {
                    data,
                    prev: Some(r.clone()),
                    next: None,
                }));
                // tell the last object this is now in back
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                self.last = Some(Rc::downgrade(&new_back));
                m.next = Some(new_back);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    prev: None,
                    next: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(r) => {
                // create new front object
                let new_front = Rc::new(RefCell::new(DbNode {
                    data,
                    next: Some(r.clone()),
                    prev: None,
                }));
                // tell the first object this is now in front
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                // put this on the front
                self.first = Some(new_front);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
    // TODO impl pop_front, and pop_back
}
fn main() {
    let mut d1 = DbList::new();
    d1.push_front(6);
    d1.push_back(10);
    println!("{:?}", d1);
}
