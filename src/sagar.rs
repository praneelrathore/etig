use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io;
use std::fs::File;
use std::io::{BufRead,BufReader,Read,stdin};
use std::env;

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

fn bfs_146 (root: i32) {
    unsafe {

        println!("Hi");
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

pub fn solve_146(graph1: &mut [[i32;350];350],n : i32) {
    unsafe{
    for i in 0..n {
        for j in 0..n {
            graph_146[i as usize][j as usize] = graph1[i as usize][j as usize];
        }
    }
    n_146 = n;
    for i in 0..n {
        for j in 0..n {
            print!("{} ",graph_146[i as usize][j as usize]);
        }
        println!(" ");
    }
    n_146 = n;
    /*    let args:Vec<_> = env::args().collect();
        if args.len()!=2{
            panic!("Error with file reading");
        }
        let file = File::open(&args[1]).expect("Error");
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
            let file1 = File::open(&args[1]).expect("Error");

            n_146 = get_node_146(file1);
            ins+=1;
            println!("Number of nodes : {}",n_146);
            let file2 = File::open(&args[1]).expect("Error");

            let mut k:i32;
            k = get_graph_146(file2);
            ins+=n_146;
            println!("");

            */
            for i in 0..n_146 {
                if visited_146[i as usize] == 0 {
                    println!("For root {}",i);
                    bfs_146(i as i32);
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

fn main () {
    unsafe{
        let mut graph1 : [[i32; 350]; 350] = [[0; 350]; 350];
        let n :i32;
        let args:Vec<_> = env::args().collect();
        if args.len()!=2{
            panic!("Error with file reading");
        }
        let file = File::open(&args[1]).expect("Error");
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
            let file1 = File::open(&args[1]).expect("Error");

            n = get_node_146(file1);
            ins+=1;
            println!("Number of nodes : {}",n_146);
            let file2 = File::open(&args[1]).expect("Error");

            let mut k:i32;
            k = get_graph_146(file2);
            ins+=n_146;
            println!("");

        solve_146(&mut graph1,n);
    }
}
