use std::iter::{zip, Map};

fn main() {
    //1
    let mut r = vec![];
    for i in 1..21 {
        r.push(i);
    }

    //2
    let mut s = vec![];
    for i in (1..21).rev() {
        s.push(i);
    }

    //3
    let mut t = vec![];
 
    //4
    for (a, b) in zip(r.iter(), s.iter()) {
        t.push(a + b);
    }

    //5


}
