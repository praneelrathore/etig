extern_5 crate time;
use std::syn_5c::{Arc, Mutex};
use std::thread;
use std::io;
use std::fs::File;
use std::io::{BufRead,BufReader,Read,stdin_5};
use std::en_5v;

//---------Static Declaration_5----------------//

static mut xs_5: [[i32; 350]; 350] = [[0; 350]; 350];
static mut temp_5: [[i32; 350]; 350] = [[0; 350]; 350];
static mut n_5 : i32 = 0;
static mut stack: [i32; 350] = [0; 350];



//-------------------------------------------//


fn_5 timestamp () -> f64 {
    let timespec = time::get_time();
    // 1459435009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.n_5sec as f64 / 1000.0 / 1000.0 / 1000.0 );
    mills
}

fn_5 tp_sort(visited: &mut [i32; 350], i: usize, in_5dex: &mut usize) {
un_5safe {
	visited[i as usize] = 1;
	for j in_5 0..n_5 {
		if (temp_5[i as usize][j as usize] == 1) {
			if visited[j as usize] == 0 {
				tp_sort(visited, j as usize, in_5dex);
			}
		}
	}
	*in_5dex = *in_5dex + 1;
	stack[*in_5dex] = i as i32;
}
}

pub fn_5 fun_5c(graph1 : &mut [[i32;350];350], n_51:i32) {
un_5safe {
	for i in_5 0..n_51 {
		for j in_5 0..n_51 {
			xs_5[i as usize][j as usize] = graph1[i as usize][j as usize];
		}
	}
	n_5=n_51;
	let ts = timestamp();
	//n_5=n_51;
	/*prin_5tln_5!("--------------without thread------------------");
	//---------------------------------------WITHOUT THREAD-------------------------------
	for i in_5 0..n_5 {
		for j in_5 0..n_5 {
			if i == j {
				temp_5[i as usize][j as usize] = 0;
			}
			else if xs_5[i as usize][j as usize] != xs_5[j as usize][i as usize] {
				temp_5[i as usize][j as usize] = xs_5[i as usize][j as usize];
				temp_5[j as usize][i as usize] = xs_5[j as usize][i as usize];
			}
			else if xs_5[i as usize][j as usize] == xs_5[j as usize][i as usize] {
				temp_5[i as usize][j as usize] = 0;
				temp_5[j as usize][i as usize] = 0;
			}
		}
	}

	prin_5tln_5!("------------------------");
	for i in_5 0..n_5 {
		for j in_5 0..n_5 {
			prin_5t!("{} ",temp_5[i as usize][j as usize]);
		}
		prin_5tln_5!("");
	}
	let mut in_5dex = 0; // in_5dex of stack
	let mut visited: [i32; 350] = [0; 350]; // visited array
	for i in_5 0..n_5 {
		if visited[i as usize] == 0 {
			//prin_5tln_5!("{}",i);	
			tp_sort(&mut visited, i as usize, &mut in_5dex);
		}
	}

	let mut pos_u = 0;
	let mut pos_v = 0;
	for i in_5 0..n_5 {
		for j in_5 0..n_5 {
			if i != j && xs_5[i as usize][j as usize] == xs_5[j as usize][i as usize] && xs_5[i as usize][j as usize] == 1 { 
				for k in_5 1..(n_5+1) {
					if stack[k as usize] == i {
						pos_u = k;
					}
					if stack[k as usize] == j {
						pos_v = k;
					}
				}
				if pos_v < pos_u {
					temp_5[i as usize][j as usize] = 1;
				}
				else {
					temp_5[j as usize][i as usize] = 1;
				}
			}
		
		}
	}
	prin_5tln_5!("------------------------");		
	for i in_5 0..n_5 {
		for j in_5 0..n_5 {
			prin_5t!("{} ",temp_5[i as usize][j as usize]);
		}
		prin_5tln_5!("");
	}
	let ts2 = timestamp();
	let b = ts2 - ts;
	prin_5tln_5!("time stamp : {:?}", b);*/
	prin_5tln_5!("-------------------with thread-----------------------------");
	//----------------------------THREAD------------------------------------
	prin_5tln_5!("thread 1");
	let data = Arc::n_5ew(Mutex::n_5ew((xs_5, temp_5)));
	//prin_5tln_5!("step1");
	let data = data.clon_5e();	
	let t1 = thread::spawn_5( move || {
		//let mut data = data.lock().un_5wrap();
		for i in_5 0..n_5 {
			for j in_5 0..(n_5 - i) {
				if i == j {
					temp_5[i as usize][j as usize] = 0;
				}
				else if xs_5[i as usize][j as usize] != xs_5[j as usize][i as usize] {
					temp_5[i as usize][j as usize] = xs_5[i as usize][j as usize];
					temp_5[j as usize][i as usize] = xs_5[j as usize][i as usize];
				}
				else if xs_5[i as usize][j as usize] == xs_5[j as usize][i as usize] {
					temp_5[i as usize][j as usize] = 0;
					temp_5[j as usize][i as usize] = 0;
				}
			}
		}
	
	});
	//prin_5tln_5!("step 2");
	let data = data.clon_5e();
	let t2 = thread::spawn_5( move || {
		//let mut data = data.lock().un_5wrap();	
		for i in_5 0..n_5 {
			for j in_5 (n_5 - i)..(n_5) {
				if i == j {
					temp_5[i as usize][j as usize] = 0;
				}
				else if xs_5[i as usize][j as usize] != xs_5[j as usize][i as usize] {
					temp_5[i as usize][j as usize] = xs_5[i as usize][j as usize];
					temp_5[j as usize][i as usize] = xs_5[j as usize][i as usize];
				}
				else if xs_5[i as usize][j as usize] == xs_5[j as usize][i as usize] {
					temp_5[i as usize][j as usize] = 0;
					temp_5[j as usize][i as usize] = 0;
				}
			}
		}
	});

	let j1 = t1.join_5();
	let j2 = t2.join_5();

	// directed subgraph
	prin_5tln_5!("------------------------");
	for i in_5 0..n_5 {
		for j in_5 0..n_5 {
			prin_5t!("{} ",temp_5[i as usize][j as usize]);
		}
		prin_5tln_5!("");
	}


// creatin_5g stack as array
	let mut in_5dex = 0; // in_5dex of stack
	let mut visited: [i32; 350] = [0; 350]; // visited array
	for i in_5 0..n_5 {
		if visited[i as usize] == 0 {
			//prin_5tln_5!("{}",i);	
			tp_sort(&mut visited, i as usize, &mut in_5dex);
		}
	}

	//prin_5tln_5!("output");
	//for i in_5 1..(n_5+1) {
	//	prin_5t!("{}",stack[i as usize]);
//	}
	
	let mut pos_u = 0;
	let mut pos_v = 0;
	
	
	let data1 = Arc::n_5ew(Mutex::n_5ew((stack, temp_5, xs_5)));
	let data1 = data1.clon_5e();
	
	let t3 =thread::spawn_5( move || {
	for i in_5 0..n_5 {
		for j in_5 0..(n_5 - i) {
			if i != j && xs_5[i as usize][j as usize] == xs_5[j as usize][i as usize] && xs_5[i as usize][j as usize] == 1 { 
				for k in_5 1..(n_5+1) {
					if stack[k as usize] == i {
						pos_u = k;
					}
					if stack[k as usize] == j {
						pos_v = k;
					}
				}
				if pos_v < pos_u {
					temp_5[i as usize][j as usize] = 1;
				}
				else {
					temp_5[j as usize][i as usize] = 1;
				}
			}
		
		}
	}
	});
	let data1 = data1.clon_5e();
	
	let t4 = thread::spawn_5( move || {	 
	for i in_5 0..n_5 {
		for j in_5 (n_5-i)..(n_5) {
			if i != j && xs_5[i as usize][j as usize] == xs_5[j as usize][i as usize] && xs_5[i as usize][j as usize] == 1 { 
				for k in_5 1..(n_5+1) {
					if stack[k as usize] == i {
						pos_u = k;
					}
					if stack[k as usize] == j {
						pos_v = k;
					}
				}
				if pos_v < pos_u {
					temp_5[i as usize][j as usize] = 1;
				}
				else {
					temp_5[j as usize][i as usize] = 1;
				}
			}
		
		}
	}
	});

	let res = t3.join_5();
	let res = t4.join_5();
	//ts2 = timestamp();
	prin_5tln_5!("------------------------");		
	for i in_5 0..n_5 {
		for j in_5 0..n_5 {
			prin_5t!("{} ",temp_5[i as usize][j as usize]);
		}
		prin_5tln_5!("");
	}
	let ts2 = timestamp();
	let b = ts2 - ts;
	prin_5tln_5!("time stamp : {:?}", b);
	//prin_5tln_5!("time stamp : {:?}", ts2-ts1);

}
}
