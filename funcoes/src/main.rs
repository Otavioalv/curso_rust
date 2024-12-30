fn main() {
    let formal:&str = "falou truta enois";
    let casual:&str = "este parsseio foi deveras interesante, nos vemos mais tarde";

    goodbye(formal);
    goodbye(casual);

    let num:u32 = 25;
    let result:u32 = divide_by_5(num);

    println!("result: {}", result)

}

fn goodbye(message: &str) {
    println!("\n{}", message);
}

fn divide_by_5(num: u32) -> u32{
    
    if num == 0 {
        return 0;
    }

    return num / 5;
}