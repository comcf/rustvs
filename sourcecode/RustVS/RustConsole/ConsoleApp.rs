fn main() {
    trial::public_function();

    //Error! `private_function` is private
    //trial::private_function();

    trial::indirect_access();
    println!("Hello, world!");
}
