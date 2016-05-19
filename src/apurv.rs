use std::io;
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
//extern crate time;
//use time::PreciseTime;

fn input() -> i32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,//println!("your integer input: {}", i),
        Err(..) =>return 0,//println!("this was not an integer: {}", trimmed)
    };
    //return 0
}


pub fn count_ways(mat: &mut [[i32; 350]; 350], n: i32){
	let u: i32 = input();
	let v: i32 = input();
	let k: i32 = input();
	let m: i32 = input();
	let mut count = Arc::new(Mutex::new(vec![[[0;32];32];32]));
	let mut adj: [[i32;32];32] = [[0;32];32];
	for i in 0..n{
		for j in 0..n{
			adj[i as usize][j as usize] = mat[i as usize][j as usize];
		}
	}
	
	//let start = PreciseTime::now();
      for e in 0..k+1 {
   		for i in 0..n {
   			for j in 0..n {
   				let mut count = count.clone();
   				let mut adj = adj.clone();
               thread::spawn(move || {
   			    	let mut count = count.lock().unwrap();
   			    	if e == 0 && i == j {
   						count[i as usize][j as usize][k as usize] = 1;
   					};
   					if e == 1 && adj[i as usize][j as usize] > 0 {
   						count[i as usize][j as usize][e as usize] = 1;
   					};
   					
   					if e > 1 {
   						for a in 0..n {
   							if adj[i as usize][a as usize] > 0 {
   								count[i as usize][j as usize][e as usize] = (count[i as usize][j as usize][e as usize] + count[a as usize][j as usize][(e-1) as usize]) % m;
   							}
   						}
   					}
   			    });
   			}
   		}
   	}
   	thread::sleep(Duration::from_millis(100));
   	let mut count = count.clone();
   	let mut count = count.lock().unwrap();
   	//let end = PreciseTime::now();
    	println!("{}",count[u as usize][v as usize][k as usize]);
}
