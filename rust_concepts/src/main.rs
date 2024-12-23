fn main() {
    println!("Hello, world!");

    // exemple_todo();
    exemple_sub_argument();
    variaveis();
    sombreamento();
}

fn exemple_sub_argument() {
    println!("\nExemplo de como concatenar variaveis ou valores em uma string");
    println!("{} + {} = {}", 1, 2, 3);
}

fn variaveis() {
    // variavel imutavel (variavel)
    let a_number = 10;

    // variavel mutavel (constante)
    let mut b_number = 20;
    
    println!("\nb: {}", b_number);
    
    b_number = 230;

    println!("a: {}, b: {}", a_number, b_number);
}

fn sombreamento() {
    let shadow_num = 5;
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num + 5;

    println!("shadow_num: {}", shadow_num);
}

// fn exemple_todo() {
//     todo!("Menssagem de alerta quando nãop tem uma funcionalidade concluida");
// }
