mod stdvec;
mod stdarray;


fn main() {
    println!("Hello, stdlib!");

    println!("\n-----------  Vec  ---------------");
    stdvec::main();

    println!("\n----------- array ---------------");
    stdarray::main();
}
