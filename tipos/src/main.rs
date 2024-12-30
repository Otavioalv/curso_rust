fn main() {
    println!("Variaveis inteiras");
    variavel_inteira();
    
    println!("Variaveis float");
    variavel_float();

    println!("Operações matematicas");
    operacoes_matematicas();

    println!("Variavel boleana");
    variavel_booleana();

    println!("Variavel caracter");
    variavel_caracter();
}

fn variavel_inteira() {
    // se for negativo tem o "i" no inicio (i de inteiro)
    // se não, tem o "u" no inicio

    // tipo inteiro no rust
    // 8 - inteiro 8 bits
    // 16 - inteiro 16 bits
    // 32 - inteiro 32 bits
    // 64 - inteiro 64 bits

    // exemplo de inteiro negativo
    let number_negative_8bits: i8 = -128; // tipo inteiro negativo com 8 bits

    // exenplo de inteiro positivo
    let number_variable: u32 = 4294967295; // tipo inteiro com 32 bits


    // associar numero negativo ao tipo positivo (erro)
    // let number_neg: u8 = 1;
    
    // associar numero positivo ao tipo negativo (erro)
    // let number_pos: i8 = -8;

    // println!("number_neg: {} number_pos: {}", number_neg, number_pos);


    println!("negativo: {} positivo: {}\n", number_negative_8bits, number_variable);
}

fn variavel_float() {
    // existem 2 tipos de floats
    // f32 e f64
    // por padrão e associado o tipo f64

    let number_f = 3.5; // tipa por padrão o 64 bits
    let number_f_32: f32 = 3.4;

    println!("64: {} 32: {}\n", number_f, number_f_32);
}

fn operacoes_matematicas() {
    println!("1 + 2 = {} and 8 - 5 = {}  and 15 * 3 = {}\n", 1u32 + 2, 8i32 - 5, 15 * 3);
    println!("9 / 2 = {} but 9.0 / 2.0 = {}\n", 9u32 / 2, 9.0 / 2.0); 
}

fn variavel_booleana() {
    // colocar (bool)
    let is_bigger:bool = 1 > 4;

    println!("is i > 4: {} \n", is_bigger);
}

fn variavel_caracter() {
    // tipo char, somente um caracter com aspas simples
    let character: char = 'c';

    let emoji:char = '🍆';
    
    let string: &str = "Cadeia de caracteres  ";

    println!("caracter: {} - emoji: {} - string: {}\n", character, emoji, string);
}