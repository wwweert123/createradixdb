use radixdb::{store::{BlobStore, MemStore, PagedFileStore}, RadixTree};

use rand::{distributions::{Alphanumeric, Uniform}, Rng}; // 0.8

use std::fs;

use parking_lot::Mutex;
use std::{io::Write, sync::Arc};
#[derive(Debug, Clone)]
pub struct VecStore(Arc<Mutex<Vec<u8>>>);


fn main() {
    let file = match fs::OpenOptions::new()
        .create(false)
        .read(true)
        .write(true)
        .open("hello.radixdb")
    {
        Ok(file) => file,
        Err(e) => return {
            println!("cannot open file")
        }
    };

    // open file into store
    let store = match PagedFileStore::new(file, 1024 * 1024) {
        Ok(store) => store,
        Err(e) => return {
            println!("cannot open file")
        }
    };

    let last_id = match store.last_id() {
        Some(last_id) => last_id,
        None => return {
            println!("cannot open file")
        }
    };

    match RadixTree::try_load(store.clone(), store.last_id()) {
        Ok(tree) => {
            println!("read ok")
        }
        Err(e) => {
            println!("failed");
        },
    }
}

// Code referenced from: https://github.com/cloudpeers/radixdb/blob/master/examples/large_tree.rs