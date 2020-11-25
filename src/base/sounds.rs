use rodio::Sink;
use std::fs::File;
use std::io::BufReader;

pub struct Soundboard {
    sink_pool: Vec<Sink>,
}

impl Soundboard {
    pub fn new(sink_count: u64) -> Soundboard {
        let device = rodio::default_output_device().unwrap();
        let mut sinks: Vec<Sink> = Vec::new();
        for _ in 1..(sink_count + 1) {
            sinks.push(Sink::new(&device));
        }
        Self { sink_pool: sinks }
    }
    pub fn play_sound(&self, path : &str, volume : &f32){
        println!("Playing Sound \"{}\"", path);
        let mut found_sink = false;
        for sink in &self.sink_pool {
            if sink.empty() {
                let file = File::open(path);
                match file {
                    Ok(f) => {
                        let source = rodio::Decoder::new(BufReader::new(f));
                        match source {
                            Ok(s) => {
                                sink.set_volume(*volume);
                                sink.append(s);
                                sink.play();
                            },
                            Err(e) => {
                                println!("Couldn't Decode Sound File: {}", err);
                            }
                        }
                    },
                    Err(e) => {
                        println!("Couldn't Open File: {}", e);
                    },
                }
                // rodio::play_raw(&device, source.convert_samples());
                found_sink = true;
                break;
            }
        }
        if !found_sink {
            println!("`Too many sounds are playing at once,\
                No sound was played, please wait for sounds to end,\
                or increase the number of audio pools by using the\
                \"--sinks\" option at startup. See \"midibase run --help\"\
                for more information on the \"--sinks\" option"
            );
        }
    }
}

