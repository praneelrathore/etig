use std::io;
use std::thread;

/*fn input() -> i16 {
  let mut input_text = String::new();
  io::stdin()
  .read_line(&mut input_text)
  .expect("failed to read from stdin");

  let trimmed = input_text.trim();
  match trimmed.parse::<i16>() {
    Ok(i) => return i,
    Err(..) =>return -1i16,
  };
}*/

static mut n180: i32 = 0;
static mut MAX_THREADS180: i32 = 10;
static mut thr_count180: i32 = 1;
static mut adj180: [[bool; 500]; 500] = [[false; 500]; 500];
static mut cycle_found180: bool = false;
static mut visited180: [bool; 500] = [false; 500];
static mut visiting180: [bool; 500] = [false; 500];

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

pub fn cyc_det_directed_y2k(matrix: &mut [[i32; 500]; 500], n_tmp: i32) {
  unsafe {
   
    n180 = n_tmp;


    for i in 0..n180 {
		for j in 0..n180 {
    			let x180: i32 = matrix[i as usize][j as usize];

			if (x180 != 0) {
      				adj180[i as usize][j as usize] = true;
			}
		}
    }

    for i in 0..n180 {
      if cycle_found180 {
      	break;
      }
      if !visited180[i as usize] {
        dfs180(i);
      }
    }

    if !cycle_found180 {
      println!("no cycle present");
    } else {
      println!("cycle present");
    }
  }
}

fn dfs180(crr_nod: i32) {
  unsafe {
    if cycle_found180 {
     return;
   }

   let mut children = vec![];
   visiting180[crr_nod as usize] = true;

   for i in 0..n180 {
    if cycle_found180 {
     return;
   }
   
   if adj180[crr_nod as usize][i as usize] {
    if visiting180[i as usize] {
      cycle_found180 = true;            
      return;
    } else if !visited180[i as usize] {
      if thr_count180 < MAX_THREADS180 {
       children.push(thread::spawn(move || {              
        dfs180(i);
      }));
     } else {
      dfs180(i);
    }
  }
}
}

for child in children {
  let _ = child.join();
}

visiting180[crr_nod as usize] = false;
visited180[crr_nod as usize] = true;
}
}
