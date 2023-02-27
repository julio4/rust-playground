#![allow(dead_code)]

struct Power {
    next: u32
}

impl Power {
    fn new() -> Self {
        Power { next: 1 }
    }
}

impl Iterator for Power {
    type Item = u32;
    
    // 1
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;
        self.next *= 2;
        Some(current)
    }
}

fn main() {
    for x in Power::new().take(5) {
        println!("x: {}", x);
    }

    let vec_sqrt: Vec<f64> = Power::new().take(10)
        .map(|x| (x as f64).sqrt())
        .collect();
    println!("vec_sqrt: {vec_sqrt:?}");
}
