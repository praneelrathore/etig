/*
Cycle Detection using threads
SP Harish IIT2013134
Shubham Gupta IIT2013180
*/
use std::io;
//extern crate time;
//use time::PreciseTime;
use std::thread;
//use std::sync::mpsc;
//use std::collections::VecDeque;
// use std::sync::{Arc, Mutex};
// use std::time::Duration;
// use std::vec::Vec;

fn input() -> i32 {
	let mut input_text = String::new();
	io::stdin()
		.read_line(&mut input_text)
		.expect("failed to read from stdin");

	let trimmed = input_text.trim();
	match trimmed.parse::<i32>() {
		Ok(i) => return i,		//println!("your integer input: {}", i),
			Err(..) =>return -1i32,	//println!("this was not an integer: {}", trimmed)
	};
}


static mut NTHREADS180 : i32 = 30;
static mut cnt180: i32 = 1;
static mut n180: i32 = 0;
static mut adj180: [[i32; 500]; 500] = [[0 ; 500]; 500];
static mut cycle_found180: bool = false;
static mut visited180: [bool; 500] = [false; 500];
//static mut queue:VecDeque<Thread> = VecDeque::new();
pub fn cycle_detection(mat134180: &mut [[i32; 500]; 500], n1: i32) {
	unsafe {
		//n = input();
		//let e: i32 = input();
		/*for _ in 0..e {
			let nod_1: i32 = input();
			let nod_2: i32 = input();
			//static mut queue:VecDeque<Thread> = VecDeque::new();

			adj[nod_1 as usize][nod_2 as usize] = true;
			adj[nod_2 as usize][nod_1 as usize] = true;
		}
		*/
		
		n180= n1;
		for i in 0..n180 {
			for j in 0..n180 {
				adj180[i as usize][j as usize] = mat134180[i as usize][j as usize]; 
			}
		}
		//let start = PreciseTime::now();
		for i in 0..n180 {
			if !visited180[i as usize] {
				dfs134180(i, -1i32);
			}
		}

		//let end = PreciseTime::now();
		if !cycle_found180 {
			println!("no cycle present");
		} else {
			println!("cycle present");
		}

		//println!("{:?} is the time taken",start.to(end));
	}
}

fn dfs134180(crr_nod: i32, crr_par: i32) {
	unsafe {
		let mut children = vec![];
		visited180[crr_nod as usize] = true;
		if cycle_found180 {
			return ;
		}
		for i in 0..n180 {
			if adj180[crr_nod as usize][i as usize] == 1 && i != crr_par {
				if visited180[i as usize] {
					//println!("i = {}, crr_nod = {}, crr_par = {}", i, crr_nod, crr_par);
					cycle_found180 = true;            
					return;
				} else {

					if cnt180 < NTHREADS180 {
						cnt180 += 1;
						children.push(thread::spawn(move || {              
									dfs134180(i, crr_nod);

									}));
					}
					else {
						dfs134180(i, crr_nod);
					}

				}


			}
		}	
		for child in children {
			let _ = child.join();
		}


	}
}
