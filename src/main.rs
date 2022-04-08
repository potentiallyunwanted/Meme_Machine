use std::io;

fn main() {
    struct Meme{
        name: String,
        text: String
    }

    let will_slap = Meme {
        name: String::from("Will Smith Slapping Chris Rock"),
        text: String::new()
    };

    let two_button = Meme {
        name: "Two Buttons".to_string(),
        text: String::new()
    };

    const APP_TITLE: &str = "Meme Machine";
    println!("This App is called {}", APP_TITLE);

    println!("Pick the meme you want to make:");
    println!("1. {}", will_slap.name);
    println!("2. {}", two_button.name);
    
    let mut pick = String::new();

    io::stdin()
        .read_line(&mut pick)
        .expect("Failed to read line");

    let pick: u32 = pick.trim().parse().expect("Please type a number!");

    if pick == 1{
        println!("You picked the Will Smith Slapping Chris Rock Meme")
    }else if pick == 2{
        println!("You picked the Two Buttons Meme")
    }else{
        println!("Please pick 1 or 2. Thanks.")
    }


}
