
fn main() {
    println!("{}",hello());
    forloop(10);
}
// &'static is a "lifetime specifier", something you'll learn more about later
pub fn hello() -> &'static str {
    "Hello, World!"
}

fn forloop(loop_time: i32){
    for _i in 0..loop_time{
        println!("{}:{}",hello(),_i);
    }
}
