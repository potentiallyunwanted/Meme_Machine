use std::{io, path::Path};
use image::Rgba;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};


fn main() {
    struct Meme{
        name: String,
        text_top: String,
        text_bottom: String,
        image_path: String
    }

    let mut one_does_not = Meme {
        name: String::from("One Does Not Simply"),
        text_top: String::from("One Does Not Simply"),
        text_bottom: String::new(),
        image_path: String::from("/home/qduong/rust_projects/meme_machine/src/one_does_not.png")
    };

fn make_meme(meme: &Meme) {

    let path = Path::new(&meme.image_path);
    let mut meme_image = image::open(path).unwrap();

    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 25.0;
    let scale = Scale {
        x: height * 2.0,
        y: height,
    };

    let color = Rgba([255u8, 255u8, 255u8, 0u8]);

    draw_text_mut(&mut meme_image, color, 40, 0, scale, &font, &meme.text_top);
    draw_text_mut(&mut meme_image, color, 40, 250, scale, &font, &meme.text_bottom);
    let _ = meme_image.save(Path::new("new_meme.png")).unwrap();

    println!("Your meme has been created.");
}


    const APP_TITLE: &str = "Meme Machine";
    println!("This App is called {}", APP_TITLE);

    println!("Pick the meme you want to make:");
    println!("1. {}", one_does_not.name);
    
    let mut pick = String::new();

    io::stdin()
        .read_line(&mut pick)
        .expect("Failed to read line");

    let pick: u32 = pick.trim().parse().expect("Please type a number!");

    if pick == 1{
        println!("You picked the One Does Not Simply...");
        println!("What is your bottom text going to be?");

        let mut bottom = String::new();

        io::stdin()
            .read_line(&mut bottom)
            .expect("Failed to read line");
        
        one_does_not.text_bottom = bottom.trim().to_string();
        
        make_meme(&one_does_not);

    }else{
        println!("Please pick 1 or 2. Thanks.")
    }


}
