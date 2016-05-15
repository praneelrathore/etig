use std::io;
use std::thread;

//extern crate time;
//use time::PreciseTime;

use std::sync::
{
    Arc, Mutex
};
use std::io::
{
    BufReader,BufRead
};
use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::collections::VecDeque;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;




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


static mut adjMatrix_073:[[i32; 500]; 500] = [[0; 500];500];

pub fn conn_comp_073(matrix: &mut [[i32; 500]; 500], n_tmp: i32) -> () {
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
	    let mut test:[[i32; 500]; 500] = [[0; 500];500]; 
	    
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

