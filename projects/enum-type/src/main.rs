fn main() {
    struct KeyPress(String, char);
    struct MouseClick {
        x: i64,
        y: i64
    }
    
    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    enum WebEvent {
        WELoad(bool),
        WEKeys(KeyPress),
        WEClick(MouseClick)
    }
    
    let we_load = WebEvent::WELoad(true);
    
    let click = MouseClick { x: 100, y: 250 };
    let we_click = WebEvent::WEClick(click);
    
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    let we_keys = WebEvent::WEKeys(keys);
    
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_keys)
}
