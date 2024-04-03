fn main() {
    println!("{}", "Welcome");
    let mt num_times : u32 = 0;
    for i in 1..301 {
        match (i%3, i%5) {
            (0,0) => {
                println!("fizzbuzz");
                num_times += 1;
            }
            (0,_) => println!("fizz");
            (_,0) => println!("buzz");
            _ => {}
        }
    }
    println!("{}", num_times);
}