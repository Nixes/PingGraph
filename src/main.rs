extern crate piston_window;
extern crate time;
extern crate regex;
extern crate rust_plot;

use piston_window::*;
use std::result::Result;
use time::*; // used for timers to measure performance
use std::process::Command; // for running ping command
use std::thread::sleep_ms;
use regex::Regex; // used to parse ping command result
use rust_plot::Plot;

struct app_str {
    samples_sec: u64,
    samples_max: usize,
    border: i32,
	window_width: u32,
	window_height: u32
}

fn get_unix_ping (host:String) -> u32 {
    let output = Command::new("ping")
                    .arg("-c")
                    .arg("1")
                    .arg("-w")
                    .arg("800") // give a timeout in ms
                    .arg(host) // for osx the host needs to be the last argument
                    .output()
                    .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
	let stringStdOut = String::from_utf8_lossy(&output.stdout);
    let mut finalvalue :u32 = 0;
    if stringStdOut.contains("Request timed out.") {
        finalvalue = 1000; // over 1 second
        println!("Ping Timed Out.");
    } else {
        let value = stringStdOut.split(" time=").last().unwrap(); // get part of string after this piece of text, then remove the units
        let value2 = value.split("ms").nth(0).unwrap(); // get rid of excess
        // this mess here try's to first convert to a floating point number, and rounds it. Failing that it converts the string to a unsigned int directly
        match value2.trim().parse::<f32>() {
            Ok(val) => {
                finalvalue = val.round() as u32;
            }
            Err(why) => {
                match value2.trim().parse::<u32>() {
                    Ok(val) => {
                        finalvalue = val;
                    }
                    Err(why) => {}
                }
            }
        }
    }
    println!("Ping: {}", finalvalue);
    finalvalue
}

fn get_win_ping (host:String) -> u32 { // TODO: improve error handling
	let output = Command::new("ping")
                    .arg(host)
                    .arg("-n")  // tell to only perform one ping
                    .arg("1")
                    .arg("-w")
                    .arg("800") // give a timeout in ms
                    .output()
                    .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
	let stringStdOut = String::from_utf8_lossy(&output.stdout);
    let mut finalvalue :u32 = 0;
    if stringStdOut.contains("Request timed out.") {
        finalvalue = 1000; // over 1 second
        println!("Ping Timed Out.");
    } else {
        let value = stringStdOut.split("Average = ").last().unwrap().replace("ms", ""); // get part of string after this piece of text, then remove the units
        finalvalue = value.trim().parse::<u32>().unwrap();
    }
    println!("Ping: {}", finalvalue);
    finalvalue
}

/* have not yet been able to draw text, so disabling this for now 
fn drawLastPingValue() {
    let mut normal_text = graphics::text::new(factory.clone()).unwrap();
}*/

fn main() {
	let mut application = app_str{samples_sec:1,samples_max:150,border: 0,window_width: 600, window_height: 300};

    let opengl = OpenGL::V2_1; // lowest supported opengl version
    let windowsettings = WindowSettings::new("Ping Graph", [application.window_width, application.window_height]).vsync(false);
    //windowsettings.vsync(true);

    let mut window: PistonWindow =
        windowsettings
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();
    window.set_max_fps(application.samples_sec);
    window.set_ups(application.samples_sec);
    //sleep_ms(pollingTime);
    let mut graph = Plot::new(application.samples_max,application.window_width);

    for e in window {
        e.update(|args| {
            graph.add_point(
                if cfg!(windows) {
                    get_win_ping("www.google.com".to_string())
                } else {
                    get_unix_ping("www.google.com".to_string())
                }
            )
        });
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);
            graph.draw(c,g);
        });
		e.mouse_scroll(|dx, dy| { });
        e.press(|button| { });
        // TODO: catch resize event and change application window size values
    }
}
