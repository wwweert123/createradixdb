use std::convert::TryInto;

use anyhow::{Error, Ok};
use radixdb::{store::{BlobStore, PagedFileStore}, RadixTree};

use std::fs;

const LOW_NAMES: &[&str] = &[
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS_NAMES: &[&str] = &[
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const BIG_NAMES: &[&str] = &["thousand", "million", "billion"];

/**
 * Converts an integer number into words (american english).
 * @author Christian d'Heureuse, Inventec Informatik AG, Switzerland, www.source-code.biz
 */
fn number_to_words(n0: i64) -> String {
    let mut n = n0;
    if n < 0 {
        "minus ".to_string() + &number_to_words(-n)
    } else if n <= 999 {
        convert999(n)
    } else {
        let mut s: String = "".to_string();
        let mut t: usize = 0;
        while n > 0 {
            if n % 1000 != 0 {
                let mut s2 = convert999(n % 1000);
                if t > 0 {
                    s2 = s2 + " " + BIG_NAMES[t - 1];
                }
                if s.is_empty() {
                    s = s2
                } else {
                    s = s2 + ", " + &s;
                }
            }
            n /= 1000;
            t += 1;
        }
        s
    }
}

fn convert999(n: i64) -> String {
    let s1 = LOW_NAMES[(n / 100) as usize].to_string() + " hundred";
    let s2 = convert99(n % 100);
    if n <= 99 {
        s2
    } else if n % 100 == 0 {
        s1
    } else {
        s1 + " " + &s2
    }
}

fn convert99(n: i64) -> String {
    if n < 20 {
        LOW_NAMES[n as usize].to_string()
    } else {
        let s = TENS_NAMES[(n / 10 - 2) as usize].to_string();
        if n % 10 == 0 {
            s
        } else {
            s + "-" + LOW_NAMES[(n % 10) as usize]
        }
    }
}

fn generate_new_tree(file_path: &str) -> anyhow::Result<RadixTree<PagedFileStore>> {
    let file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(file_path)?;

    let store = PagedFileStore::new(file, 1024 * 1024)?;
    let mut tree = RadixTree::empty(store.clone());
    Ok(tree)
}

fn generate_test_1() -> anyhow::Result<bool> {
    let path = "test1.rdb";
    let mut tree = generate_new_tree(path)?;
    tree.try_insert("helloworld", "hi");
    tree.try_insert("helloworld1", "hi");
    tree.try_insert("hi", "hi");
    let final_id = tree.try_reattach()?;
    println!("{:?}", final_id);
    Ok(true)
}

fn generate_test_2() -> anyhow::Result<bool> {
    let path = "test2.rdb";
    let mut tree = generate_new_tree(path)?;
    for i in 0..1_000 {
        let key = number_to_words(i);
        let value = i.to_string().repeat(10000);
        tree.try_insert(key, value)?;
    }
    let final_id = tree.try_reattach()?;
    println!("{:?}", final_id);
    Ok(true)
}

fn generate_test_3() -> anyhow::Result<bool> {
    let path = "test3.rdb";
    let mut tree = generate_new_tree(path)?;
    for i in 0..10_000 {
        let key = number_to_words(i);
        let value = i.to_string().repeat(10000);
        tree.try_insert(key, value)?;
    }
    let final_id = tree.try_reattach()?;
    println!("{:?}", final_id);
    Ok(true)
}



fn main() -> anyhow::Result<()> {
    let _result = generate_test_1();
    let _result = generate_test_2();
    let _result = generate_test_3();

    // let final_size = u64::from_be_bytes(final_id[0..8].try_into()?);
    // for e in tree.try_iter() {
    //     let (k, v) = e?;
    //     let v = v.load(&store)?;
    //     println!("{:?} {:?}", std::str::from_utf8(k.as_ref())?, v.len());
    // }
    // println!("{:?}", final_id);
    // println!("final size {}", final_size);
    // println!("{:?}", store.read(&final_id).unwrap().to_vec());
    //     let store = PagedFileStore::new()?;
    Ok(())

}

// Code referenced from: https://github.com/cloudpeers/radixdb/blob/master/examples/large_tree.rs