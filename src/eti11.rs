use std::thread;
use std::sync::{Arc, Mutex};
use std::io;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::collections::VecDeque;
use std::time::Duration;
extern crate time;

//Main Parallel
fn input_25() -> i32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,//println!("your integer input: {}", i),
        Err(..) =>return 0,//println!("this was not an integer: {}", trimmed)
    };
    return 0;
}

fn timestamp () -> f64 {
    let timespec = time::get_time();
    // 1459440009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0 );
    mills
}

pub fn input(mat : & mut [[i32;500];500], n:i32) -> bool{
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
	
	   if v.len() > 500 {
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
