extern crate time;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io;
use std::fs::File;
use std::io::{BufRead,BufReader,Read,stdin};
use std::env;

//---------Static Declaration----------------//

static mut xs: [[i32; 500]; 500] = [[0; 500]; 500];
static mut temp: [[i32; 500]; 500] = [[0; 500]; 500];
static mut n : i32 = 0;
static mut stack: [i32; 500] = [0; 500];



//-------------------------------------------//


fn timestamp () -> f64 {
    let timespec = time::get_time();
    // 1459440009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0 );
    mills
}

fn tp_sort(visited: &mut [i32; 500], i: usize, index: &mut usize) {
unsafe {
	visited[i as usize] = 1;
	for j in 0..n {
		if (temp[i as usize][j as usize] == 1) {
			if visited[j as usize] == 0 {
				tp_sort(visited, j as usize, index);
			}
		}
	}
	*index = *index + 1;
	stack[*index] = i as i32;
}
}

pub fn func(graph1 : &mut [[i32;500];500], n1:i32) {
unsafe {
	for i in 0..n1 {
		for j in 0..n1 {
			xs[i as usize][j as usize] = graph1[i as usize][j as usize];
		}
	}
	n=n1;
	let ts = timestamp();
	//n=n1;
	/*println!("--------------without thread------------------");
	//---------------------------------------WITHOUT THREAD-------------------------------
	for i in 0..n {
		for j in 0..n {
			if i == j {
				temp[i as usize][j as usize] = 0;
			}
			else if xs[i as usize][j as usize] != xs[j as usize][i as usize] {
				temp[i as usize][j as usize] = xs[i as usize][j as usize];
				temp[j as usize][i as usize] = xs[j as usize][i as usize];
			}
			else if xs[i as usize][j as usize] == xs[j as usize][i as usize] {
				temp[i as usize][j as usize] = 0;
				temp[j as usize][i as usize] = 0;
			}
		}
	}

	println!("------------------------");
	for i in 0..n {
		for j in 0..n {
			print!("{} ",temp[i as usize][j as usize]);
		}
		println!("");
	}
	let mut index = 0; // index of stack
	let mut visited: [i32; 500] = [0; 500]; // visited array
	for i in 0..n {
		if visited[i as usize] == 0 {
			//println!("{}",i);	
			tp_sort(&mut visited, i as usize, &mut index);
		}
	}

	let mut pos_u = 0;
	let mut pos_v = 0;
	for i in 0..n {
		for j in 0..n {
			if i != j && xs[i as usize][j as usize] == xs[j as usize][i as usize] && xs[i as usize][j as usize] == 1 { 
				for k in 1..(n+1) {
					if stack[k as usize] == i {
						pos_u = k;
					}
					if stack[k as usize] == j {
						pos_v = k;
					}
				}
				if pos_v < pos_u {
					temp[i as usize][j as usize] = 1;
				}
				else {
					temp[j as usize][i as usize] = 1;
				}
			}
		
		}
	}
	println!("------------------------");		
	for i in 0..n {
		for j in 0..n {
			print!("{} ",temp[i as usize][j as usize]);
		}
		println!("");
	}
	let ts2 = timestamp();
	let b = ts2 - ts;
	println!("time stamp : {:?}", b);*/
	println!("-------------------with thread-----------------------------");
	//----------------------------THREAD------------------------------------
	println!("thread 1");
	let data = Arc::new(Mutex::new((xs, temp)));
	
	let data = data.clone();	
	let t1 = thread::spawn( move || {
		//let mut data = data.lock().unwrap();
		for i in 0..n {
			for j in 0..(n - i) {
				if i == j {
					temp[i as usize][j as usize] = 0;
				}
				else if xs[i as usize][j as usize] != xs[j as usize][i as usize] {
					temp[i as usize][j as usize] = xs[i as usize][j as usize];
					temp[j as usize][i as usize] = xs[j as usize][i as usize];
				}
				else if xs[i as usize][j as usize] == xs[j as usize][i as usize] {
					temp[i as usize][j as usize] = 0;
					temp[j as usize][i as usize] = 0;
				}
			}
		}
	
	});

	let data = data.clone();
	let t2 = thread::spawn( move || {
		//let mut data = data.lock().unwrap();	
		for i in 0..n {
			for j in (n - i)..(n) {
				if i == j {
					temp[i as usize][j as usize] = 0;
				}
				else if xs[i as usize][j as usize] != xs[j as usize][i as usize] {
					temp[i as usize][j as usize] = xs[i as usize][j as usize];
					temp[j as usize][i as usize] = xs[j as usize][i as usize];
				}
				else if xs[i as usize][j as usize] == xs[j as usize][i as usize] {
					temp[i as usize][j as usize] = 0;
					temp[j as usize][i as usize] = 0;
				}
			}
		}
	});

	let j1 = t1.join();
	let j2 = t2.join();

	// directed subgraph
	println!("------------------------");
	for i in 0..n {
		for j in 0..n {
			print!("{} ",temp[i as usize][j as usize]);
		}
		println!("");
	}


// creating stack as array
	let mut index = 0; // index of stack
	let mut visited: [i32; 500] = [0; 500]; // visited array
	for i in 0..n {
		if visited[i as usize] == 0 {
			//println!("{}",i);	
			tp_sort(&mut visited, i as usize, &mut index);
		}
	}

	//println!("output");
	//for i in 1..(n+1) {
	//	print!("{}",stack[i as usize]);
//	}
	
	let mut pos_u = 0;
	let mut pos_v = 0;
	
	
	let data1 = Arc::new(Mutex::new((stack, temp, xs)));
	let data1 = data1.clone();
	
	let t3 =thread::spawn( move || {
	for i in 0..n {
		for j in 0..(n - i) {
			if i != j && xs[i as usize][j as usize] == xs[j as usize][i as usize] && xs[i as usize][j as usize] == 1 { 
				for k in 1..(n+1) {
					if stack[k as usize] == i {
						pos_u = k;
					}
					if stack[k as usize] == j {
						pos_v = k;
					}
				}
				if pos_v < pos_u {
					temp[i as usize][j as usize] = 1;
				}
				else {
					temp[j as usize][i as usize] = 1;
				}
			}
		
		}
	}
	});
	let data1 = data1.clone();
	
	let t4 = thread::spawn( move || {	 
	for i in 0..n {
		for j in (n-i)..(n) {
			if i != j && xs[i as usize][j as usize] == xs[j as usize][i as usize] && xs[i as usize][j as usize] == 1 { 
				for k in 1..(n+1) {
					if stack[k as usize] == i {
						pos_u = k;
					}
					if stack[k as usize] == j {
						pos_v = k;
					}
				}
				if pos_v < pos_u {
					temp[i as usize][j as usize] = 1;
				}
				else {
					temp[j as usize][i as usize] = 1;
				}
			}
		
		}
	}
	});

	let res = t3.join();
	let res = t4.join();
	//ts2 = timestamp();
	println!("------------------------");		
	for i in 0..n {
		for j in 0..n {
			print!("{} ",temp[i as usize][j as usize]);
		}
		println!("");
	}
	let ts2 = timestamp();
	let b = ts2 - ts;
	println!("time stamp : {:?}", b);
	//println!("time stamp : {:?}", ts2-ts1);

}
}
