extern crate image;
extern crate notify;
extern crate gif;

use std::fs::File;
use std::path::PathBuf;
use std::io::{Read, BufReader };


use gif::SetParameter;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;

const WIDTH: u16 = 100;
const HEIGHT: u16 = 100;

fn watch() -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("C:\\Users\\Minauteur\\io-png\\loops", RecursiveMode::Recursive)?;
    // try!(watcher.watch("/var/www/microwavemansion.com/loops", RecursiveMode::Recursive));
    

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(event) => {
                if let DebouncedEvent::Create(file_path) = event {
                    println!("file added to /loops! checking extension...");
                    match file_path.clone().extension() {
                        Some(os_str) => {
                            match os_str.to_str() {
                                Some("wav") => {
                                    println!("wav file added! checking for image...");
                                    if !check_img(&file_path) {
                                        println!("found a wav file, but no corresponding image! generating...");
                                        generate_img(&file_path);
                                    } else {
                                        println!("We already have an image for this audio. neato!");
                                    }
                                },
                                Some("mp3") => {
                                    println!("mp3 file added! checking for image...");
                                    if !check_img(&file_path) {
                                        println!("found an mp3 file, but no corresponding image! generating...");
                                        generate_img(&file_path);
                                    } else {
                                        println!("We already have an image for this audio. neato!");
                                    }
                                },
                                Some(ext) => println!("file added wasn't an mp3 or a wav! ext == {}", &ext),
                                _ => println!("Well, this is awkward..."),
                            }
                        },
                        None => {
                            println!("no extension found!");
                        }
                    }
                } else {
                    println!("{:?}", event)
                }                
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn check_img(p_buf: &PathBuf) -> bool {
    p_buf.with_extension("gif").exists()
}

fn generate_img(p_buf: &PathBuf) {
    let mut image_buffer = match File::create(p_buf.with_extension("gif")) {
        Ok(file) => file,
        Err(e) => panic!("error! {}", e),
    };
    let mut bytes_vec: Vec<u8> = Vec::new();
    if let Ok(audio_data) = File::open(p_buf) {
        let mut gif = gif::Encoder::new(&mut image_buffer, WIDTH, HEIGHT, &[]).expect("problem initializing encoder!");
        gif.set(gif::Repeat::Infinite).unwrap();
        // let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH, HEIGHT);
        BufReader::new(audio_data).read_to_end(&mut bytes_vec).expect("something went wrong  reading audio data!");

        let mut bytes_iter = bytes_vec.into_iter();
        let mut frame_count = 1;
        while frame_count <= 20 {
            let mut cur_frame = Vec::new();
            for _ in 0..30000 {
                    if let Some(rgb) = bytes_iter.next() {
                        cur_frame.push(rgb);
                    }
                    else { cur_frame.push(0u8) }
            }

            let new_frame = gif::Frame::from_rgb(100, 100, &cur_frame.as_slice());
            gif.write_frame(&new_frame).expect("something went wrong writing frame data!");
            println!("wrote frame {}!", &frame_count);
            frame_count = frame_count+1;
        }
        println!("wrote image!");
    }
}

fn main() {
    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}
