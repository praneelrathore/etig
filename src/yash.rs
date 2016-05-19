use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead,BufReader,Read,stdin};
use std::env;

//extern crate time;
//use time::PreciseTime;


static mut ans_105 : [i32; 350] = [0;350];
static mut parent_105 : [i32; 350] = [-1; 350];
static mut graph_105 : [[i32; 350]; 350]=[[0; 350]; 350];
static mut findset_105 : [[i32; 2]; 350] = [[0; 2]; 350];

static mut s1_105 : i32 = 0;
static mut s2_105 : i32 = 0;
static mut p1_105 : i32 = 0;
static mut p2_105 : i32 = 0;

fn get_input_105 () -> (i32, i32) {
	unsafe {
		
		let mut stri = String::new();
		io::stdin().read_line(&mut stri).expect("error!");
		let e: i32 = stri.trim().parse().expect("error!");
		

		let mut stri = String::new();
		io::stdin().read_line(&mut stri).expect("error!");
		let pr: i32 = stri.trim().parse().expect("error!");
		

		for i in 0..e {
			let mut strin = String::new();
			io::stdin().read_line(&mut strin).expect("error!");
			let mut num : i32 = strin.trim().parse().expect("error!");			
			findset_105[i as usize][0] = num;
				
			let mut strin2 = String::new();
			io::stdin().read_line(&mut strin2).expect("error!");
			let mut num1 : i32 = strin2.trim().parse().expect("error!");			
			findset_105[i as usize][1] = num1;		
		}

		return (e, pr);
	}
}


fn find_parent_105(x: i32) -> i32 {
	unsafe {
    	if parent_105[x as usize] == -1 {
        	return x;
    	}
    	return find_parent_105(parent_105[x as usize]);
    }
}

fn checksame_ser_105 (n1 :i32, n2 : i32) -> i32 {
	unsafe {	
		p1_105 = find_parent_105(n1);
		p2_105 = find_parent_105(n2);
	
		if p1_105 == p2_105 {
			return 1;
		} else {
			return 0;
		}
	}
}

fn checksame_105(n1 :i32, n2 : i32) -> i32 {
	unsafe {
		let data = Arc::new(parent_105);
		s1_105 = n1;
		s2_105 = n2;
	
	
		let data = data.clone();
		thread::spawn( move || {
			unsafe{
				p1_105= find_parent_105(s1_105);
			}
		});
	
		let data = data.clone();
		thread::spawn( move || {
			unsafe {
				p2_105 = find_parent_105(s2_105);
			}
		});
	
		if p1_105 == p2_105 {
			return 1;
		} else {
			return 0;
		}
	}
}

fn get_pair_105<R:Read>(reader:R)->(i32,i32) {
	unsafe{	
		let mut reader = BufReader::new(reader).lines();
		let mut flag:i32 = 0;
		let mut i:i32 = 0;
		let mut nn:i32=0;
		let mut pro : i32 = 0;
		while let Some(Ok(line)) = reader.next() {
			if flag == 0 {        	
				let node_info = line.to_owned();
		        	nn = node_info.parse::<i32>().unwrap();
				flag = 1;	        
			} else if flag == 2{
				let node_info = line.to_owned();
				let nodes: Vec<&str> = node_info.split_whitespace().collect();
					findset_105[i as usize][0] = nodes[0].parse::<i32>().unwrap();	
					findset_105[i as usize][1] = nodes[1].parse::<i32>().unwrap();
					i+= 1;		
			} else if flag == 1{
				let node_info = line.to_owned();
		        	pro = node_info.parse::<i32>().unwrap();
				flag = 2;
			}
	    }
	    return (nn, pro);
	}
}
fn get_node_105<R:Read>(reader:R)->i32 {
	unsafe {
		let mut reader = BufReader::new(reader).lines();
		let mut flag:i32 = 0;
		let mut nn:i32=0;
		let mut counter : i32 = 0;
		//println!("pr");
		while let Some(Ok(line)) = reader.next() {
			if flag == 0 {        	
				let node_info = line.to_owned();
			    	nn = node_info.parse::<i32>().unwrap();
				flag = 1;
		//		println!("pr1");	        
			} else {
				let node_info = line.to_owned();
				let nodes: Vec<&str> = node_info.split_whitespace().collect();
				for i in 0..nn {
					unsafe {
						graph_105[counter as usize][i as usize] = nodes[i as usize].parse::<i32>().unwrap();		
						if graph_105[counter as usize][i as usize] != 0 {
							graph_105[counter as usize][i as usize] = 1;
						}
					}		
				}
		//		println!("pr3");
				counter += 1;		
			}
	    }
	    return nn;
	}
}

fn union_serial_105 (x: i32, y: i32) {
	unsafe {	
		let par1 = find_parent_105(x);
		let par2 = find_parent_105(y);
		if(par1 != par2) {
			parent_105[par1 as usize] = par2;	
		}
	}
}

fn union_105 (x: i32, y: i32) {
	    unsafe {
	
		let data = Arc::new(parent_105);	//Atomic Reference Counter : 
		
		s1_105 = x;
		s2_105 = y;
		
		let data = data.clone();
			let t1 = thread::spawn( move || {
		    p1_105 = find_parent_105(s1_105);
	    });
	    
	    let data = data.clone();	//cloned data for reference counting 
			let t2 = thread::spawn( move || {
		    p2_105 = find_parent_105(s2_105);
	    });
	    
	    let res1 = t1.join();
	    let res2 = t2.join();
	    
	    if p1_105 != p2_105 {
	        parent_105[p1_105 as usize] = p2_105;
		}
 	}
}

pub fn caller_105 (dummy : &mut [[i32; 350]; 350], n : i32) {
	unsafe {
				
			
			for i in 0..350 {
				for j in 0..350 {
					graph_105[i as usize][j as usize] = dummy[i as usize][j as usize]; 
				}		
			}
			
			
			for i in 0..n {
				for j in 0..n {
					print!("{}\t", graph_105[i as usize][j as usize]);			//UNSAFE using graph
				}
				print!("\n");
			}
			
			let (e, pro) = get_input_105();

			for i in 0..n {
				for j in i..n {
					if graph_105[i as usize][j as usize] == 1 {
						union_serial_105(i, j);			
					}	
				}	
			}
		
			let data = Arc::new(parent_105);
			let mut st = 0;
			let mut en = e/pro;
			let mut dif = en - st;
			let mut v = vec![];
              //          let start = PreciseTime::now();
			for i in 0..pro {
				let data = data.clone();
				v.push(thread::spawn(move ||{
					for j in st..en {
						ans_105[j as usize] = checksame_ser_105(findset_105[j as usize][0 as usize], findset_105[j as usize][1 as usize]);		
					}
				}));
				st = en;
				en = en + dif;
			}
			for i in v {
				let res = i.join();	
			}
			//let end = PreciseTime::now();
			//println!("{} ",start.to(end));
			for i in 0..n {
		      	print!("{}\t", parent_105[i as usize]);							//UNSAFE using parent
				
			}
			println!("");
			for i in 0..e {
		      	print!("{}\t", ans_105[i as usize]);							//UNSAFE using parent
				
			}

	}
}

/*fn main() {
	unsafe {
		let mut dummy :[[i32;350];350] = [[0;350];350];
		let mut n:i32 = 0;
	    	caller_105(dummy, n);
	}
}*/
