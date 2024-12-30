fn main() {
    println!("Hello, world!");

    // exemple_todo();
    println!("Exemplo de colocar variaveis concatenadas no print (exemple_sub_argument) \n");
    exemple_sub_argument();

    println!("Exemplo de variaveis (variaveis)\n");
    variaveis();

    println!("Exemplo de sombreamento (sombreamento)\n");
    sombreamento();
}

fn exemple_sub_argument() {
    println!("{} + {} = {}", 1, 2, 3);
}

fn variaveis() {
    // variavel imutavel (constante)
    let a_number = 10;

    // variavel mutavel (variavel)
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
