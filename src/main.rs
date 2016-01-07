extern crate piston_window;
extern crate time;
extern crate regex; // used to parse ping command result

use piston_window::*;
use std::result::Result;
use time::*; // used for timers to measure performance
use std::process::Command; // for running ping command
use std::thread::sleep_ms;
use regex::Regex;

// polygon
struct graph_obj {
    line_values: Vec<u32>,
    line_max_samples: usize,
    line_height_scale_factor:f64, // how much to multiply the sample value by to produce the number of pixels high the line is
    line_gap:u16,
    line_thickness:f64
}

impl graph_obj {
    pub fn new(max_samples:usize,window_width:u32) -> graph_obj {
        // calculate the line thicknes based on the size of window to fill and the max sample number
        let line_gap :u16 = 0;
        let line_thickness:f64 = (window_width as f64 / max_samples as f64) - line_gap as f64;
        println!("Line thickness calced: {}",line_thickness);
        graph_obj{line_values: Vec::with_capacity(max_samples),line_max_samples:max_samples,line_height_scale_factor:1.0,line_gap:line_gap,line_thickness:line_thickness }
    }

    pub fn add_sample (&mut self,sample:u32) {
        if self.line_values.len() >= self.line_max_samples {
            self.line_values.remove(0);// remove oldest value / value in first index
            self.line_values.push(sample); // then add latest value to end
        } else {
            self.line_values.push(sample); // not yet filled so just add value
        }
    }
    pub fn gen_test(&mut self,min:u32,max:u32) {
        let step_amount: u32 = (max - min) / self.line_max_samples as u32;

        for i in 0..self.line_max_samples as u32 {
            self.add_sample(i * step_amount);
            println!("Gen sample: {}",i * step_amount);
        }
    }
    //pub fn draw (&self) {} // TODO: figure out how to implement a recursive draw function
}

struct app_str {
    samples_sec: u64,
    samples_max: usize,
    border: i32,
	window_width: u32,
	window_height: u32
}

fn get_ping (host:String) -> u32 { // TODO: improve error handling
	let output = Command::new("ping")  // sets the brightness, might be 0-100 or -100 to 100?
                    .arg(host) // tell to only perform one ping
                    .arg("-n")
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
    //println!("Result: {}",stringStdOut);
    //println!("status: {}", output.status);
    //println!("stdout: {}", stringStdOut);
    finalvalue
}

// very similar to a HSV to RGB conversion
fn convert_value_to_color(value: u32,max_value: u32) -> [f32; 4] { // used for heatmap colors
    let mut color = [0.0, 0.5, 0.0, 1.0];
    if value >= 1000 { // invalid result (connection lost?)
        color = [0.0, 0.0, 0.0, 0.5];
    } else if value >= max_value {
        color = [1.0, 0.0, 0.0, 1.0];
    }
    // normalise the input value
    let norm_value :f32 = value as f32 / max_value as f32;

    if norm_value <= 0.25 {
        color = [0.0, norm_value*4.0, 1.0, 1.0]; //color.push_back(ColorPoint(0, 1, 1,   0.25f));// Cyan.
    } else if norm_value > 0.25 && norm_value <= 0.5 {
        color = [0.0, 1.0, 1.0-((norm_value-0.25)*4.0), 1.0]; //color.push_back(ColorPoint(0, 1, 0,   0.5f)); // Green.
    } else if norm_value > 0.5 && norm_value <= 0.75 {
        color = [(norm_value-0.5)*4.0, 1.0, 0.0, 1.0]; //color.push_back(ColorPoint(1, 1, 0,   0.75f));// Yellow.
    } else if norm_value > 0.75 && norm_value <= 1.0 {
        color = [1.0, 1.0-((norm_value-0.75)*4.0), 0.0, 1.0]; //color.push_back(ColorPoint(1, 0, 0,   1.0f)); // Red.
    }
    color
}

fn main() {
	let mut application = app_str{samples_sec:1,samples_max:150,border: 0,window_width: 600, window_height: 300};

	// preferably use V3_2, but can use v2_1
    let opengl = OpenGL::V2_1;
    let windowsettings = WindowSettings::new("Ping Graph", [application.window_width, application.window_height]).vsync(false);
    //windowsettings.vsync(true);
    //println!("Vsync was: {}",windowsettings.get_vsync() );
    let mut window: PistonWindow =
        windowsettings
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();
    window.set_max_fps(application.samples_sec); // setting to 80 gets around 65 fps
    window.set_ups(application.samples_sec);
    //sleep_ms(pollingTime);
    let mut graph = graph_obj::new(application.samples_max,application.window_width);
    //graph.gen_test(&mut graph, 0, 300, 50);

    for e in window {
        e.update(|args| {
            graph.add_sample( get_ping("www.google.com".to_string()) )
        });
        e.draw_2d(|c, g| {
            clear([1.0; 4], g); // white was clear([1.0; 4], g); original
            //rectangle([0.8, 0.8, 0.8, 1.0],[application.border as f64, application.border as f64, (application.window_width - (application.border*2) ) as f64, (application.window_height - (application.border*2) ) as f64],c.transform, g);
            //graphics::grid(10,10,10);
            for (itemNo,&item) in graph.line_values.iter().enumerate() {
                //println!("One bar drawn value: {} number: {}",item,itemNo);
                let color = convert_value_to_color(item,200);
                rectangle(color,[itemNo as f64 * (graph.line_thickness + graph.line_gap as f64),
                                                300.0,
                                                graph.line_thickness,
                                                item as f64 * graph.line_height_scale_factor * -1.0]
                                                /* x,y,width,height */
                                                ,c.transform, g);
                //text([0.5, 0.0, 0.0, 1.0],10,"Something",[10, 10],c.transform,g);
            }
        });
		e.mouse_scroll(|dx, dy| { });
        e.press(|button| { });
        // TODO: catch resize event and change application window size values
    }
}
