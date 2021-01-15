use std::fmt::Debug;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                } else {
                    bd.right.add_sorted(data);
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }))
            }
        }
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
            println!("{}{:?}", spc, bd.data);
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
    bt.print_l_first(0);
}
