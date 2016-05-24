// Divyanshu Raj IIT2013058
// Shivam kumar	 IIT2013104

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::io;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::process::Command;
use std::str::FromStr;
use std::io::{BufReader,BufRead,Read,stdin};
use std::sync::mpsc;
use std::collections::VecDeque;
use std::sync::
{
    Arc, Mutex
};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
extern crate time;


//extern crate libc;

/*extern {
    fn double_input(input: libc::c_int) -> libc::c_int;
}*/

static NTHREADS: usize = 4;

#[derive(Copy, Clone, Eq, PartialEq)]
struct QueNode {
    cost: u32,
    vertex: u32,
}

impl Ord for QueNode {
    fn cmp(&self, other: &QueNode) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for QueNode {
    fn partial_cmp(&self, other: &QueNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const  INF: u32 = 999999999;
static mut output : [[u32; 350]; 350] = [[0; 350]; 350];



static mut dist_110: [i32; 350] = [9999999; 350];
static mut source_110: [i32; 350] = [0; 350];
static mut dest_110: [i32; 350] = [0; 350];
static mut weight_110: [i32; 350] = [0; 350];
static mut adj_110: [[i32; 350]; 350] = [[0; 350]; 350];

static mut dist_207:[[i32; 350]; 350] = [[0; 350]; 350];
static mut adj_034:[[i32; 350]; 350] = [[0; 350];350];
static mut n_034:i32 = 0;
static mut order_034:[[i32; 350]; 350] = [[999; 350];350];
static mut irow_034:i32=0;
static mut icol_034:[i32;350]=[0;350];
static mut vis_034:[[i32; 350]; 350] = [[0; 350];350];
static mut printed_034:[i32; 350] = [0; 350];
static mut linecount_034:i32=0;
static mut assigned_034:i32=-1;
static mut children_034:[usize; 350]= [0;350];
static mut graph_146 : [[i32; 350]; 350] = [[0; 350]; 350];
static mut level_146:[i32; 350] = [-1;350];
static mut visited_146:[i32; 350] = [0; 350];
static mut mat_146 : [[i32; 350]; 350] = [[0;350];350];//matrix
static mut levsize_146:[i32; 350] = [-1; 350];
static mut maxlevel_146 :i32 = 0;
static mut queue_146 : [i32; 350] = [0; 350];
static mut n_146 : i32 = 0;
static mut front_146 : i32 = 0;
static mut rear_146 : i32 = 0;
static mut no_of_processors_146 :i32 = 4;
static mut ins :i32 = 0;

static mut dist1_062:[[i32; 350]; 350] = [[0; 350];350]; 
static mut v_062:[[i32; 350]; 350] = [[0; 350];350]; 
static mut tmp_062:[i32;350]=[0;350]; 
static mut n_062:i32 = 0; 

static mut ans_105 : [i32; 350] = [0;350];
static mut parent_105 : [i32; 350] = [-1; 350];
static mut graph_105 : [[i32; 350]; 350]=[[0; 350]; 350];
static mut findset_105 : [[i32; 2]; 350] = [[0; 2]; 350];

static mut s1_105 : i32 = 0;
static mut s2_105 : i32 = 0;
static mut p1_105 : i32 = 0;
static mut p2_105 : i32 = 0;

static mut Graph054:[[i32; 350]; 350] = [[0; 350];350];
static mut visited054:[i32; 350]= [0;350];
static mut n054:i32 = 0;
static mut nn054:usize = 0;
static mut DepthOfTree054 :i32 = 0;
static mut LastNodevisited054 :i32=0;

static mut NTHREADS180 : i32 = 10;
static mut cnt180: i32 = 1;
static mut n180: i32 = 0;
static mut adj180: [[i32; 350]; 350] = [[0 ; 350]; 350];
static mut cycle_found180: bool = false;
static mut visited180: [bool; 350] = [false; 350];
//static mut queue:VecDeque<Thread> = VecDeque::new();

static mut v_173 : [[i32; 350]; 350] = [[0; 350]; 350];
static mut tc_173 : [[i32; 350]; 350] = [[0; 350]; 350];
static mut n_173 : i32 = 0;

//-----------------------------------------------------------//
static MAX_THREADS43: i32 = 10;
static mut thr_count43: i32 = 1;
static col_red43: i32 = 1;
static col_black43: i32 = 0;
static mut n43: i32 = 0;
static mut adj43: [[bool; 350]; 350] = [[false; 350]; 350];
static mut bipartite43: bool = true;
static mut color43: [i32; 350] = [-1i32; 350];
static mut n_second: i32 = 0;
static mut MAX_THREADS_second: i32 = 10;
static mut thr_count_second: i32 = 1;
static mut adj_second: [[bool; 350]; 350] = [[false; 350]; 350];
static mut cycle_found_second: bool = false;
static mut visited_second: [bool; 350] = [false; 350];
static mut visiting_second: [bool; 350] = [false; 350];

static NTHREADS_TONY: usize = 4;
static mut CYCLE_tonny: bool = false;
static mut row_del: usize = 0;



fn is_tree( mut adj_list: &mut Vec< Vec<usize> >,mut visited: &mut Vec<bool>,nodes:usize) {
    unsafe {
        let mut flag = false;
        let mut row: usize = nodes;

        while flag != true && row_del < nodes - 1 {

            flag = true;
            for index in 0..row {
                if visited[index] == false {
                    flag = tree(&mut adj_list, &mut visited,index,flag,nodes);
                }
            }

            if flag == true && row_del < nodes - 1{
                CYCLE_tonny = true;
            }
        }
    }

}

fn tree(mut adj_list: &mut Vec< Vec<usize>>,mut visited: &mut Vec<bool>,index:usize,flag:bool,nodes: usize)->bool{

    unsafe {
        let col = nodes;
        let row = nodes;
        let mut sum = 0;
        let mut flag = flag;

        for i in 0..col {
            sum = sum + adj_list[index][i];
        }

        if sum == 0 {
            for j in 0..row {
                adj_list[j][index] = 0;
            }
            flag = false;
            row_del = row_del + 1;
            visited[index] = true;
        }
        return flag;
    }

}

pub fn check_tree(mut adj_list_c: &mut [[i32;350];350],nodes_r: i32) {

    let nodes = nodes_r as usize;
    let mut adj_list: Vec< Vec< usize> > = vec![vec![0;nodes];nodes];
    for i in 0..nodes{
        for j in 0..nodes {
                adj_list[i][j] = adj_list_c[i][j] as usize;
        }
    }

    let mut child_threads = Vec::new();
    let mut adj_list = Arc::new(Mutex::new(adj_list));

    let mut loop_start = 0;

    unsafe {
        let visited = Arc::new(Mutex::new(vec![false;nodes]));
        while loop_start <= nodes && loop_start <= (NTHREADS_TONY) {

            let visited = visited.clone();
            let adj_list= adj_list.clone();
            child_threads.push(
                thread::spawn(move || {
                    let mut visited = visited.lock().unwrap();
                    let mut adj_list = (adj_list.lock().unwrap());
                    is_tree(&mut adj_list,&mut visited,nodes);

                })
            );

            loop_start = loop_start + 1;
        }

    }
    for child in child_threads {
        let _ = child.join();
    }


    unsafe {
        if CYCLE_tonny{
            println!("Output        :   Not a tree");
        }else {
            println!("Output        :   Tree");
        }
    }

}


/*fn main() {
  unsafe {
    println!("ENTER THE NUMBER OF NODES");    
    n = input();

    println!("ENTER NO OF EDGES");
    let e: i16 = input();

    for _ in 0..e {
      let nod_1: i16 = input();
      let nod_2: i16 = input();

      adj[nod_1 as usize][nod_2 as usize] = true;
    }

    for i in 0..n {
      if cycle_found {
      	break;
      }
      if !visited[i as usize] {
        dfs(i);
      }
    }

    if !cycle_found {
      println!("no cycle present");
    } else {
      println!("cycle present");
    }
  }
}*/

pub fn cyc_det_directed_y2k(matrix: &mut [[i32; 350]; 350], n_tmp: i32) {
  unsafe {
   
    n_second = n_tmp;


    for i in 0..n_second {
		for j in 0..n_second {
    			let x_second: i32 = matrix[i as usize][j as usize];

			if (x_second != 0) {
      				adj_second[i as usize][j as usize] = true;
			}
		}
    }

    for i in 0..n_second {
      if cycle_found_second {
      	break;
      }
      if !visited_second[i as usize] {
        dfs_second(i);
      }
    }

    if !cycle_found_second {
      println!("no cycle present");
    } else {
      println!("cycle present");
    }
  }
}

fn dfs_second(crr_nod: i32) {
  unsafe {
    if cycle_found_second {
     return;
   }

   let mut children = vec![];
   visiting_second[crr_nod as usize] = true;

   for i in 0..n_second {
    if cycle_found_second {
     return;
   }
   
   if adj_second[crr_nod as usize][i as usize] {
    if visiting_second[i as usize] {
      cycle_found_second = true;            
      return;
    } else if !visited_second[i as usize] {
      if thr_count_second < MAX_THREADS_second {
       children.push(thread::spawn(move || {              
        dfs_second(i);
      }));
     } else {
      dfs_second(i);
    }
  }
}
}

for child in children {
  let _ = child.join();
}

visiting_second[crr_nod as usize] = false;
visited_second[crr_nod as usize] = true;
}
}



pub fn check_bipartite(matrix: &mut [[i32; 350]; 350], n_tmp: i32) -> () {
	unsafe {
	n43 = n_tmp;
    	for i in 0..n43 {
		  for j in 0..n43 {
    			let x43: i32 = matrix[i as usize][j as usize];

			     if (x43 != 0) {
      				adj43[i as usize][j as usize] = true;
			     }
		  }
    	}
    	
    	for i in 0..n43 {
		for j in i..n43 {
			if adj43[i as usize][j as usize] != adj43[j as usize][i as usize] {
				println!("error: input does not contain undirected graph\n");
				return;
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

pub fn cycle_detection(mat134180: &mut [[i32; 350]; 350], n1: i32) {
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
		
		for i in 0..n180 {
			for j in 0..i {
				if adj180[i as usize][j as usize] != adj180[j as usize][i as usize] {
					println!("not an undirected graph: wrong input!");
					return;
				}
			}
		}
		
		//let start = PreciseTime::now();
		for i in 0..n180 {
			if cycle_found180 {
				break;
			}
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
		if cycle_found180 {
			return;
		}
		
		let mut children = vec![];
		visited180[crr_nod as usize] = true;
		
		for i in 0..n180 {
			if cycle_found180 {
				return;
			}
			
			if adj180[crr_nod as usize][i as usize] != 0 && i != crr_par {
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


use std::env;

//---------Static Declaration----------------//

static mut xs_5: [[i32; 350]; 350] = [[0; 350]; 350];
static mut temp_5: [[i32; 350]; 350] = [[0; 350]; 350];
static mut n_5 : i32 = 0;
static mut stack_5: [i32; 350] = [0; 350];



//-------------------------------------------//


fn timestamp () -> f64 {
    let timespec = time::get_time();
    // 1459435009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0 );
    mills
}

fn tp_sort(visited: &mut [i32; 350], i: usize, index: &mut usize) {
unsafe {
	visited[i as usize] = 1;
	for j in 0..n_5 {
		if (temp_5[i as usize][j as usize] == 1) {
			if visited[j as usize] == 0 {
				tp_sort(visited, j as usize, index);
			}
		}
	}
	*index = *index + 1;
	stack_5[*index] = i as i32;
}
}

pub fn func(graph1 : &mut [[i32;350];350], n1:i32) {
unsafe {
	for i in 0..n1 {
		for j in 0..n1 {
			xs_5[i as usize][j as usize] = graph1[i as usize][j as usize];
		}
	}
	n_5=n1;
	let ts = timestamp();
	//n_5=n1;
	/*println!("--------------without thread------------------");
	//---------------------------------------WITHOUT THREAD-------------------------------
	for i in 0..n_5 {
		for j in 0..n_5 {
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

	println!("------------------------");
	for i in 0..n_5 {
		for j in 0..n_5 {
			print!("{} ",temp_5[i as usize][j as usize]);
		}
		println!("");
	}
	let mut index = 0; // index of stack_5
	let mut visited: [i32; 350] = [0; 350]; // visited array
	for i in 0..n_5 {
		if visited[i as usize] == 0 {
			//println!("{}",i);	
			tp_sort(&mut visited, i as usize, &mut index);
		}
	}

	let mut pos_u = 0;
	let mut pos_v = 0;
	for i in 0..n_5 {
		for j in 0..n_5 {
			if i != j && xs_5[i as usize][j as usize] == xs_5[j as usize][i as usize] && xs_5[i as usize][j as usize] == 1 { 
				for k in 1..(n_5+1) {
					if stack_5[k as usize] == i {
						pos_u = k;
					}
					if stack_5[k as usize] == j {
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
	println!("------------------------");		
	for i in 0..n_5 {
		for j in 0..n_5 {
			print!("{} ",temp_5[i as usize][j as usize]);
		}
		println!("");
	}
	let ts2 = timestamp();
	let b = ts2 - ts;
	println!("time stamp : {:?}", b);*/
	println!("-------------------with thread-----------------------------");
	//----------------------------THREAD------------------------------------
	println!("thread 1");
	let data = Arc::new(Mutex::new((xs_5, temp_5)));
	//println!("step1");
	let data = data.clone();	
	let t1 = thread::spawn( move || {
		//let mut data = data.lock().unwrap();
		for i in 0..n_5 {
			for j in 0..(n_5 - i) {
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
	//println!("step 2");
	let data = data.clone();
	let t2 = thread::spawn( move || {
		//let mut data = data.lock().unwrap();	
		for i in 0..n_5 {
			for j in (n_5 - i)..(n_5) {
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

	let j1 = t1.join();
	let j2 = t2.join();

	// directed subgraph
	println!("------------------------");
	for i in 0..n_5 {
		for j in 0..n_5 {
			print!("{} ",temp_5[i as usize][j as usize]);
		}
		println!("");
	}


// creating stack_5 as array
	let mut index = 0; // index of stack_5
	let mut visited: [i32; 350] = [0; 350]; // visited array
	for i in 0..n_5 {
		if visited[i as usize] == 0 {
			//println!("{}",i);	
			tp_sort(&mut visited, i as usize, &mut index);
		}
	}

	//println!("output");
	//for i in 1..(n_5+1) {
	//	print!("{}",stack_5[i as usize]);
//	}
	
	let mut pos_u = 0;
	let mut pos_v = 0;
	
	
	let data1 = Arc::new(Mutex::new((stack_5, temp_5, xs_5)));
	let data1 = data1.clone();
	
	let t3 =thread::spawn( move || {
	for i in 0..n_5 {
		for j in 0..(n_5 - i) {
			if i != j && xs_5[i as usize][j as usize] == xs_5[j as usize][i as usize] && xs_5[i as usize][j as usize] == 1 { 
				for k in 1..(n_5+1) {
					if stack_5[k as usize] == i {
						pos_u = k;
					}
					if stack_5[k as usize] == j {
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
	let data1 = data1.clone();
	
	let t4 = thread::spawn( move || {	 
	for i in 0..n_5 {
		for j in (n_5-i)..(n_5) {
			if i != j && xs_5[i as usize][j as usize] == xs_5[j as usize][i as usize] && xs_5[i as usize][j as usize] == 1 { 
				for k in 1..(n_5+1) {
					if stack_5[k as usize] == i {
						pos_u = k;
					}
					if stack_5[k as usize] == j {
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

	let res = t3.join();
	let res = t4.join();
	//ts2 = timestamp();
	println!("------------------------");		
	for i in 0..n_5 {
		for j in 0..n_5 {
			print!("{} ",temp_5[i as usize][j as usize]);
		}
		println!("");
	}
	let ts2 = timestamp();
	let b = ts2 - ts;
	println!("time stamp : {:?}", b);
	//println!("time stamp : {:?}", ts2-ts1);

}
}




pub fn caller_54 (dummy : &mut [[i32; 350]; 350], n : i32) {
		
	unsafe{
			for i in 0..350 {
				for j in 0..350 {
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


#[derive(Clone)]
enum SetItem<ValueType>
{
    Root(ValueType, u32),
    Link(Box<SetItem<ValueType>>)
}


/// Tarjan's Union-Find Data structure
//#[derive(Copy, Clone)]
pub struct DisjointSet<T: Clone>
{
set_size:
    usize,
    /// The structure saves the parent information of each subset in continuous
    /// memory(a vec) for better performance.
parent:
    Vec<usize>,

    /// Each T entry is mapped onto a usize tag.
map:
    HashMap<T, usize>
}

impl<T> DisjointSet<T> where T:
Clone+Hash+Eq
{
    pub fn new() -> Self{
        DisjointSet{
            set_size: 0,
            parent:
                Vec::new(),
            map:
                HashMap::new()
        }
    }

    pub fn make_set(&mut self, x: T)
    {
        let mut len = &mut self.set_size;
        match self.map.get(&x)
        {
            Some(p) => return,
            None => {}
        }

        self.map.insert(x, *len);
        self.parent.push(*len);

        *len += 1;
    }


    pub fn find(&mut self, x: T) -> Option<usize>
    {
        //! Returns Some(num), num is the tag of subset in which x is.
        //! If x is not in the data structure, it returns None.

    let mut pos:
        usize;
        match self.map.get(&x)
        {
            Some(p) => {pos = *p;},
            None => return None
        }

        let ret = DisjointSet::<T>::find_internal(&mut self.parent, pos);
        Some(ret)
    }

    fn find_internal(p: &mut Vec<usize>, n: usize) -> usize{
        if p[n] != n{
            let parent = p[n];
            p[n] = DisjointSet::<T>::find_internal(p, parent);
            p[n]
        }
        else {
            n
        }
    }


    pub fn union(&mut self, x: T, y: T) -> Result<usize, ()>
    {
        //! Union the subsets to which x and y belong.
        //! If it returns Ok<u32>, it is the tag for unified subset.
        //! it returns Err(), at least one of x and y is not in the disjoint-set.
        let x_root;
        let y_root;
        match self.find(x)
        {
            Some(x_r) => {x_root = x_r;},
            None =>
            {
                return Err(());
            }
        }

        match self.find(y)
        {
            Some(y_r) => {y_root = y_r;},
                 None =>
            {
                return Err(());
            }
        }

        self.parent[x_root] = y_root;
        Ok(y_root)
    }
}


static mut adjMatrix_073:[[i32; 350]; 350] = [[0; 350];350];

pub fn conn_comp_073(matrix: &mut [[i32; 350]; 350], n_tmp: i32) -> () {
    unsafe {	
	let mut n = n_tmp as i32;	
	for i in 0..n { 
	    for j in 0..n { 
	    	if matrix[i as usize][j as usize] != 0 {
			adjMatrix_073[i as usize][j as usize] = 1;
		} else {
			adjMatrix_073[i as usize][j as usize] = 0;
		} 
	    } 
	}
	
	let mut p = 2;
	

    let mut start = 0;
    let mut end =  ((n) / p);

    let mut setData: Vec<DisjointSet<i32>> = Vec::with_capacity(p as usize);
    let mut storeConnectedComponents = vec![0;n as usize];
    let mut numberOfConnectedComponents = 0;
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
     
    //let startt = PreciseTime::now();

    for i in 0..p {
        let thread_tx = tx.clone();
	let handle = thread:: spawn(move ||{

            if end >= n {
                end = n-1;
            }

            //let mut adjMatrix = vec;
            let mut visited = vec![0; n as usize];
            let mut connComp = 0;

            let mut ds = DisjointSet::<i32>::new();


            for _j in 0..n {
                ds.make_set(_j as i32);
            }


            for _j in 0..n {
                if start <= _j && _j <= end && visited[_j as usize] == 0 {
                    connComp+=1;
                    dfs_073(start as usize, end as usize, _j as usize, n as usize, &mut visited, &mut ds);
                }
            }

            for _k in 0..n {
                if visited[_k as usize] == 0 {
                    connComp+=1;
                }
            }

            thread_tx.send(ds).unwrap();
            //println!("Thread number {}, Connected Components = {}", i, connComp);
            //tmp.push(handle);

        });



        start = end+1;
        end = start + (n)/p;
        if i == p-1 {
            end = n-1;
        }

        handles.push(handle);
        //let res = handle.join();
    }

    for _h in handles {
        let mut handle = _h;
        let res = handle.join();
    }

    for _i in 0..p {
        setData.push(rx.recv().unwrap());
    }

    let mut ind = 0;

    
            
        let mut _i = 1;
        while _i < p {
              
             for _j in 0..n {


                let mut u = _j;
                let mut v = setData[_i as usize].parent[_j as usize];
                let mut prod = DisjointSet::<i32>::new();
                let mut flag = 0;

                if u as i32 != v as i32 {
                    match setData[0].find(u as i32) {
                        Some(_r) => match setData[0].find(v as i32) {
                            Some(_rr) => {
                                if _r != _rr {
                                    setData[0].union(u as i32, v as i32);
                           //         flag = 1;
                 //                   println!("----******------");
                                }
                            },
                            None => {},
                        },
                        None => {},
                        }
                 }


              }
              
            _i = _i+1;
        }

           //let endt = PreciseTime::now();
    
            for _j in 0..n {
                match setData[0].find(_j as i32) {
                        Some(k) => {
                            //println!("Node:{}, Parent:{}", _j, k);
                            storeConnectedComponents[k as usize] += 1;
                        },
                        None=> {},
                }
            }
            for _i in &storeConnectedComponents {
                if *_i >= 1 {

                    numberOfConnectedComponents += 1;
                }
            }
            println!("Number of Connected Components : {}", numberOfConnectedComponents);
            //println!("{:?}", startt.to(endt));
        
    }
} 

/*
fn main() {
    
   
	println!("1");
	    let mut test:[[i32; 350]; 350] = [[0; 350];350]; 
	    
	   test[0][1] = 1; test[1][0] = 1;
	   test[3][2] = 1; test[2][3] = 1;
	   
	   
	  println!("2"); 
	    
	    let mut n = 10;    
	    conn_comp_073(&mut test, n); 
	println!("3");

}

*/
fn dfs_073(start: usize, end: usize, source: usize, n: usize, visited: &mut Vec<i32>, ds: &mut DisjointSet<i32>)
{

    unsafe {
	    let mut stack: Vec<i32> = Vec::new();

	    stack.push(source as i32);
	    visited[source] = 1;
	    while let Some(top) = stack.pop() {
		for _i in 0..n {
		    if _i != top as usize && adjMatrix_073[top as usize][_i] == 1 && visited[_i] == 0 {
		        if start <= _i && _i <= end {
		            stack.push(_i as i32);
		        }
		        visited[_i as usize] = 1;
		        ds.union(top, _i as i32);

		    }
		}
	    }
    }
}


fn input_apurv() -> i32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,//println!("your integer input_apurv: {}", i),
        Err(..) =>return 0,//println!("this was not an integer: {}", trimmed)
    };
    //return 0
}


pub fn count_ways(mat: &mut [[i32; 350]; 350], n: i32){
	let u: i32 = input_apurv();
	let v: i32 = input_apurv();
	let k: i32 = input_apurv();
	let m: i32 = input_apurv();
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



fn input_25() -> i32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,//println!("your integer input_1111: {}", i),
        Err(..) =>return 0,//println!("this was not an integer: {}", trimmed)
    };
    return 0;
}



pub fn input_1111(mat : & mut [[i32;350];350], n:i32) -> bool{
	let mut v:Vec<Vec<i32>> = Vec::new();
	for i in 0..n{
		let mut v1:Vec<i32> = Vec::new(); 
		for j in 0..n{
			v1.push(mat[i as usize][j as usize]);
		}	
		v.push(v1);
	}

	let mut n1:i32 = input_25();

	
	let mut n2:i32 = input_25();

	let mut res1: bool = findTheNode_serial(&v, n1, n2);
	let mut res: bool = findTheNode(&v, n1, n2);
	return res;
}


fn findTheNode_serial(mut v : &Vec<Vec<i32>>, nodeA:i32, nodeB:i32) -> bool{
	let mut visited:Vec<i32> = Vec::new();
	let mut queue:Vec<i32> = Vec::new();
	println!("Serial Version");
	
	let mut numb:i32 = 1000;
        println!("Number of Nodes Taken:\t{:?}", v.len());

	queue.push(nodeA);
	visited.push(nodeA);
	let mut k:usize = 0;

	let mut var = false;
	let time1  =timestamp();
	
	 while k < queue.len() {
	     let mut node = queue[k];
	     //println!("Node Out:: {}",node);
	     k = k + 1;
	     
	     if node == nodeB {
	     	//println!("Found Node::{}, actually: {}", node,nodeB);
	        var =  true;
		let time2 = timestamp();
		let diff = time2 - time1;
		println!("Time Taken:\t{:?}",diff);
		break;
	     }

	   let mut cnt:i32 = 0;
	
	   if v.len() > 350 {
	   	for ss in &visited {
			for jj in &visited{
	   			cnt = cnt + 1;
				if cnt == numb{
					break;
				}
			}
	    	}
	    }
	
	     let mut i:i32 = 0;
	     for vector in v{
	     	if i == node {//go the current node here okay actually::
	     	    let mut ff:i32 = 0;
	     	    for j in vector{//now check if already visited:::
	     	    	let mut flag:i32 = 0;
	     	    	for k in &visited {
	     	    		if *k == ff {
	     	    		    flag = 1;
	     	    		    break;
				    
	     	    		}
	     	    	}
	     	    	
	     	    	if flag == 0 && *j == 1{
	     	    		queue.push(ff);
	     	    		visited.push(ff);
	     	    	}
	     	    	ff = ff + 1;
	     	    }
	     	}
	     	i = i + 1;
	     }
	 }

	if var == false {
		let mut tt = timestamp();
 		let mut te = tt-time1;
		println!("Time Taken: {:?}",te);
	}
	return var;
}


pub fn findTheNode(mut v : &Vec<Vec<i32>>, nodeA:i32, nodeB:i32)-> bool{
	let mut visited:Vec<i32> = Vec::new();
	let mut queue:VecDeque<i32> = VecDeque::new();
	let mut var:bool;
	let processor:usize = 4;
	let mut graph: Vec<Vec<i32>> = Vec::new();
	
	println!("Parallel Version");
	for vec in v{
		let mut vv:Vec<i32> = Vec::new();
		for i in 0..vec.len(){
			vv.push(vec[i]);
		}
		graph.push(vv);
	}
    

    queue.push_back(nodeA);
    visited.push(nodeA);
    let mut k:usize = 0;
    let time1 = timestamp();

    loop {
         let mut node = queue[k];
         k = k + 1;

         if(queue.len() - k) >= processor{
            break;
         }
         
         if node == nodeB {
            //println!("Found Node::{}, actually: {}", node,nodeB);
	 	let time2 = timestamp();
		let diff = time2-time1;
		println!("Time Taken:\t{:?}", diff);
             	return true;
         }

         let mut i:i32 = 0;
         for vector in v{
            if i == node {//go the current node here okay actually::
                let mut ff:i32 = 0;
                for j in vector{//now check if already visited:::
                    let mut flag:i32 = 0;
                    for k in &visited {
                        if *k == ff {
                            flag = 1;
                            break;
                        }
                    }
                    if flag == 0 && *j == 1{
                        queue.push_back(ff);
                        visited.push(ff);
                    }
                    ff = ff + 1;
                }
            }
            i = i + 1;
         }
     }
     for i in 0..k{
     	queue.pop_front();
     }

    let  graph1 = Arc::new(Mutex::new(v));
    let nVisit = Arc::new(Mutex::new(visited));
    let nqueue = Arc::new(Mutex::new(queue));
    let cnt:i32 = 0;
    let mut cntr = Arc::new(Mutex::new(cnt));
    let mut handles = Vec::new();
    let tar = Arc::new(Mutex::new(nodeB));
    let mut kk:Vec<i32> = Vec::new();
    kk.push(-1);
    let result = Arc::new(Mutex::new(kk));

    for adi in 0..processor{
    	
    	let mut out = nqueue.clone();
    	let mut temp = nVisit.clone();
    	let mut temp1 = tar.clone();
    	let mut temp2 = graph.clone();
        let mut res = result.clone();

        let handle = thread::spawn(move || unsafe {
            let x = out.lock().unwrap().pop_front();
            let mut num:i32 = 0;
            let mut flag = -1;
            loop{
                
              	let mut x = out.lock().unwrap().pop_front();
                let mut node:i32 = 0;
                let mut te = 0;
                match x{
                    Some(d) => node = d,
                    None => te = 1
                }

                if out.lock().unwrap().len() == 0{
                    flag = 0;
                    break;
                }

                if node == nodeB {
                    flag = 1;
                    res.lock().unwrap()[0] = 1;
		    let time2 = timestamp();
		    let diff = time2-time1;
		    println!("Time Taken:\t{:?}", diff);
		    println!("The Nodes are Connected");
                    break;
                }
                let mut i:i32 = 0;
                for vector in &temp2{
                    if i == node {//go the current node here okay actually::
                        let mut ff:i32 = 0;
                        for j in vector{//now check if already visited:::
                            let mut flag:i32 = 0;
                            for k in temp.lock().unwrap().iter() {
                                if *k == ff {
                                    flag = 1;
                                    break;
                                }
                            }
                            if flag == 0 && *j == 1{
                                out.lock().unwrap().push_back(ff);
                                temp.lock().unwrap().push(ff);
                            }
                            ff = ff + 1;
                        }
                    }
                    i = i + 1;
                }
            }            
        });
        handles.push(handle);
	
    }
    
    for h in handles {
        h.join().unwrap();
    }

    if result.lock().unwrap()[0] == 1{
        return true;
    }
    else
    {
	let mut ts = timestamp();
	let mut p = ts - time1;
	println!("Time taken: {}",p);
    }


    return false;
}

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

pub fn check_same_set_105 (dummy : &mut [[i32; 350]; 350], n : i32) {
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
	    	check_same_set_105(dummy, n);
	}
}*/

fn input_062() -> i32 { 
    let mut input_text = String::new(); 
    io::stdin() 
        .read_line(&mut input_text) 
        .expect("failed to read from stdin"); 
 
    let trimmed = input_text.trim(); 
    match trimmed.parse::<i32>() { 
        Ok(i) => return i, 
        Err(..) =>return 0, 
    }; 
    return 0 
} 
fn get_node_062<R:Read>(reader:R)->i32 { 
    let mut reader = BufReader::new(reader).lines(); 
    let mut nn:i32=0; 
    while let Some(Ok(line)) = reader.next() { 
        let node_info= line.to_owned(); 
        nn = node_info.parse::<i32>().unwrap(); 
        break; 
    } 
    return nn; 
} 
 
fn get_graph_062<R:Read>(reader:R)->i32 { 
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
                unsafe{v_062[i as usize][j as usize]=q;} 
            }
            i+=1; 
        } 
    } 
    return k; 
} 
 
fn topologicalsort_062(mut used: &mut Vec<i32>,mut ans : &mut Vec<i32>,i: i32) -> () { 
    unsafe { 
    used[i as usize]=1; 
    for k in 0..n_062 { 
        if v_062[i as usize][k as usize]!=0&&used[k as usize]==0 { 
            topologicalsort_062(&mut used,&mut ans,k); 
        } 
    } 
//    println!("here {}",i); 
    ans.push(i); 
    } 
} 
 
pub fn solve_062(matr: &mut [[i32; 350]; 350], x__1: i32) -> () { 
        unsafe {   
		n_062=x__1;
		for i in 0..n_062 { 
			for j in 0..n_062 { 
				v_062[i as usize][j as usize]=matr[i as usize][j as usize]; 
			} 
		}
    let mut used:Vec<i32> = Vec::new(); 
    let mut ans:Vec<i32> = Vec::new(); 
    /* 
    for i in 0..n_062 { 
        for j in 0..n_062 { 
            print!("{} ", v_062[i as usize][j as usize]); 
        } 
        println!(""); 
    } 
    */ 
    for i in 0..n_062 { 
        used.push(0); 
    } 
    for i in 0..n_062 { 
        if used[i as usize]==0 { 
            topologicalsort_062(&mut used,&mut ans,i); 
        } 
    } 
    let x = ans.len(); 
    ans.reverse(); 
//    println!("here x = {}",x); 
//    println!("topological order"); 
    for i in 0..x { 
        tmp_062[i as usize]=ans[i as usize]; 
//        println!("{}",ans[i as usize]); 
    } 
//    println!("----------------"); 
    for i in 0..n_062 { 
        for j in 0..n_062 { 
            dist1_062[i as usize][j as usize]=9999999999; 
        } 
    }
    let t1 = thread::spawn( move || { 
    for s in 0..n_062/3 { 
        dist1_062[s as usize][s as usize]=0; 
            for i in 0..x { 
                let mut ll:i32 = tmp_062[i as usize]; 
                if dist1_062[s as usize][ll as usize]!=9999999999 { 
                    for k in 0..n_062 { 
                        if v_062[ll as usize][k as usize]!=0 { 
                            if dist1_062[s as usize][k as usize]>dist1_062[s as usize][ll as usize]+v_062[ll as usize][k as usize] { 
                                dist1_062[s as usize][k as usize]=dist1_062[s as usize][ll as usize]+v_062[ll as usize][k as usize]; 
                            } 
                        } 
                    } 
                } 
            } 
         
    } 
    }); 
    let t2 =thread::spawn( move || { 
    for s in n_062/3..2*n_062/3 { 
        dist1_062[s as usize][s as usize]=0; 
            for i in 0..x { 
                let mut ll:i32 = tmp_062[i as usize]; 
                if dist1_062[s as usize][ll as usize]!=9999999999 { 
                    for k in 0..n_062 { 
                        if v_062[ll as usize][k as usize]!=0 { 
                            if dist1_062[s as usize][k as usize]>dist1_062[s as usize][ll as usize]+v_062[ll as usize][k as usize] { 
                                dist1_062[s as usize][k as usize]=dist1_062[s as usize][ll as usize]+v_062[ll as usize][k as usize]; 
                            } 
                        } 
                    } 
                } 
            } 
    } 
    });
    let t3 =thread::spawn( move || { 
    for s in 2*n_062/3..n_062 { 
        dist1_062[s as usize][s as usize]=0; 
            for i in 0..x { 
                let mut ll:i32 = tmp_062[i as usize]; 
                if dist1_062[s as usize][ll as usize]!=9999999999 { 
                    for k in 0..n_062 { 
                        if v_062[ll as usize][k as usize]!=0 { 
                            if dist1_062[s as usize][k as usize]>dist1_062[s as usize][ll as usize]+v_062[ll as usize][k as usize] { 
                                dist1_062[s as usize][k as usize]=dist1_062[s as usize][ll as usize]+v_062[ll as usize][k as usize]; 
                            } 
                        } 
                    } 
                } 
            } 
    } 
    }); 
    let res1 = t1.join(); 
    let res2 = t2.join();
    let res3 = t3.join();
    for i in 0..n_062 { 
        for j in 0..n_062 { 
            if(dist1_062[i as usize][j as usize]==9999999999) { 
                print!("INF "); 
            } 
            else { 
                print!("{} ",dist1_062[i as usize][j as usize]); 
            } 
        } 
        println!(""); 
    } 
   } 
} 


fn input_146() -> usize {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<usize>() {
        Ok(i) => return i,//println!("your integer input: {}", i),
        Err(..) =>return 0,//println!("this was not an integer: {}", trimmed)
    };
    return 0
}


fn get_node_146<R:Read>(reader:R)->i32 {
    let mut reader = BufReader::new(reader).lines();
    let mut ll = 0;
    unsafe {
        while ll < ins {
            while let Some(Ok(line)) = reader.next() {
                break;
            }
            ll+=1;
        }
    }
    let mut nn:i32=0;
    while let Some(Ok(line)) = reader.next() {
        let node_info= line.to_owned();
        nn = node_info.parse::<i32>().unwrap();
        break;
    }
    return nn;
}

fn get_graph_146<R:Read>(reader:R)->i32 {
    let mut reader = BufReader::new(reader).lines();
    let mut ll = 0;
    unsafe {
        while ll < ins {
            while let Some(Ok(line)) = reader.next() {
                break;
            }
            ll+=1;
        }
    }
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
                unsafe{graph_146[i as usize][j as usize]=q;}
            }
            i+=1;
        }
        else {
            break;
        }
    }
    return k;
}

fn bfs_146 (mut root: i32) {
    unsafe {
        queue_146[rear_146 as usize] = root;
        level_146[root as usize] = 0;
        visited_146[root as usize] = 1;
        rear_146 = rear_146 + 1;
        let mut len:i32 = 0;
        loop {
            if front_146 >= rear_146 {
                break;
            }
            len = rear_146 - front_146;
            if (len >= 6/*no_of_processors_146*/) {
                break;
            }
            let mut p = queue_146[front_146 as usize];

            levsize_146[level_146[p as usize] as usize] = levsize_146[level_146[p as usize] as usize] + 1;
            mat_146[level_146[p as usize] as usize][levsize_146[level_146[p as usize] as usize] as usize] = p;
            //print!("{} ", p);
            front_146 = front_146 + 1;
            for i in 0..n_146 {
                if graph_146[p  as usize][i as usize] != 0  &&  visited_146[i as usize] == 0 {
                        queue_146[rear_146  as usize] = i;
                        rear_146 = rear_146 + 1;
                        visited_146[i as usize] = 1;
                        level_146[i as usize] = level_146[p as usize] + 1;
                        if (maxlevel_146 < level_146[i as usize]) {
                            maxlevel_146 = level_146[i as usize];
                        }
                }
            }
        }
        if rear_146 > front_146 {
            let data = Arc::new(Mutex::new((visited_146, mat_146, levsize_146, level_146, maxlevel_146, queue_146, front_146, rear_146)));
            let mut children = vec![];

            //println!("INSIDE");
            for i in 0..no_of_processors_146
            {

                let data = data.clone();
                children.push(thread::spawn( move || {
                println!("Thread Id : {}", i);
                //println!("front_146 = {}", front_146);
                //println!("rear_146 = {}", rear_146);
                loop {
                    if front_146 >= rear_146 {
                        break;
                    }

                    let mut p = queue_146[front_146 as usize];
                    levsize_146[level_146[p as usize] as usize] = levsize_146[level_146[p as usize] as usize] + 1;
                    mat_146[level_146[p as usize] as usize][levsize_146[level_146[p as usize] as usize] as usize] = p;
                    //print!("{} ", p);
                    front_146 = front_146 + 1;
                    let mut data = data.lock().unwrap();
                    for i in 0..n_146 {
                        if graph_146[p  as usize][i as usize] != 0 &&  visited_146[i as usize] == 0 {
                            queue_146[rear_146  as usize] = i;
                            rear_146 = rear_146 + 1;
                            visited_146[i as usize] = 1;
                            level_146[i as usize] = level_146[p as usize] + 1;
                            if (maxlevel_146 < level_146[i as usize]) {
                                maxlevel_146 = level_146[i as usize];
                            }
                        }
                    }
                }
                }));
            }

            for child in children {
                // Wait for the thread to finish. Returns a result.
                let _ = child.join();
            }
        }
        //println();
    }

}

pub fn parallel_bfs_146(graph1: &mut [[i32;350];350],n : i32) {
    unsafe{
    for i in 0..n {
        for j in 0..n {
            graph_146[i as usize][j as usize] = graph1[i as usize][j as usize];
        }
    }
    n_146 = n;
	//println!("num nodes : {}",n_146);
    /*    let args:Vec<_> = env::args().collect();
        if args.len()!=2{
            panic!("Error with file reading");
        }
        let file = File::open(&args[1]).expect("Error");
        //let mut t2 = get_node_146(file);
        //ins+=1;
        //println!("Number of test cases : {}",t2);
        //while t2>0 {
            ---------initializations---------------------
                level_146 = [-1;350];
                visited_146 = [0; 350];
                levsize_146 = [-1;350];
                queue_146 = [0; 350];
                graph_146 = [[0;350];350];
                mat_146 = [[0;350];350];
                front_146 = 0;
                rear_146 = 0;
                maxlevel_146 = 0;
            -----------------------------


            //t2 -= 1;
            //let file1 = File::open(&args[1]).expect("Error");

            //n_146 = get_node_146(file1);
            //ins+=1;
            println!("Number of nodes : {}",n);
            let file2 = File::open(&args[1]).expect("Error");

            //let mut k:i32;
            //k = get_graph_146(file2);
            //ins+=n_146;
            //println!("");

            */
            for i in 0..n_146 {
                if visited_146[i as usize] == 0 {
                    println!("For root {}",i);
                    bfs_146(i);
                }
            }
            println!("");
            println!("Printing levelwise :");

            for i in 0..(maxlevel_146 + 1) {
                println!("level :: {}", i);
                for j in 0..(levsize_146[i as usize] + 1) {
                    println!("{} ", mat_146[i as usize][j as usize]);
                }
            }
        //}
    }
}


//fn main () {
pub fn solve_bfs(input: &mut [[i32; 350]; 350], nodes: i32)
{
    unsafe{
        let mut graph1 : [[i32; 350]; 350] = [[0; 350]; 350];
        let n :i32;
        //let args:Vec<_> = env::args().collect();
       // if args.len()!=2{
      //      panic!("Error with file reading");
      //  }
       // let file = File::open(&args[1]).expect("Error");
        //let mut t2 = get_node_146(file);
        //ins+=1;
        //println!("Number of test cases : {}",t2);
        //while t2>0 {
            //---------initializations---------------------
                level_146 = [-1;350];
                visited_146 = [0; 350];
                levsize_146 = [-1;350];
                queue_146 = [0; 350];
                graph_146 = [[0;350];350];
                mat_146 = [[0;350];350];
                front_146 = 0;
                rear_146 = 0;
                maxlevel_146 = 0;
            //-----------------------------


            //t2 -= 1;
           // let file1 = File::open(&args[1]).expect("Error");

           // n = get_node_146(file1);
            n = nodes;
            //ins+=1;
            println!("Number of nodes : {}",n);
           // let file2 = File::open(&args[1]).expect("Error");

            let mut k:i32;
          //  k = get_graph_146(file2);
            //ins+=n_146;
            //println!("");

            for i in 0..nodes {
                for j in 0..nodes {
                    graph1[i as usize][j as usize] = input[i as usize][j as usize];
                }
            }


        parallel_bfs_146(&mut graph1,n);
    }
}

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

pub fn dfsutil_034(local_adj_034: &mut [[i32; 350]; 350], local_nodes_034:i32) {
	/*unsafe{
    	n_034=0;
    	irow_034=0;
    	assigned_034=-1;
    	for izero in 0..350 {
    		for jzero in 0..350 {
    			adj_034[izero as usize][jzero as usize]=0;
    			order_034[izero as usize][jzero as usize]=0;
    			vis_034[izero as usize][jzero as usize]=0;
    		}
    	}
    	for izero in 0..350 {
    		icol_034[izero as usize]=0;
    		printed_034[izero as usize]=0;
    	}
    	for izero in 0..350 {
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
						flag2=350;
						break;
					}
				}
				if flag2==0{
					printed_034[index as usize]=children_034[i as usize] as i32;
					index+=1;
					
					for j in 0..350 {
						if order_034[i as usize][j as usize]!=999 {
							let mut flag:i32=0;
							for k in 0..index {
								if order_034[i as usize][j as usize]==printed_034[k as usize] {
									flag=350;
									break;
								}
							}
							if flag!=350 {
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
    let mut local_adj_034:[[i32; 350]; 350] = [[0; 350];350];
    unsafe{
    	for i in 0..n_034 {
    	for j in 0..n_034 {
    		local_adj_034[i as usize][j as usize] = adj_034[i as usize][j as usize];
    	}
    }	
    }
    
		
	dfsutil_034(&mut local_adj_034, local_nodes_034);	
}*/


pub fn floydwarshall_207(inputx: &mut [[i32; 350]; 350],n: i32) {                               
                               
  unsafe{
       for i in 0..n {
         for j in 0..n {
            dist_207[i as usize][j as usize] = inputx[i as usize][j as usize];                                      
         }
       }
      
       
   
       let mut children = vec![];                            // Make a vector to hold the children which are spawned.

       for m in 0..NTHREADS{
         for k in 0..n {
            for i in 0..n {
              children.push(thread::spawn(move || {           // Spin up another thread
                  for j in 0..n {
        
                     if dist_207[i as usize][k as usize] + dist_207[k as usize][j as usize] < dist_207[i as usize][j as usize] {
                     dist_207[i as usize][j as usize] = dist_207[i as usize][k as usize] + dist_207[k as usize][j as usize];
                     }
                  }
              })); 
            } 
          }
        }
        println!("NUMBER OF NODES = {}",n);
        //println!("NNUMBER OF THREADS = {}", NTHREADS);
        for child in children {
           let _ = child.join();                                   // Wait for the thread to finish. Returns a result.
        }
     
        print(n);                                                                      
  }
}





fn print(n: i32) {
unsafe{
  println!("Following matrix shows the shortest dist_207ances between every pair of vertices");
    for i in 0..n {
       for j in 0..n {
            if dist_207[i as usize][j as usize] == 99999{
                print!("99999,");
                }
            else {
            print!("{},  ",dist_207[i as usize][j as usize]);
                  }    
        }
        println!("");

    }
 println!("");
}
}


pub fn bellman_ford_110(mat_110: &mut [[i32; 350]; 350], n: i32) {
unsafe{
  	for i in 0..n {
      	for j in 0..n {
      		adj_110[i as usize][j as usize] = mat_110[i as usize][j as usize];
      	}
    }

    let mut root: i32 = 0;
    let mut num_th: i32 = 4;

    let mut edg: i32;
    let mut ke: i32 = 0;
    for i in 0..n {
      for j in 0..n {
        let mut tt: i32 = adj_110[i as usize][j as usize];
        if tt != 0 {
          source_110[ke as usize] = i;
          dest_110[ke as usize] = j;
          weight_110[ke as usize] = tt;
          ke = ke + 1;
        }
      }
    } 

    edg = ke;

    let mut itt:i32 = n/num_th+1;

//    let mut it:i32 = 0;
  dist_110[root as usize] = 0;
  if num_th > 1 && num_th <= n {
    for th in 0..num_th {
      thread::spawn(move||{
          for oo in 1..itt {
            for ii in 0..edg {
              let mut ww = source_110[ii as usize];
              let mut ee = dest_110[ii as usize];
              let mut wt = weight_110[ii as usize];

              if dist_110[ww as usize] != 9999999 && dist_110[ww as usize] + wt < dist_110[ee as usize]{
                dist_110[ee as usize] = dist_110[ww as usize] + wt;
              }
            }
          }
      });
    }
  }
  else if num_th == 1 {      
      for oo in 1..n {
          for ii in 0..edg {
            let mut ww = source_110[ii as usize];
            let mut ee = dest_110[ii as usize];
            let mut wt = weight_110[ii as usize];

            if dist_110[ww as usize] != 9999999 && dist_110[ww as usize] + wt < dist_110[ee as usize]{
              dist_110[ee as usize] = dist_110[ww as usize] + wt;
            }
          }
      }
  }
  else {
      for oo in 1..n {
        thread::spawn(move||{
          for ii in 0..edg {
            let mut ww = source_110[ii as usize];
            let mut ee = dest_110[ii as usize];
            let mut wt = weight_110[ii as usize];

            if dist_110[ww as usize] != 9999999 && dist_110[ww as usize] + wt < dist_110[ee as usize]{
              dist_110[ee as usize] = dist_110[ww as usize] + wt;
            }
          }
        });
      }
  }

/*  
    
      for jj in 0..edg{
          let mut qw = source_110[jj as usize];
          let mut we = dest_110[jj as usize];
          let mut wt2 = weight_110[jj as usize];

          if dist_110[qw as usize] != 9999999 && dist_110[qw as usize] + wt2 < dist_110[we as usize]{
            println!("Graph contains negative weight_110 cycle");
          }
      }



*/
      for hg in 0..n {
          print!("min distance of {} = ",hg);
          println!("{}",dist_110[hg as usize]);
      }      
  }
}


fn find_shortest_path(start:u32, nodes: u32, adj_matrix: &Vec<Vec<u32>> )   {
        
    let mut distance = Vec::new();  
    
    let mut i = 0;
    while i != nodes {
        distance.push(INF);
        i = i + 1;
    }

    let mut heap = BinaryHeap::new();
    distance[start as usize] = 0;
    heap.push(QueNode { cost: 0, vertex: start });
    
     while let Some(QueNode { cost , vertex })  = heap.pop() {
        if cost > distance[vertex as usize] { 
            continue; 
        }

        for i in 0..nodes {
            let weigh =  adj_matrix[vertex as usize][i as usize];    
            if i != vertex && weigh != 0 {
                 let next = QueNode { cost: cost + weigh, vertex: i };
                 if next.cost < distance[next.vertex as usize] {
                     heap.push(next);
                     distance[next.vertex as usize] = next.cost;
                 }
            }
        }
     }
     
     //println!("From Source {}" , start);
     unsafe {
     for i in 0..nodes {
         if distance[i as usize] != INF {
          //  print!("{} ", distance[i as usize]);
            output[start as usize][i as usize] = distance[i as usize];
         }
         else {
          //  print!("INF ");
            output[start as usize][i as usize] = INF;            
         }
     }
    }
    //println!("");
}

//pub fn johnson(adj_list: &Vec<Vec<u32>>, nodes: usize)
pub fn johnson(their_array: &mut [[i32; 350]; 350], node: i32)
{

   let mut nodes: usize = node as usize;     
    let mut adj_list: Vec<Vec<u32>> =  Vec::new();
  

    for i in 0..nodes {
        
        let mut temp = Vec::new();
        for j in 0..nodes {
            temp.push(their_array[i as usize][j as usize] as u32);
        }
        adj_list.push(temp);
        
    }


   
   let mut children = vec![];
   let mut j = 0;
   

   let output1 = Command::new("sh")
                     .arg("-c")
                     .arg("date +%s%3N")
                     .output()
                     .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    let mut start_time = String::from_utf8_lossy(&output1.stdout) ;

    
    let len1 = start_time.len();

    
    let hachi = &start_time[5..len1-1];
    let starter: u32 = hachi.parse().unwrap(); 





    while j <= nodes {
      //  println!("{}\n", j);
        for i in j..NTHREADS + j {
            if i < nodes {
                let adj_list =adj_list.clone();
                    children.push(thread::spawn(move || {
                        find_shortest_path(i as u32,nodes as u32,&adj_list);
                }));
            }
        }
        j = j + NTHREADS;
    }
    
    for child in children {
        let _ = child.join();
    }  

          
     let output2 = Command::new("sh")
                     .arg("-c")
                     .arg("date +%s%3N")
                     .output()
                     .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });


     let mut end_time = String::from_utf8_lossy(&output2.stdout) ;            


    unsafe {
    for i in 0..nodes {
        print!("Source {} : ", i);
        for j in 0..nodes {

            if output[i as usize][j as usize] != INF  {   

            print!("{} ",output[i as usize][j as usize]);
            
            }else {
            print!("INF ");

            }
        }
        println!("");

       
    } 
    }

   
    
     

    let len2 = end_time.len();

   
    let substr = &end_time[5..len2-1];
    let ender: u32 = substr.parse().unwrap(); 
    
    

   let diff: u32 = ender - starter;

 println!("Execution time : {} milliseconds",diff);
}

static mut it_058_adj_matrix: [[usize; 350]; 350] = [[0; 350]; 350];
static mut it_058_nodes:usize = 0;
static mut it_058_degree: [usize; 350] = [0; 350];
static mut it_058_sort: [usize; 350] = [0; 350];
static mut it_058_color: [usize; 350] = [0; 350];
static mut it_058_u:[usize; 350] = [0; 350];
static mut it_058_d2_matrix: [[usize; 350]; 350] = [[0; 350]; 350];
static mut it_058_forbidden: [[usize; 350]; 350] = [[0; 350]; 350];	// n * (2n - 1)
static mut it_058_cnt: [usize; 350] = [0; 350];

pub fn graphcoloring (mat_110: &mut [[i32; 350]; 350], n: i32) {	
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


pub fn radiocoloring (mat_110: &mut [[i32; 350]; 350], n: i32) {	
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
		println!("{} nodes.\nadj_matrix", it_058_nodes);
		print!("  ");
		for it_058_j in 0..it_058_nodes {
				print!("{} ", it_058_j);
			}	println!("");
		for it_058_i in 0..it_058_nodes {
			print!("{} ", it_058_i);	
			for it_058_j in 0..it_058_nodes {

				print!("{} ", it_058_adj_matrix[it_058_i][it_058_j]);
			}	println!("");
		}	println!("");

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

		println!("sort_matrix");
		for it_058_i in 0..it_058_nodes {
			print!("{} ", it_058_sort[it_058_i]);
		}	println!("");
		println!("degree_matrix");
		for it_058_i in 0..it_058_nodes {
			print!("{} ", it_058_degree[it_058_i]);
		}	println!("");
		println!("");

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

		println!("d2_matrix");
		print!("  ");
		for it_058_j in 0..it_058_nodes {
				print!("{} ", it_058_j);
			}	println!("");
		for it_058_i in 0..it_058_nodes {
			print!("{} ", it_058_i);
			for it_058_j in 0..it_058_nodes {
				print!("{} ", it_058_d2_matrix[it_058_i][it_058_j]);
			}	println!("");
		}	println!("");
		let mut calculate = 0;
		let mut it_058_lambda = it_058_nodes;
		//let start = PreciseTime::now();
		let mut it_058_j = 0;
		let mut it_058_flag = 0;
		let lam = 2 * it_058_nodes - 2;

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
				
					if it_058_adj_matrix[it_058_sort[it_058_j]][it_058_i] == 1 && it_058_c != 0 {
						it_058_forbidden[it_058_i][it_058_c - 1] = 1;
					}
					if it_058_adj_matrix[it_058_sort[it_058_j]][it_058_i] == 1 {
						it_058_forbidden[it_058_i][it_058_c] = 1;
					}
				
					let mut it_058_lim = it_058_nodes;
					it_058_lim = it_058_lim * 2 - 2;
					if it_058_adj_matrix[it_058_sort[it_058_j]][it_058_i] == 1 && it_058_c != it_058_lim {	// 2n - 2
						it_058_forbidden[it_058_i][it_058_c + 1] = 1;
					}
					if it_058_d2_matrix[it_058_sort[it_058_j]][it_058_i] == 1 {
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
					
						if it_058_adj_matrix[it_058_sort[it_058_jj]][it_058_i] == 1 && it_058_c != 0 {
							it_058_forbidden[it_058_i][it_058_c - 1] = 1;
						}
						if it_058_adj_matrix[it_058_sort[it_058_jj]][it_058_i] == 1 {
							it_058_forbidden[it_058_i][it_058_c] = 1;
						}
						let mut it_058_lim = it_058_nodes;
						it_058_lim = it_058_lim * 2 - 2;
						if it_058_adj_matrix[it_058_sort[it_058_jj]][it_058_i] == 1 && it_058_c != it_058_lim {	// 2n - 2
							it_058_forbidden[it_058_i][it_058_c + 1] = 1;
						}
						if it_058_d2_matrix[it_058_sort[it_058_jj]][it_058_i] == 1 {
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
						let mut d = 0;
						if it_058_color[j] > it_058_color[i] {
							d = it_058_color[j] - it_058_color[i];
						} else {		
							d = it_058_color[i] - it_058_color[j];
						}
						if d < 2 { it_058_flag = 2;}
					}
				}
			}

			if it_058_flag == 2 {
				it_058_lambda += 1;
				if it_058_lambda > lam {
				
					break;
				}
			} else {
				break;
			}
	
		}	

		if (it_058_flag != 2) {	
		println!("forbidden_matrix");
		print!("  ");
		for it_058_j in 0..it_058_lambda {
				print!("{} ", it_058_j);
			}	println!("");
		for it_058_i in 0..it_058_nodes {
			print!("{} ", it_058_i);
			for it_058_j in 0..it_058_lambda {
				print!("{} ", it_058_forbidden[it_058_i][it_058_j]);
			}	println!("");
		}	println!("");
		it_058_forbidden[0][0] = calculate;
		println!("lambda = {}", it_058_lambda);	
		println!("radio coloring");
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





