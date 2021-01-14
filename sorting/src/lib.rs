mod b_rand;
use std::fmt::Debug;

// O(n**2)
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for i in 1..v.len() {
        let mut sorted = true;
        for n in 1..v.len() + 1 - i {
            if v[n] < v[n - 1] {
                v.swap(n, n - 1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
}

// O(n * ln(n))
pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // sort the left half
    // sort the right half
    // bring the sorted halfs together

    if v.len() <= 1 {
        return v;
    }

    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    //bring them together
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => {
                match b_peek {
                    Some(ref b_val) => {
                        if b_val < a_val {
                            res.push(b_peek.take().unwrap()); //take removes a value from an option, unwrap removes option layer
                            b_peek = b_it.next();
                        } else {
                            res.push(a_peek.take().unwrap());
                            a_peek = a_it.next();
                        }
                    }
                    None => {
                        res.push(a_peek.take().unwrap());
                        res.extend(a_it);
                        return res;
                    }
                }
            }
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

// Move first element to the correct place
// Everything lower should be before it
// everything higher should be after it
// return its location
pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            //move our pivot forward 1, and put this element before it
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);

    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]); // Middle element already sorted
}

struct RawSend<T>(*mut [T]);

unsafe impl<T> Send for RawSend<T> {}
pub fn threaded_quick_sort<T: 'static + PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);

    let (a, b) = v.split_at_mut(p);
    let raw_a: *mut [T] = a as *mut [T];
    let raw_s = RawSend(raw_a);
    unsafe {
        let handle = std::thread::spawn(move || threaded_quick_sort(&mut *raw_s.0));

        threaded_quick_sort(&mut b[1..]);

        handle.join().ok(); // here we guarantee our threads will either merge or the program will terminate
    }
}

pub fn quick_sort_rayon<T: Send + PartialOrd>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);

    let (a, b) = v.split_at_mut(p);
    rayon::join(|| quick_sort_rayon(a), || quick_sort_rayon(&mut b[1..]));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pivot_test() {
        let mut v = vec![4, 6, 3, 1];
        let p = pivot(&mut v);

        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }
    }
    #[test]
    fn quick_sort_rayon_simple() {
        let mut v = vec![2, 3, 1, 5, 10, 4];
        quick_sort_rayon(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 10]);
    }
    #[test]
    fn quick_sort_rayon_neg() {
        let mut v = vec![2, 3, -1, 5, 10, -4];
        quick_sort_rayon(&mut v);
        assert_eq!(v, [-4, -1, 2, 3, 5, 10]);
    }
    #[test]
    fn threaded_quick_sort_simple() {
        let mut v = vec![2, 3, 1, 5, 10, 4];
        threaded_quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 10]);
    }
    #[test]
    fn threaded_quick_sort_neg() {
        let mut v = vec![2, 3, -1, 5, 10, -4];
        threaded_quick_sort(&mut v);
        assert_eq!(v, [-4, -1, 2, 3, 5, 10]);
    }
    #[test]
    fn quick_sort_simple() {
        let mut v = vec![2, 3, 1, 5, 10, 4];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 10]);
    }
    #[test]
    fn quick_sort_neg() {
        let mut v = vec![2, 3, -1, 5, 10, -4];
        quick_sort(&mut v);
        assert_eq!(v, [-4, -1, 2, 3, 5, 10]);
    }
    #[test]
    fn merge_sort_simple() {
        let v = vec![2, 3, 1, 5, 10, 4];
        let v = merge_sort(v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 10]);
    }
    #[test]
    fn merge_sort_neg() {
        let v = vec![2, 3, -1, 5, 10, -4];
        let v = merge_sort(v);
        assert_eq!(v, [-4, -1, 2, 3, 5, 10]);
    }
    #[test]
    fn bubble_sort_simple() {
        let mut v = vec![2, 3, 1, 5, 10, 4];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 10]);
    }
    #[test]
    fn bubble_sort_neg() {
        let mut v = vec![2, 3, -1, 5, 10, -4];
        bubble_sort(&mut v);
        assert_eq!(v, [-4, -1, 2, 3, 5, 10]);
    }
}
