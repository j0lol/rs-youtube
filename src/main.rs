use directories::ProjectDirs;
use serde_derive::Deserialize;
use serde_json::Value;
use std::fs;
use std::io;
use std::io::Read;
use std::process::{Command, Output};
use toml::value::Array;

// Define modules & import them
mod backend;
use backend::*;
mod frontend;
use frontend::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    frontend::main_menu::show_menu();

    Ok(())
}

// Make a link made of two parts
fn make_url(prefix: &str, suffix: &str) -> String {
    [prefix, suffix].join("")
}

// Run a command on windows, linux or mac
fn run_command(command: &str) -> Output {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };
    output
}

// Select results
/*
fn results_selector(vec: Vec<Results>) {
    clear_screen();

    for i in 0..vec.len() {
        let message = match &vec[i] {
            Results::Video(video) => video.summarize(),
            Results::Channel(channel) => channel.summarize(),
            Results::Playlist(playlist) => playlist.summarize(),
            Results::Shelf(shelf) => shelf.summarize(),
            Results::None => String::from(""),
        };
        println!("{}) {}", i, message);
    }
    println!("Select an item.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    match input {
        "exit" => {}
        input => {
            let input: usize = input.parse().unwrap();
            if input < vec.len() {
                clear_screen();

                match &vec[input] {
                    Results::Video(video) => {
                        println!(
                            "▶ Now playing: {} by {}\nhttps://youtube.com/watch?v={}",
                            video.title, video.owner, video.id
                        );
                        let command = make_url("mpv https://youtube.com/watch?v=", &*video.id);
                        run_command(&command);
                    }

                    Results::Channel(channel) => {
                        //println!("{} {}", channel.id, channel.name);
                        show_channel(&*channel.id);
                    }

                    Results::Playlist(playlist) => {
                        println!(
                            "▶ Now playing playlist: {}\nhttps://youtube.com/playlist?list={}",
                            playlist.title, playlist.id
                        );
                        let command =
                            make_url("mpv https://youtube.com/playlist?list=", &*playlist.id);
                        run_command(&command);
                    }
                    Results::Shelf(shelf) => {
                        let vec = shelf.content.to_vec();
                        results_selector(vec);
                    }
                    Results::None => {
                        println!("Option picked is blank.")
                    }
                }
            } else {
                println!("Not a valid number!")
            }
        }
    }
}

// Select results
fn channel_video_selector(vec: Vec<Results>) {
    clear_screen();

    for i in 0..vec.len() {
        let message = match &vec[i] {
            Results::Video(video) => video.summarize(),
            Results::Channel(channel) => channel.summarize(),
            Results::Playlist(playlist) => playlist.summarize(),
            Results::Shelf(shelf) => shelf.summarize(),
            Results::None => String::from(""),
        };
        println!("{}) {}", i, message);
    }
    println!("Select an item.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    match input {
        "exit" => {}
        input => {
            let input: usize = input.parse().unwrap();
            if input < vec.len() {
                clear_screen();

                match &vec[input] {
                    Results::Video(video) => {
                        println!(
                            "▶ Now playing: {} by {}\nhttps://youtube.com/watch?v={}",
                            video.title, video.owner, video.id
                        );
                        let command = make_url("mpv https://youtube.com/watch?v=", &*video.id);
                        run_command(&command);
                    }

                    Results::Channel(channel) => {
                        //println!("{} {}", channel.id, channel.name);
                        show_channel(&*channel.id);
                    }

                    Results::Playlist(playlist) => {
                        println!(
                            "▶ Now playing: {}\nhttps://youtube.com/playlist?list={}",
                            playlist.title, playlist.id
                        );
                        let command =
                            make_url("mpv https://youtube.com/playlist?list=", &*playlist.id);
                        run_command(&command);
                    }
                    Results::Shelf(shelf) => {
                        let vec = shelf.content.to_vec();
                        results_selector(vec);
                    }
                    Results::None => {
                        println!("Option picked is blank.")
                    }
                }
            } else {
                println!("Not a valid number!")
            }
        }
    }
}
*/
