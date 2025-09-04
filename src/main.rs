use rodio::{OutputStreamBuilder, Sink};

mod noise;
mod types;
use types::Noise;

fn main() {
    let stream =
        OutputStreamBuilder::open_default_stream().expect("Could not open default audio stream");
    let sink = Sink::connect_new(stream.mixer());

    println!("1 - white");
    println!("2 - pink");
    println!("3 - brown");
    println!("s - stop");
    println!("q - quit");

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
            "1" => {
                println!("white");
                sink.clear();
                sink.append(Noise::white());
            }
            "2" => {
                println!("pink");
                sink.clear();
                sink.append(Noise::pink());
            }
            "3" => {
                println!("brown");
                sink.clear();
                sink.append(Noise::brown());
            }
            "s" => {
                sink.stop();
            }
            "q" => {
                break;
            }
            _ => {
                println!("???");
            }
        }
    }

    drop(stream);
}
