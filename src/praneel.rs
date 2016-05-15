use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io;
use std::fs::File;
use std::io::{BufRead,BufReader,Read};
use std::env;
//extern crate time;
//use time::PreciseTime;

static mut adj_034:[[i32; 500]; 500] = [[0; 500];500];
static mut n_034:i32 = 0;
static mut order_034:[[i32; 500]; 500] = [[999; 500];500];
static mut irow_034:i32=0;
static mut icol_034:[i32;500]=[0;500];
static mut vis_034:[[i32; 500]; 500] = [[0; 500];500];
static mut printed_034:[i32; 500] = [0; 500];
static mut linecount_034:i32=0;
static mut assigned_034:i32=-1;
static mut children_034:[usize; 500]= [0;500];

fn assignedtask_034(root:usize, idx:usize) {
	unsafe{	
		if assigned_034 as usize == idx {
			return;
		}
		assigned_034+=1;
		vis_034[irow_034 as usize][root as usize]=1;
		vis_034[irow_034 as usize][children_034[assigned_034 as usize] as usize]=1;
		dfs_034(children_034[assigned_034 as usize] as usize, irow_034 as usize);
		irow_034+=1;
		assignedtask_034(root as usize, idx as usize);
	}
}

pub fn dfsutil_034(local_adj_034: &mut [[i32; 500]; 500], local_nodes_034:i32) {
	/*unsafe{
    	n_034=0;
    	irow_034=0;
    	assigned_034=-1;
    	for izero in 0..500 {
    		for jzero in 0..500 {
    			adj_034[izero as usize][jzero as usize]=0;
    			order_034[izero as usize][jzero as usize]=0;
    			vis_034[izero as usize][jzero as usize]=0;
    		}
    	}
    	for izero in 0..500 {
    		icol_034[izero as usize]=0;
    		printed_034[izero as usize]=0;
    	}
    	for izero in 0..500 {
    		children_034[izero as usize]=0;
    	}
	}*/
	unsafe{
		n_034=local_nodes_034;
    	for i in 0..n_034 {
    	for j in 0..n_034 {
    		adj_034[i as usize][j as usize]=local_adj_034[i as usize][j as usize];
    	}
    }	
    }
    println!("ENTER ROOT");
	let root: usize = input();
	    /*unsafe{
		    for i in 0..n_034 {
		      	for j in 0..n_034 {
		        	print!("{} ", adj_034[i as usize][j as usize]);
		      	}	
		      	println!("\n");
		    }

	    }*/
		
	    //let mut root: usize = input();
		//println!("{} is root", root);
		let mut index:i32=0;
		unsafe{printed_034[index as usize]=root as i32;}
		index+=1;
		
		let mut idx:usize=0;
		unsafe {
			for j in 0..n_034 {
				if adj_034[root as usize][j as usize]==1 {
					children_034[idx as usize]=j as usize;
					idx+=1;
				}
			}
		}
		
		unsafe{
			//let start = PreciseTime::now();
			let mut childthread = vec![];
			for tot in 0..4 {
				childthread.push(thread::spawn(move||{assignedtask_034(root as usize, idx as usize)}));
			}
			for child in childthread {
        		let _ = child.join();
    		}
			//let end = PreciseTime::now();			
			for i in 0..idx {
				let mut flag2:i32=0;
				for s in 0..index {
					if printed_034[s as usize]==(children_034[i as usize] as i32) {
						flag2=500;
						break;
					}
				}
				if flag2==0{
					printed_034[index as usize]=children_034[i as usize] as i32;
					index+=1;
					
					for j in 0..500 {
						if order_034[i as usize][j as usize]!=999 {
							let mut flag:i32=0;
							for k in 0..index {
								if order_034[i as usize][j as usize]==printed_034[k as usize] {
									flag=500;
									break;
								}
							}
							if flag!=500 {
								printed_034[index as usize]=order_034[i as usize][j as usize];
								index+=1;
							}
						}
					}
				}
				
			}
			//println!("Depth First Traversal for input graph with given root node as {}", root);
			for i in 0..index {
				print!("{} ", printed_034[i as usize]);
			}
			println!("\n");

			//println!("{} total time taken.",start.to(end));
		}	
	//}
}
    


fn get_test_034<R:Read>(reader:R)->i32 {
	let mut reader = BufReader::new(reader).lines();
	let mut nn:i32=0;
	while let Some(Ok(line)) = reader.next() {
        let node_info= line.to_owned();
        nn = node_info.parse::<i32>().unwrap();
        break;
    }
    return nn;
}

fn dfs_034(cur: usize, r:usize) {
	unsafe {
		for i in 0..n_034 {
			if (adj_034[cur as usize][i as usize] != 0) && (vis_034[r as usize][i as usize] != 1) {
					vis_034[r as usize][i as usize] = 1;
					order_034[r as usize][icol_034[r as usize] as usize]=i;
					icol_034[r as usize]+=1;	
					dfs_034(i as usize,r as usize);
			}
		}
	}
}

fn input() -> usize {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<usize>() {
        Ok(i) => return i,//println!("your integer input: {}", i),
        Err(..) =>return 0,//println!("this was not an integer: {}", trimmed)
    };
}


fn get_node_034<R:Read>(reader:R)->i32 {
	let mut reader = BufReader::new(reader).lines();
	let mut nn:i32=0;
	let mut li:i32=0;

	while let Some(Ok(line)) = reader.next() {
		li+=1;
		unsafe {
			if li>linecount_034 {
				let node_info= line.to_owned();
			    let nodes: Vec<&str> = node_info.split_whitespace().collect();
			    if nodes.len()==1 {
				    let node_info= line.to_owned();
	                nn = node_info.parse::<i32>().unwrap();
	                break;
			    } else {
			    	panic!("Error in reading nodes");
			    }
			}
    	}
    }
    return nn;
}

fn get_graph_034<R:Read>(reader:R) {
	let mut reader = BufReader::new(reader).lines();
	let mut var:i32=0;
	unsafe {
		while let Some(Ok(line)) = reader.next() {
			var+=1;
			if var==linecount_034 {
				break;
			}
		}
	}
	let mut i:i32=0;
	while let Some(Ok(line)) = reader.next() {
        let node_info= line.to_owned();
        let nodes: Vec<&str> = node_info.split_whitespace().collect();
        if nodes.len()!=1 {
        	for j in 0..nodes.len() {
        		let q:i32=nodes[j].parse::<i32>().unwrap();
        		unsafe{adj_034[i as usize][j as usize]=q;}
        	}
        	i+=1;
        } else {
        	break
        }
    }
    unsafe{linecount_034 = linecount_034+i;}
}

/*fn main() {
	let args:Vec<_> = env::args().collect();
    if args.len()!=2{
        panic!("Error with file reading");
    }
  
	
	//t-=1;
	let file = File::open(&args[1]).expect("Error");
	//println!("ENTER THE NUMBER OF NODES");    
	let mut local_nodes_034:i32=0;
	unsafe{
		local_nodes_034=get_node_034(file);
	//	n_034=local_nodes_034;
	}
	unsafe{
		linecount_034+=1;
	}
	unsafe{
	//	println!("{}",n_034);
	}
    let file2 = File::open(&args[1]).expect("Error");
	//println!("ENTER THE ADJACENCY MATRIX");
    get_graph_034(file2);
    let mut local_adj_034:[[i32; 500]; 500] = [[0; 500];500];
    unsafe{
    	for i in 0..n_034 {
    	for j in 0..n_034 {
    		local_adj_034[i as usize][j as usize] = adj_034[i as usize][j as usize];
    	}
    }	
    }
    
		
	dfsutil_034(&mut local_adj_034, local_nodes_034);	
}*/

