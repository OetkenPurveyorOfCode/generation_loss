use std::io::Cursor;
use image::{ImageReader, DynamicImage};


fn main() -> std::io::Result<()> {
    let args : std::vec::Vec<std::string::String> = std::env::args().collect();
    let usage = "generation_loss.exe <infile> <generations> <outfile> <format>";
    let example = "generation_loss.exe cool.png 100 out jpg";
    if args.len() != 5 {
        eprintln!("Expected 5 positional arguments got {}", args.len());
        eprintln!("{usage}");
        eprintln!("Here is an example: {example}");
        std::process::exit(0);  
    }
    let generation_str = &args[2].parse::<usize>();
    let generations = match generation_str {
        Ok(generations) => generations,
        Err(err) => {
            eprintln!("Invalid generation number: {} - string: {}", err, &args[2]);
            std::process::exit(0);
        }
    };
    let filename = &args[3];
    let fileformat = &args[4];

    let mut img: DynamicImage = ImageReader::open(&args[1])?.decode().unwrap();
    for i in 0..*generations {
        img.save(format!("{filename}{i}.{fileformat}")).unwrap();
        img = ImageReader::open(format!("{filename}{i}.{fileformat}"))?.decode().unwrap();
        println!("Generation: {i}");
    }
    return Ok(());
}
