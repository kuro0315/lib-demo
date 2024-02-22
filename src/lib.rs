mod generator;

pub fn print_ran() {
    let n = generator::gen_ran();
    println!("Random number: {}", n);
}
