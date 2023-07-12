#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
// use parking_lot::Mutex;

use rand::Rng;
use sha1::{Sha1, Digest};
use hex;

type Set = Arc<Mutex<HashSet<u16>>>;


async fn call_n(n_of_iters: u64) -> Set {

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
        if i % 1000 == 0 {
            print!("Allocated {:.0} / {:.0}\r", i+1, n_of_iters);
        }
    }

    println!();

    let mut i = 0;

    for handle in handles {
        let out = handle.await.unwrap();
        i += 1;
        if i % 1000 == 0 {
            print!("Done {:?}!\r", i);
        }
    }

    println!("\nDone!");

    out_set
}

async fn n(set_ref: Set) -> u16 {


    let mut rng = rand::thread_rng();
    let mut hasher = Sha1::new();

    let bytes = rng.gen::<u16>();

    let mut local_set = set_ref.lock().unwrap();

    if !local_set.contains(&bytes) {
        local_set.insert(bytes);
        return bytes;
    }

    return 0u16;
}

#[tokio::main]
async fn main() {

    let out_set = call_n(10_000_000).await;

    println!("{:?}", out_set.lock().unwrap());
    
}
