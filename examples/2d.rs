use loop_code::repeat;

fn main() {
    repeat!(X /* Default type is usize */ 4 {
        repeat!(Y i32 4 {
            println!("X: {} Y: {}", X, Y);
        });
    });
}
