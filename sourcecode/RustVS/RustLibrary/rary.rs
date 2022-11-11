pub fn public_function() {
    println!("called trial's `public_function()`");
}

fn private_function() {
    println!("called trial's `private_function()`");
}

pub fn indirect_access() {
    print!("called trial's `indirect_access()`, that\n> ");

    private_function();
}