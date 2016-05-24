use std::path::Path;
use std::thread;
use std::sync::{Arc, Mutex};

static NTHREADS: usize = 4;
static mut CYCLE: bool = false;
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
                CYCLE = true;
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
        while loop_start <= nodes && loop_start <= (NTHREADS) {

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
        if CYCLE{
            println!("Output        :   Not a tree");
        }else {
            println!("Output        :   Tree");
        }
    }

}
