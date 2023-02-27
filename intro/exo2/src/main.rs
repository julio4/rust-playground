use std::iter::zip;

fn main() {
    //1
    let r: Vec<i32> = (1..21).collect();

    //2
    let s: Vec<i32> = (1..21).rev().collect();

    //3
    let mut t = Vec::new();

    //4
    for (a, b) in zip(r.iter(), s.iter()) {
        t.push(a + b);
    }

    //5
    let t: Vec<i32> = r.iter().zip(s.iter())
        .map(|(a, b)| a + b)
        .collect();

    //6
    assert!(t.iter().all(|x| *x == 21));

    //7
    println!("Sum: {:?}", r.iter().sum::<i32>());

    //8
    let _u: Vec<i32> = (1..11).map(|i| i * i).collect();
}
