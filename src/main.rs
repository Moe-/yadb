pub mod yadb;

fn main() {
    println!("Hello, world!");

    yadb::attach::attach();
}
