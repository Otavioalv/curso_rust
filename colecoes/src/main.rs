fn main() {
    println!("Exemplo de tupla\n");
    exemplo_tupla();

    println!("Exemplo de structs\n");
    exemplo_struct();
}

fn exemplo_tupla() {
    let tupla = ("E", 4u8, true, "string");

    println!("0: {}, 1: {}, 2: {}, 3: {}", tupla.0, tupla.1, tupla.2, tupla.3);
}


fn exemplo_struct() {
    /*  
        o rust da suporte a tres tipos de structs

        struct classico: mais usados. cada campo tem um nome e um tipo de dado (<struct>.<field>>) (parecido com obj em js)
        struct de tupla: semelhantes ao classico, porem os campos nao tem nomes (<tuple>.<index>)
        struct de unidade: usados como marcadores.

        o nome das structs e em maiusculo
    */

    // Classic struct 
    struct Student {
        name: String, 
        level: u8, 
        remote: bool
    }

    // tuple struct
    struct Grades(char, char, char, char, f32);

    // Unit struct
    struct Unit;

    // Instatiate classic struct
    let user_1 = Student {
        name: String::from("Nome estudante 1"), 
        remote: true,
        level: 3
    };

    let user_2 = Student {
        name: "Nome estudante 2".to_string(),
        remote: false,
        level: 1
    };

    /* 
        String::from($str); converte um literal de cadeia de caracteres em um tipo string
        ou usar a função -> .to_string();
    */


    // Instatiate tuple structs
    let mark_2 = Grades ('A', 'B', 'B', 'A', 3.23);
    let mark_1 = Grades ('A', 'A', 'A', 'A', 4.00);

    println!("Student 1:\n\tName: {}\n\tremote: {}\n\tlevel: {}\n", user_1.name, user_1.remote, user_1.level);
    println!("Mark_1:\n\t1-{}\n\t2-{}\n\t3-{}\n\t4-{}\n\ttotal-{}\n", mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);

    println!("Student 1:\n\tName: {}\n\tremote: {}\n\tlevel: {}\n", user_2.name, user_2.remote, user_2.level);
    println!("Mark_1:\n\t1-{}\n\t2-{}\n\t3-{}\n\t4-{}\n\ttotal-{}\n", mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);
}

