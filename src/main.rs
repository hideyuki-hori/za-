use rodio::{OutputStreamBuilder, Sink};

mod noise;
mod types;
use types::Noise;

fn main() {
    let stream = OutputStreamBuilder::open_default_stream().unwrap_or_else(|e| {
        println!("Could not open audio stream: {}", e);
        std::process::exit(1);
    });
    let sink = Sink::connect_new(stream.mixer());

    println!("1 - White");
    println!("2 - Pink");
    println!("3 - Brown");
    println!("q - Quit");

    let play = |noise: Noise| {
        println!("{}", noise.state);
        sink.stop();
        sink.append(noise);
    };

    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(e) => {
                println!("ERROR: {}", e);
                break;
            }
        }
        let command = input.trim();

        match command {
            "1" => play(Noise::white()),
            "2" => play(Noise::pink()),
            "3" => play(Noise::brown()),
            "q" => break,
            _ => println!("???"),
        }
    }

    drop(stream);
}
