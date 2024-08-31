fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

fn main() {
    let mut a = 10;
    let mut b = 20;
    swap(&mut a, &mut b);
    println!("a = {}, b = {}", a, b); // Output: a = 20, b = 10
}