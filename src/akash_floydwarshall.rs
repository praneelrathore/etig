use std::io;
use std::thread;
static mut dist_207:[[i32; 500]; 500] = [[0; 500]; 500];

pub fn floydwarshall_207(inputx: &mut [[i32; 500]; 500],n: i32) {                               
                               
  unsafe{
  let NTHREADS: i32 = 4; 
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
