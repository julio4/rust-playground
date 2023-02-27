fn set_to_zero(slice: &mut [i32]) {
    slice.fill(0);
}

fn main() {
    // 1
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Vector: {vector:?}");

    // 2
    let mut sum = 0;
    for v in &vector {
        sum += v;
    };
    println!("Sum: {sum:?}");

    //3
    let mut vector2: Vec<i32> = vec![0; 5];
    println!("Vector2: {vector2:?}");
    
    //4
    for i in 1..6 {
        vector2[i - 1] = i as i32;
    }
    assert_eq!(vector, vector2);
    println!("vector and vector2 are equals!");

    //5
    let slice = vector2.as_mut_slice();
    set_to_zero(slice);
    println!("Vector2 after set_to_zero: {vector2:?}");
}
