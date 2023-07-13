#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::io::Write;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use std::fs::File;
use std::io::prelude::*;
// use parking_lot::Mutex;

use rand::Rng;
use sha1::{Sha1, Digest};
use hex;

type Set = Arc<Mutex<HashSet<Vec<u8>>>>;

async fn call_async(n_of_iters: u64) -> Set {

    let out_set: Set = Arc::new(Mutex::new(HashSet::new()));

    let mut handles = Vec::new();

    for i in 0..n_of_iters {
        // Clone to handle the HashSet
        let async_set = out_set.clone();

        handles.push(
            tokio::spawn(async move{
                n(async_set).await
            })
        );
        if (i+1) % 1000 == 0 {
            print!("Seting Task {:.0} / {:.0}\r", i+1, n_of_iters);
        }
    }

    println!();

    let mut i = 0;

    for handle in handles {
        let out = handle.await.unwrap();
        i += 1;
        if (i+1) % 1000 == 0 {
            print!("Done {:?}!\r", (i+1));
        }
    }

    println!("\nDone!");

    out_set
}

async fn n(set_ref: Set) {
    /*
        Function for evaluating a specific situation
    */

    let mut rng = rand::thread_rng();
    let mut hasher = Sha1::new();

    // let bytes = rng.gen::<u16>();
    hasher.update(rng.gen::<u16>().to_string());
    let bytes: Vec<u8> = hasher.finalize().to_vec();

    let mut local_set = set_ref.lock().unwrap();

    if !local_set.contains(&bytes) {
        local_set.insert(bytes);
    }

}

fn write_to_file(text: &String, filename: &String) -> std::io::Result<()>{
    /*
    Writes "text" to the file "filename"
    Args:
        text:
            The text that is being written to the file
        filename:
            The name of the file which the text is being written to
    */

    let mut file = File::create(filename)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() {

    println!("\n\t@some1and2 - Polycube Enumeration\n");

    let the_amount_of_cubes = 3; // result should be 2 when 3

    let out_set = call_async(10_000_000).await; // The Cry function
    let filename = "outfile.set.json"; // filename of the out file

    let _ = write_to_file(
        &format!("{:?}", out_set.lock().unwrap()),
        &String::from(filename)
    );
    println!("Length of List : [ {:?} ]", out_set.lock().unwrap().len());
    println!("\tFINISHED!")
    
}
