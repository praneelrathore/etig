use std::thread;
use std::sync::{Arc, Mutex};
use std::io;
use std::io::{BufReader,BufRead,Read,stdin};
use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::collections::VecDeque;
use std::time::Duration;

static mut dist_110: [i32; 500] = [9999999; 500];
static mut source_110: [i32; 500] = [0; 500];
static mut dest_110: [i32; 500] = [0; 500];
static mut weight_110: [i32; 500] = [0; 500];
static mut adj_110: [[i32; 500]; 500] = [[0; 500]; 500];
pub fn bellman_ford_110(mat_110: &mut [[i32; 500]; 500], n: i32) {
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
