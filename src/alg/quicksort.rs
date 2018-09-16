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

/**
This implementation of trait modifies Vec for i32
*/
impl QuickSort for Vec<i32> {
    /**
     * Rearranges the array in ascending order, using the natural order.
     */
    fn quicksort(&mut self) {
        self::thread_rng().shuffle(self);
        let len = self.len() - 1;
        self.private_sort(0, len)
    }
    // quicksort the subarray from a[lo] to a[hi]
    fn private_sort(&mut self, low: usize, high: usize) {
        if high <= low {
            return;
        }
        let j = self.partition(low, high);
        self.private_sort(low, if j > 0 { j - 1 } else { 0 });
        self.private_sort(j + 1, high);
    }

    // partition the subarray self[lo..hi] so that self[lo..j-1] <= self[j] <= self[j+1..hi]
    // and return the index j.
    fn partition(&mut self, low: usize, high: usize) -> usize {
        let mut i = low;
        let mut j = high + 1;
        let v = self[low];
        loop {
            i += 1;
            // find item on low to swap
            while i != high && self[i] <= v { // TODO optimise code
                i += 1;
            }
            j -= 1;
            // find item on high to swap
            while j != low && self[j] >= v {
                j -= 1;
            }

            // check if pointers cross
            if i >= j {
                break;
            }
            self.exch(i, j);
        }
        // put partitioning item v at self[j]
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
    let mut unsorted_array: Vec<i32> = first_line
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    unsorted_array.quicksort();
    println!("{:?}", unsorted_array);
    Ok(())
}

