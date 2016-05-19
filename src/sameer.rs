//rustc  new.rs -A warnings
use std::io;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::thread;
use std::process::Command;
use std::str::FromStr;

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
