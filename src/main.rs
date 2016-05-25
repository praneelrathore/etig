#[allow(unused_mut)]
#[warn(unused_unsafe)]
#[warn(unused_imports)]
#[warn(unused_variables)]

extern crate algorithms;

use std::thread;
use std::sync::{Arc, Mutex};
use std::io;
use std::io::{BufReader,BufRead,Read,stdin};
use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::collections::VecDeque;
use std::time::Duration;
use std::env;

static mut gra: [[i32;350];350] = [[0;350];350];

use algorithms::parallel;

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
            unsafe{gra[i as usize][j as usize]=q;}
          }
          i+=1;
        }
    }
    return k;
}

fn main() {

 unsafe {
//fun::kuch();
	let args:Vec<_> = env::args().collect();
	if args.len()!=2 {
		panic!("Error in reading");
	}

    let file = File::open(&args[1]).expect("Error");
//println!("g");
   let mut na: i32=get_node_173(file);
    let file2  = File::open(&args[1]).expect("Error");
    let mut k:i32;
    k=get_graph_173(file2);
	//println!("{},{}",na,k);


/****************************Uncomment any function to use here****************************************/

  //parallel::bellman_ford_110(&mut gra,na);
  // parallel::dfsutil_034(&mut gra, na);
   //parallel::radiocoloring(&mut gra, na);
  // parallel::graphcoloring(&mut gra, na);
   //parallel::solve_062(&mut gra, na);
   //parallel::check_same_set_105(&mut gra, na);
   //parallel::input_1111(&mut gra,na);
   //parallel::count_ways(&mut gra, na);
   //parallel::conn_comp_073(&mut gra, na);
    //parallel::caller_54(&mut gra, na);
   //parallel::floydwarshall_207(&mut gra, na);
   //parallel::johnson(&mut gra, na);
  //parallel::func(&mut gra, na);
  //parallel::cycle_detection(&mut gra, na);
   //parallel::transitive_closure(&mut gra, na);
   //parallel::check_bipartite(&mut gra, na);
   //parallel::cyc_det_directed_y2k(&mut gra, na);
   //parallel::check_tree(&mut gra, na);
  // parallel::parallel_bfs_146(&mut gra, na);
  //	parallel::solve_070(&mut gra, na);
}


}
