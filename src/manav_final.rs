use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io;
use std::fs::File;
use std::io::{BufRead,BufReader,Read,stdin};
use std::env;
//------------------------------static declaration-----------//

static mut v_173 : [[i32; 350]; 350] = [[0; 350]; 350];
static mut tc_173 : [[i32; 350]; 350] = [[0; 350]; 350];
static mut n_173 : i32 = 0;

//-----------------------------------------------------------//

fn input_173() -> i32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,//println!("your integer input_173: {}", i),
        Err(..) =>return 0,//println!("this was not an integer: {}", trimmed)
    };
    return 0
}

fn get_node_173<R:Read>(reader:R)->i32 {
	let mut reader = BufReader::new(reader).lines();
	let mut nn:i32=0;
	while let Some(Ok(line)) = reader.next() {
        let node_info= line.to_owned();
        nn = node_info.parse::<i32>().unwrap();
        break;
    }
    return nn;
}

fn get_graph_173<R:Read>(reader:R)->i32 {
	let mut reader = BufReader::new(reader).lines();
	let mut nn:i32=0;
	let mut k:i32=0;
	let mut row: Vec<i32>;
	let mut i:usize=0;
	while let Some(Ok(line)) = reader.next() {
        let node_info= line.to_owned();
        let nodes: Vec<&str> = node_info.split_whitespace().collect();
        if nodes.len()!=1 {
        	for j in 0..nodes.len() {
        		let mut q:i32=nodes[j].parse::<i32>().unwrap();
        		unsafe{v_173[i as usize][j as usize]=q;}
        	}
        	i+=1;
        }
    }
    return k;
}
fn dfs_173(s : i32, d : i32) -> () {
	unsafe {
		tc_173[s as usize][d as usize] = 1;
		for i in 0..n_173 {
			if v_173[d as usize][i as usize] == 1 && tc_173[s as usize][i as usize]==0 {
				dfs_173(s,i);
			}
		}
	}
}
pub fn solve_173( dummy: &mut [[i32; 350]; 350], n:i32 ) {
	unsafe {
		/*
		
		//println!("ENTER THE NUMBER OF NODES");
		let file = File::open("panna.txt").expect("Error");
		n_173=get_node_173(file);
		//println!("ENTER THE ADJACENCY MATRIX");
		let file2 = File::open("panna.txt").expect("Error");
		let mut k:i32;
		k=get_graph_173(file2);
		//for i in 0..n_173 {
		//	for j in 0..n_173 {
		//		print!("{} ", v_173[i as usize][j as usize]);
		//	}
		//	println!("");
		//}
		
		*/
		
		// copying dummy graph in the global graph of the code 
		
		for i in 0..350 {
			for j in 0..350 {
				v_173[i][j] = dummy[i][j];
			}
		}
		
		n_173 = n;
		
		let data = Arc::new(Mutex::new((v_173, tc_173, n_173)));
		//println!("Number of Threads ");
		let mut pro = 3;
		if n_173 < pro {
			pro = n_173;
		}

		let mut t = vec![];
		let mut st = 0;
		let mut en = n_173/pro;
		let mut dif = en - st;
		for j in 0..pro {
			let data = data.clone();
			t.push(thread::spawn( move || {
				let mut data = data.lock().unwrap();
				for i in st..en {
					dfs_173(i,i);
				}
			}));
			st = en;
			en = en + dif;
			if (en > n_173) {
				en = n_173;
			}
			if(j==pro-2) {
				en=n_173;
			}
		}
		for k in t {
					let res = k.join();
		}

		for i in 0..n_173 {
			for j in 0..n_173 {
				print!("{} ",tc_173[i as usize][j as usize]);
			}
			println!("");
		}



		//let mut tmp: i32 = input_173();
	}
}
/*fn main() {
	
	let mut dummy :[[i32; 350]; 350];
	let mut n :i32;
	solve_173(&mut dummy, n);
}*/
