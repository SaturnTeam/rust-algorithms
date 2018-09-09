extern crate rand;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use self::rand::{thread_rng, Rng};
trait QuickSort {
    fn quicksort(&mut self) {}
    fn private_sort(&mut self, low: usize, high: usize) {}
    fn partition(&mut self, low: usize, high: usize) -> usize;
    fn exch(&mut self, i: usize, j: usize) {}
}

impl QuickSort for Vec<i32> {
    fn quicksort(&mut self) {
        self::thread_rng().shuffle(self);
        let len = self.len() -1;
        self.private_sort(0, len)
    }
    fn private_sort(&mut self, low: usize, high: usize) {
        if high <= low {
            return;
        }
        let j = self.partition(low, high);
        self.private_sort(low, j - 1);
        self.private_sort(j + 1, high);
    }
    fn partition(&mut self, low: usize, high: usize) -> usize {
        let mut i = low;
        let mut j = high + 1;
        let v = self[low];
        while true {
            i += 1;
            while self[i] <= v {
                i += 1;
                if i == high {
                    break;
                }
            }
            j -= 1;
            while self[j] <= v {
                j -= 1;
                if j == low {
                    break;
                }
            }
            if i >= j{
                break;
            }
            self.exch(i, j)
        }
        self.exch(low, j);
        return j;
    }
    fn exch(&mut self, i: usize, j: usize) {
        let mut a = self[i];
        self[i] = self[j];
        self[j] = a;
    }
}

pub fn run() -> Result<()> {
    let file = File::open("test-examples/quicksort.txt")?;
    let mut buffer = BufReader::new(file);
    let mut first_line = String::new();
    buffer.read_line(&mut first_line)?;
//    for line in buffer.lines() {
    let mut unsorted_array: Vec<i32> = first_line
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
//    }
    unsorted_array.quicksort();
    println!("{:?}", unsorted_array);
//    assert!(uf.connected(6, 7));
//    assert!(quuf.connected(6, 7));
//    assert!(wquuf.connected(6, 7));
//    assert_eq!(uf.count(), 2);
//    assert_eq!(quuf.count(), 2);
//    assert_eq!(wquuf.count(), 2);
//    println!("{:?}", uf);
//    println!("{:?}", quuf);
//    println!("{:?}", wquuf);
//    println!();
    Ok(())
}

