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
            graph.add_sample(
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
