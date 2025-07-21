
// Fill in the blanks
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn main() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");
} 

pub fn show_message(msg: Message) {
    match msg {
        Message::Move{x:a,y:b} => { // match  Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(r, g, b) => {
            assert_eq!(g,255);
            assert_eq!(b,0);
        }
        _ => println!("no data in these variants")
    }
}