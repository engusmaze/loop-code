use loop_code::repeat;

fn main() {
    let mut a = [5, 3, 1, 3];
    println!("{:?}", a);

    let b = [5, 2, 15, 0];

    repeat!(I /* You can type in here a variable type, but default type is usize */ 4 {
        a[I] += b[I];
    });

    println!("{:?}", a);
}
