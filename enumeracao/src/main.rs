fn main() {
    println!("Exemplo de enumeração\n");
    exemplo_enumeracao();
}

fn exemplo_enumeracao() {
    #[derive(Debug)]
    struct KeyPress(String, char);
    
    #[derive(Debug)]
    struct MouseClick{x: i64, y:i64}
    

    // Criação do enum
    #[derive(Debug)]
    enum WebEvent{
        WELoad(bool),
        WEClick(MouseClick),
        WEKeys(KeyPress)
    }

    let we_load:WebEvent = WebEvent::WELoad(true);

    let mouse_click:MouseClick = MouseClick {x: 12334, y: 3422};
    let we_click:WebEvent = WebEvent::WEClick(mouse_click);
    let we_click_2:WebEvent = WebEvent::WEClick(MouseClick{x: 2342, y: 2894});

    let key_press:KeyPress = KeyPress(String::from("CTRL+"), 'N');
    let we_keys:WebEvent = WebEvent::WEKeys(key_press);
    let we_keys_2:WebEvent = WebEvent::WEKeys(KeyPress(String::from("CTRL+"), 'C'));

    
    println!("we_load:\n\t{:#?}\n", we_load);
    println!("we_click:\n\t{:#?}\n", we_click);
    println!("we_key:\n\t{:#?}\n", we_keys);


    println!("we_click_2:\n\t{:#?}\n", we_click_2);
    println!("we_key_2:\n\t{:#?}\n", we_keys_2);
}   