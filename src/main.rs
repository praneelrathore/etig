extern crate etig;
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

static mut gra: [[i32;500];500] = [[0;500];500];

use etig::samarth;
use etig::praneel;
use etig::vaibhav;
use etig::eti11;
use etig::yash;
use etig::d_raj_radio_coloring;
use etig::d_raj_graph_coloring;
use etig::rahlav;
use etig::apurv;
use etig::rahul_073;
use etig::kamal;
use etig::akash_floydwarshall;
use etig::sameer;
use etig::hima;
use etig::cycle_detection;

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

   //samarth::bellman_ford_110(&mut gra,na);
   //praneel::dfsutil_034(&mut gra, na);
   //vaibhav::solve_bfs(&mut gra, na);
   //d_raj_radio_coloring::radiocoloring(&mut gra, na);
   //d_raj_graph_coloring::graphcoloring(&mut gra, na);
   //rahlav::solve_062(&mut gra, na);
   //yash::caller_105(&mut gra, na);
   //eti11::input(&mut gra,na);
   //apurv::count_ways(&mut gra, na);
   //rahul_073::conn_comp_073(&mut gra, na);
   //kamal::caller_54(&mut gra, na);
   //akash_floydwarshall::floydwarshall_207(&mut gra, na);
   //sameer::johnson(&mut gra, na);
   //hima::func(&mut gra, na);
   //cycle_detection::cycle_detection(&mut gra, na);
}
    
	
}
