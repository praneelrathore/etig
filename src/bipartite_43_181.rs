use std::io;
use std::thread;

//extern crate time;
//use time::PreciseTime;

/*fn input() -> i16 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i16>() {
        Ok(i) => return i,		//println!("your integer input: {}", i),
        Err(..) =>return -1i16,	//println!("this was not an integer: {}", trimmed)
    };
}*/


static MAX_THREADS43: i32 = 10;
static mut thr_count43: i32 = 1;
static col_red43: i32 = 1;
static col_black43: i32 = 0;
static mut n43: i32 = 0;
static mut adj43: [[bool; 500]; 500] = [[false; 500]; 500];
static mut bipartite43: bool = true;
static mut color43: [i32; 500] = [-1i32; 500];


/*fn main() {
	unsafe {
    	println!("ENTER THE NUMBER OF NODES");    
    	n = input();

    	for i in 0..n {
		for j in 0..n {
    			let x: i16 = input();

			if (x != 0) {
      				adj[i as usize][j as usize] = true;
	      			adj[j as usize][i as usize] = true;
			}
		}
    	}

	let start = PreciseTime::now();
    	for i in 0..n {
            if bipartite == false {
                break;
            }
    		if color[i as usize] == -1i16 {
    			color[i as usize] = col_red;
    			dfs(i);
    		}
		}
			
    	
	let end = PreciseTime::now();
	println!("{:?}", start.to(end));

    	if (bipartite) {
    		println!("Bipartite graph");
    	} else {
    		println!("Non-Bipartite graph");
    	}
    }
}*/

pub fn check_bipartite(matrix: &mut [[i32; 500]; 500], n_tmp: i32) -> () {
	unsafe {
	n43 = n_tmp;
    	for i in 0..n43 {
		  for j in 0..n43 {
    			let x43: i32 = matrix[i as usize][j as usize];

			     if (x43 != 0) {
      				adj43[i as usize][j as usize] = true;
	      			adj43[j as usize][i as usize] = true;
			     }
		  }
    	}

//	let start = PreciseTime::now();
    	for i in 0..n43 {
            if bipartite43 == false {
                break;
            }
    		if color43[i as usize] == -1i32 {
    			color43[i as usize] = col_red43;
    			dfs43(i);
    		}
	}
			
    	
//	let end = PreciseTime::now();
//	println!("{:?}", start.to(end));

    	if bipartite43 {
    		println!("Bipartite graph");
    	} else {
    		println!("Non-Bipartite graph");
    	}
    }
}

fn dfs43(node: i32) {
	unsafe {
		if bipartite43 == false {
                	return;
	        }
		let mut children = vec![];

		for i in 0..n43 {
            if bipartite43 == false {
                return;
            }

			if adj43[node as usize][i as usize] == true {
				if color43[i as usize] == -1i32 {
					color43[i as usize] = 1 - color43[node as usize];

                    if (thr_count43 < MAX_THREADS43) {
                        thr_count43 += 1;
    					children.push(thread::spawn(move || {
		    				dfs43(i);
			    		}));
                    } else {
                        dfs43(i);
                    }
				} else if color43[i as usize] == color43[node as usize] {
					bipartite43 = false;					
					return;
				}
			}
		}

		for child in children {
			child.join();
		}
	}
}
