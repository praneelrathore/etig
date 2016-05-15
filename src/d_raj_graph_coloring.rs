// Divyanshu Raj IIT2013058
// Shivam kumar	 IIT2013104

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::thread;
use std::time::Duration;

//extern crate time;
//use time::PreciseTime;

static mut it_058_adj_matrix: [[usize; 500]; 500] = [[0; 500]; 500];
static mut it_058_nodes:usize = 0;
static mut it_058_degree: [usize; 500] = [0; 500];
static mut it_058_sort: [usize; 500] = [0; 500];
static mut it_058_color: [usize; 500] = [0; 500];
static mut it_058_u:[usize; 500] = [0; 500];
static mut it_058_d2_matrix: [[usize; 500]; 500] = [[0; 500]; 500];
static mut it_058_forbidden: [[usize; 500]; 500] = [[0; 500]; 500];	// n * (2n - 1)
static mut it_058_cnt: [usize; 500] = [0; 500];

pub fn graphcoloring (mat_110: &mut [[i32; 500]; 500], n: i32) {	
	unsafe{

		for it_058_i in 0..it_058_nodes {
			for it_058_j in 0..it_058_nodes {
				it_058_adj_matrix[it_058_i][it_058_j] = 0;
			}
		}

	  		for i in 0..n {
	      			for j in 0..n {
	      				if (mat_110[i as usize][j as usize] != 0) {
						it_058_adj_matrix[i as usize][j as usize] = 1;
						it_058_adj_matrix[j as usize][i as usize] = 1;
					}
	      			}
	    		}
		
		it_058_nodes = n as usize;
		
		for it_058_i in 0..it_058_nodes {
		
			thread::spawn(move || {
			
				for it_058_j in 0..it_058_nodes {
					if it_058_adj_matrix[it_058_i][it_058_j] == 1 {
						it_058_degree[it_058_i] += 1;
						it_058_u[it_058_i] += 1;
					}
					it_058_u[it_058_i] += 1;
				}
				it_058_sort[it_058_i] = it_058_i;
				it_058_u[it_058_i] += 1;			
			});
		}
		thread::sleep(Duration::from_millis(50));
	
		for it_058_i in 0..it_058_nodes {
		
			let mut it_058_large = 0;
			let mut it_058_index = 0;
		
			for it_058_j in it_058_i..it_058_nodes {
			
				if it_058_degree[it_058_j] > it_058_large {
					it_058_large = it_058_degree[it_058_j];
					it_058_index = it_058_j;
				}
			}

			it_058_large = it_058_sort[it_058_i];
			it_058_sort[it_058_i] = it_058_sort[it_058_index];
			it_058_sort[it_058_index] = it_058_large;
		
			it_058_large = it_058_degree[it_058_i];
			it_058_degree[it_058_i] = it_058_degree[it_058_index];
			it_058_degree[it_058_index] = it_058_large;
		}

		for it_058_j in 0..it_058_nodes {
		
			for it_058_i in 0..it_058_nodes {
			
				thread::spawn(move || {
				
					if it_058_adj_matrix[it_058_i][it_058_j] == 1 {	
	
					for it_058_m in 0..it_058_nodes {
						if it_058_adj_matrix[it_058_j][it_058_m] == 1 && it_058_m != it_058_i && it_058_adj_matrix[it_058_i][it_058_m] != 1 {
							it_058_d2_matrix[it_058_i][it_058_m] = 1;
						}
					}
					}
				});
			}
		}
		thread::sleep(Duration::from_millis(50));

		let mut calculate = 0;
		let mut it_058_lambda = 2;
		//let start = PreciseTime::now();
		let mut it_058_j = 0;
		let mut it_058_flag = 0;
		
		loop {
			for i in 0..it_058_nodes {
				for j in 0..it_058_lambda {
					it_058_forbidden[i][j] = 0;
				}
				it_058_color[i] = 0;
			}
	 		
		loop {
			if it_058_j == it_058_nodes {
				break;
			}
			it_058_flag = 0;		
			let mut it_058_c = 0;
			let mut it_058_x = 0;
		
			for it_058_i in 0..it_058_lambda {
				if it_058_forbidden[it_058_sort[it_058_j]][it_058_i] == 0 {
					if it_058_x == it_058_cnt[it_058_j] {
						it_058_flag = 1;				
						it_058_c = it_058_i;
						break;
					}
					it_058_x = it_058_x + 1;
				}
			}
			it_058_color[it_058_sort[it_058_j]] = it_058_c;
				
		if it_058_flag == 1 {
			calculate += 1;
			for it_058_i in 0..it_058_nodes {
			
				thread::spawn(move || {
				
					if it_058_adj_matrix[it_058_sort[it_058_j]][it_058_i] == 1 {
						it_058_forbidden[it_058_i][it_058_c] = 1;
					}
				});
			
			}
			thread::sleep(Duration::from_millis(50));
			it_058_j = it_058_j + 1;	
		}

		if it_058_flag == 0 {
			calculate += 1;
			if (it_058_j == 0) {
				//println!("sorry! solution not possible.");
				it_058_flag = 2;			
				break;
			}
			it_058_j = it_058_j - 1;
			for it_058_jj in (it_058_j+1)..it_058_nodes {
				it_058_cnt[it_058_jj] = 0;
			}
			
			for it_058_ii in 0..it_058_nodes {
				for it_058_jj in 0..it_058_nodes {
					it_058_forbidden[it_058_ii][it_058_jj] = 0;
				}
			}
		
			for it_058_jj in 0..it_058_j {
				it_058_c = it_058_color[it_058_jj];
			
				for it_058_i in 0..it_058_nodes {
			
					thread::spawn(move || {
					
						if it_058_adj_matrix[it_058_sort[it_058_jj]][it_058_i] == 1 {
							it_058_forbidden[it_058_i][it_058_c] = 1;
						}
						
					});
			
				}
			
			}
			it_058_color[it_058_j] = 0;
			it_058_cnt[it_058_j] = it_058_cnt[it_058_j] + 1;
			thread::sleep(Duration::from_millis(50));
		}	
		}
			thread::sleep(Duration::from_millis(1050));
			for i in 0..it_058_nodes {
				for j in 0..it_058_nodes {
					if it_058_adj_matrix[i][j] == 1 {
						
						if it_058_color[j] == it_058_color[i] {
							 it_058_flag = 2;
						}
					}
				}
			}

			if it_058_flag == 2 {
				it_058_lambda += 1;
			} else {
				break;
			}
	
		}	

		if (it_058_flag != 2) {	
		
		println!("lambda = {}", it_058_lambda);	
		println!("graph coloring");
		for it_058_i in 0..it_058_nodes {
			println!("{} node will have {} color", it_058_i, it_058_color[it_058_i]);
		}	println!("");
		} else {
			println!("lambda = {}", it_058_lambda);
			println!("no solution!!");
		}
		//let end = PreciseTime::now();
		//println!("{}", start.to(end)); 
	}
}




