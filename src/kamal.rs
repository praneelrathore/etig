use std::*;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{BufRead,BufReader,Read,stdin};
use std::time::{Duration, Instant};
use std::path::Path;

static mut Graph054:[[i32; 500]; 500] = [[0; 500];500];
static mut visited054:[i32; 500]= [0;500];
static mut n054:i32 = 0;
static mut nn054:usize = 0;
static mut DepthOfTree054 :i32 = 0;
static mut LastNodevisited054 :i32=0;

/*fn main() {
	let path = Path::new("new_input3.txt");
	let mut file = BufReader::new(File::open(&path).unwrap());

	let mut input_text = &mut String::new();
	file.read_line(input_text);


	let mut trimmed = input_text.trim();
	let mut m: i32 = trimmed.parse().expect("can't parse number");

	let mut input_text = &mut String::new();
	file.read_line(input_text);

	let mut trimmed = input_text.trim();
	let mut nn054n: i32 = trimmed.parse().expect("can't parse number");
	println!("Number of Nodes : {}", nn054n);
	let mut dummy :[[i32;500];500] = [[0;500];500];

	let mut i:i32=0;
	unsafe{
		n054 = nn054n;
		loop{

			let mut j:i32=0;
			loop{
				let mut input_text = &mut String::new();
				file.read_line(input_text);
				//print!("{} {} ", i, j);

				let mut trimmed = input_text.trim();
				let mut num: i32 = trimmed.parse().expect("can't parse number");
				//v1.push(num);
				dummy[i as usize][j as usize] = num;
				j = j +1;
				if j==n054 {
					break;
				}
			}
			//v.push(v1);
			i = i+1;
			if i==nn054n {
				break;
			}
		}
		
		caller_54(&mut dummy, nn054n);

		
	}
}
*/
pub fn caller_54 (dummy : &mut [[i32; 500]; 500], n : i32) {
		
	unsafe{
			for i in 0..500 {
				for j in 0..500 {
					Graph054[i as usize][j as usize] = dummy[i as usize][j as usize]; 
				}		
			}
			n054 = n;

		visited054[0] = 1;
		Func(0,0);
		for i in 0..n054{
			visited054[i as usize] = 0;
		}
		visited054[LastNodevisited054 as usize] = 1;
		Func(LastNodevisited054, 0);
		println!("Diameter of Given Graph is : {}", DepthOfTree054);
		}
}
	


fn Func(mut CurrentNode: i32, DepthOfNode: i32) {
	let mut dep :i32 = DepthOfNode + 1; 
	unsafe {
		let visited054Arc = Arc::new(Mutex::new(visited054));
		for i in 0..n054 {
			if (Graph054[CurrentNode as usize][i as usize] != 0) {
				if(visited054[i as usize] != 1){
					let visited054Arc = visited054Arc.clone();
					let handle = thread::spawn( move || {
							let mut visited054Arc = visited054Arc.lock().unwrap();
							visited054[i as usize] = 1;
							if(dep > DepthOfTree054){
							DepthOfTree054 = dep;
							LastNodevisited054 = i;
							}
							Func(i, dep);
							});
					handle.join();
				}
			}
		}
	}
}
