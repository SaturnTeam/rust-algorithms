extern crate rand;

use std::fs::File;
use std::io::{BufRead, BufReader};
use self::rand::{thread_rng, Rng};
use std::io;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError { details: msg.to_string() }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

/**
 *  The {@code MaxPQ} class represents a priority queue of generic keys.
 *  It supports the usual <em>insert</em> and <em>delete-the-maximum</em>
 *  operations, along with methods for peeking at the maximum key,
 *  testing if the priority queue is empty, and iterating through
 *  the keys.
 *  <p>
 *  This implementation uses a binary heap.
 *  The <em>insert</em> and <em>delete-the-maximum</em> operations take
 *  logarithmic amortized time.
 *  The <em>max</em>, <em>size</em>, and <em>is-empty</em> operations take constant time.
 *  Construction takes time proportional to the specified capacity or the number of
 *  items used to initialize the data structure.
 *  <p>
 *  For additional documentation, see <a href="https://algs4.cs.princeton.edu/24pq">Section 2.4</a> of
 *  <i>Algorithms, 4th Edition</i> by Robert Sedgewick and Kevin Wayne.
 */
#[derive(Debug)]
struct MaxPQ {
    pq: Vec<i32>,
    n: usize,
}

impl MaxPQ {
    /**
     * Initializes an empty priority queue with the given initial capacity.
     *
     * @param  initCapacity the initial capacity of this priority queue
     */
    fn new(init_capacity: usize) -> MaxPQ {
        let mut pq = Vec::with_capacity(init_capacity + 1);
        pq.push(-1);
        MaxPQ { pq, n: 0 }
    }

    /**
     * Returns true if this priority queue is empty.
     */
    fn is_empty(&self) -> bool {
        self.n == 0
    }

    /**
     * Returns the number of keys on this priority queue.
     */
    fn size(&self) -> usize {
        self.n
    }

    /**
     * Returns a largest key on this priority queue.
     */
    fn max(&self) -> Result<i32, MyError> {
        if self.is_empty() {
            Err(MyError::new("Priority queue underflow"))
        } else {
            Ok(self.pq[1])
        }
    }
    /**
     * Removes and returns a largest key on this priority queue.
     *
     * @return a largest key on this priority queue
     */
    fn del_max(&mut self) -> Result<i32, MyError> {
        if self.is_empty() {
            Err(MyError::new("Priority queue underflow"));
        }
        let max = self.pq[1];
        self.exch(1, self.n);
        self.n -= 1;
        self.sink(1);
        self.pq[self.n + 1] = -1;
        Ok(max)
    }
    /**
     * Adds a new key to this priority queue.
     */
    fn insert(&mut self, x: i32) {
        self.n += 1;
        self.pq.push(x);
        let i = self.n;
        self.swim(i);
    }
    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.pq[(k / 2) as usize] < self.pq[k as usize] {
            self.exch(k, k / 2);
            k = k / 2;
        }
    }
    fn sink(&mut self, mut k: usize) {
        while 2*k <= self.n {
            let mut j = 2 * k;
            if j < self.n && self.pq[j] < self.pq[j + 1] {
                j += 1;
            }
            if self.pq[k] >= self.pq[j] {
                break;
            }
            self.exch(k, j);
            k = j;
        }
    }
    fn exch(&mut self, i: usize, j: usize) {
        let t = self.pq[i as usize];
        self.pq[i as usize] = self.pq[j as usize];
        self.pq[j as usize] = t;
    }
}

pub fn run() -> io::Result<()> {
    let file = File::open("test-examples/quicksort.txt")?;
    let mut buffer = BufReader::new(file);
    let mut first_line = String::new();
    buffer.read_line(&mut first_line)?;
    let mut unsorted_array: Vec<i32> = first_line
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
//    unsorted_array.quicksort();
//    println!("{:?}", unsorted_array);
    let mut pq = MaxPQ::new(5);
    let mut max;
    pq.insert(80);
    pq.insert(81);
    pq.insert(69);
    max = pq.del_max().unwrap(); // 81
    assert_eq!(max, 81);
    pq.insert(88);
    pq.insert(65);
    pq.insert(77);
    max = pq.del_max().unwrap(); // 88
    assert_eq!(max, 88);
    pq.insert(80);
    pq.insert(76);
    pq.insert(69);
    max = pq.del_max().unwrap(); // 80
    assert_eq!(max, 80);

//    for un in unsorted_array {
//        pq.isnert(un)
//    }
    println!("{:?}", pq);
    assert!(!pq.is_empty());
    assert_eq!(pq.size(), 5);
    Ok(())
}

