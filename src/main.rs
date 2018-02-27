extern crate image;
extern crate notify;
// extern crate byteorder;
// use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use std::path::Path;
use std::fs::File;
use std::path::PathBuf;
use std::io::{Read, Write, BufReader, BufWriter};
use std::ffi::OsStr;

use image::png::PNGEncoder;
use image::png;
use image::gif::Encoder;
use image::{save_buffer, ImageBuffer, ColorType, Rgb};


use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;

const WIDTH: u32 = 100;
const HEIGHT:u32 = 100;

fn watch() -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(2)));

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    // try!(watcher.watch("C:\\cygwin64\\home\\Minauteur\\io-png\\loops", RecursiveMode::Recursive));
    try!(watcher.watch("/var/www/microwavemansion.com/loops", RecursiveMode::Recursive));
    

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
    p_buf.with_extension("png").exists()
}

fn generate_img(p_buf: &PathBuf) {
    let image_buffer = match File::create(p_buf.with_extension("png")) {
        Ok(file) => file,
        Err(e) => panic!("error! {}", e),
    };
    let mut bytes_vec: Vec<u8> = Vec::new();
    if let Ok(audio_data) = File::open(p_buf) {
        // let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH, HEIGHT);
        BufReader::new(audio_data).read_to_end(&mut bytes_vec).unwrap();
        if bytes_vec.len() >= 1024*1024 {
            bytes_vec = bytes_vec[bytes_vec.len()-bytes_vec.len()/2..bytes_vec.len()-bytes_vec.len()/4].to_owned();
            if bytes_vec.len() >=1024*1024 {
                bytes_vec = bytes_vec[..1024*1024].to_owned()
            }
        }
        // let audio_bytes = bytes_vec[bytes_vec.len()%3..bytes_vec.len()].to_owned();
        // for pixel in image.pixels_mut() {
        //     if let Some(data) = audio_bytes.into_iter().next() {
        //         pixel.data = Rgb();
        //     }
        // }
        PNGEncoder::new(image_buffer).encode(&bytes_vec, 100, 100, ColorType::RGB(16)).expect("something went wrong writing to image file!");
        println!("wrote image!");
    }
}

fn main() {
    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}
