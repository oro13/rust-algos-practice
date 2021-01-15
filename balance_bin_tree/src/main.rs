use std::fmt::Debug;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    h: i8, // the number of nodes 'below' this node
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinData<T> {
    pub fn rot_right(mut self) -> Box<Self> {
        // result is the right node
        let mut res = match self.left.0.take() {
            Some(res) => res,
            None => return Box::new(self), //no right node, no way to rotate
        };
        // move left of right node to right of start node
        self.left = BinTree(res.right.0.take());
        self.left.set_height();
        // set the results left node to the start node
        res.right = BinTree(Some(Box::new(self)));
        res.right.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());

        res
    }
    pub fn rot_left(mut self) -> Box<Self> {
        // result is the right node
        let mut res = match self.right.0.take() {
            Some(res) => res,
            None => return Box::new(self), //no right node, no way to rotate
        };
        // move left of right node to right of start node
        self.right = BinTree(res.left.0.take());
        self.right.set_height();
        // set the results left node to the start node
        res.left = BinTree(Some(Box::new(self)));
        res.left.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());

        res
    }
}
impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }

    pub fn height(&self) -> i8 {
        match self.0 {
            Some(ref t) => t.h,
            None => 0,
        }
    }
    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.h = 1 + std::cmp::max(t.left.height(), t.right.height());
        }
    }

    pub fn rot_right(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_right());
    }

    pub fn rot_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_left());
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        let rot_dir = match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                    if bd.left.height() - bd.right.height() > 1 {
                        1
                    } else {
                        0
                    }
                } else {
                    bd.right.add_sorted(data);
                    if bd.right.height() - bd.left.height() > 1 {
                        -1
                    } else {
                        0
                    }
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    h: 0,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }));
                0
            }
        };
        match rot_dir {
            1 => self.rot_right(),
            -1 => self.rot_left(),
            _ => self.set_height(),
        }
        self.set_height();
    }
}

impl<T: Debug> BinTree<T> {
    pub fn print_l_first(&self, depth: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.print_l_first(depth + 1);
            let mut spc = String::new();
            for _ in 0..depth {
                spc.push('.');
            }
            println!("{}:{}{:?}", bd.h, spc, bd.data);
            bd.right.print_l_first(depth + 1);
        }
    }
}

fn main() {
    let mut bt = BinTree::new();
    bt.add_sorted(5);
    bt.add_sorted(1);
    bt.add_sorted(8);
    bt.add_sorted(10);
    bt.add_sorted(-1);
    bt.add_sorted(111711);
    bt.print_l_first(0);
}
