use std::fs::File;
use std::io::{BufRead, BufReader, Result};


#[derive(Debug)]
struct QuickFindUF {
    count: usize, // count of components (independent graphs)
    id: Vec<i32>, // connected items. index as source and value as destination
}

trait Connected<T>: QuickUnionUF {
    fn new(n: usize) -> T;
    fn connected(&self, p: i32, q: i32) -> bool {
        self.find(p) == self.find(q)
    }

    fn find(&self, pp: i32) -> i32;
    fn union(&mut self, p: i32, q: i32);
}

impl Connected<QuickFindUF> for  QuickFindUF {
    /**
     * Initializes an empty union–find data structure with {@code n} sites
     * {@code 0} through {@code n-1}. Each site is initially in its own
     * component.
     *
     * @param  n the number of sites
     */

    fn new(n: usize) -> QuickFindUF {
        let mut vec: Vec<i32> = Vec::with_capacity(n); // create array in heap
        for i in 0..n {
            vec.push(i as i32) // by default every point connected to itself
        }
        QuickFindUF { count: n, id: vec } // return struct
    }
    // Returns the number of components.
    fn count(&self) -> usize {
        self.count
    }
    // is two points are achievable
    fn connected(&self, p: i32, q: i32) -> bool {
        self.find(p) == self.find(q)
    }
    //  Returns the component identifier for the component containing site {@code p}
    fn find(&self, p: i32) -> i32 {
        self.id[p as usize]
    }
    /**
    * Merges the component containing site {@code p} with the
    * the component containing site {@code q}.
    *
    * @param  p the integer representing one site
    * @param  q the integer representing the other site
    */
    fn union(&mut self, p: i32, q: i32) {
        let p_id: i32 = self.find(p);
        let q_id = self.find(q);
        if p_id == q_id {
            return;
        }
        self.id.iter_mut()
            .filter(|id| **id == p_id)
            .for_each(|id| *id = q_id);
        self.count -= 1;
    }
}

#[derive(Debug)]
struct QuickUnionUF {
    count: usize, // count of components (independent graphs)
    id: Vec<i32>, // connected items. index as source and value as destination
}

impl Connected<QuickUnionUF> for QuickUnionUF  {
    fn new(n: usize) -> QuickUnionUF {
        let mut vec: Vec<i32> = Vec::with_capacity(n); // create array in heap
        for i in 0..n {
            vec.push(i as i32) // by default every point connected to itself
        }
        QuickUnionUF { count: n, id: vec } // return struct
    }
    // Returns the number of components.
    fn count(&self) -> usize {
        self.count
    }
    // is two points are achievable
    fn connected(&self, p: i32, q: i32) -> bool {
        self.find(p) == self.find(q)
    }
    //  Returns the component identifier for the component containing site {@code p}
    fn find(&self, pp: i32) -> i32 {
        let mut p: usize = pp as usize;
        while p as i32 != self.id[p] {
            p = self.id[p] as usize;
        }
        p as i32
    }
    /**
    * Merges the component containing site {@code p} with the
    * the component containing site {@code q}.
    *
    * @param  p the integer representing one site
    * @param  q the integer representing the other site
    */
    fn union(&mut self, p: i32, q: i32) {
        let p_id: i32 = self.find(p);
        let q_id = self.find(q);
        if p_id == q_id {
            return;
        }
        self.id[p_id as usize] = q_id;
        self.count -= 1;
    }
}

#[derive(Debug)]
struct WeightedQuickUnionUF {
    id: Vec<i32>,
    sz: Vec<i32>,
    count: usize,
}

impl WeightedQuickUnionUF {
    fn new(count: usize) -> WeightedQuickUnionUF {
        let mut vec = Vec::with_capacity(count);
        let mut weights = Vec::with_capacity(count);
        for i in 0..count {
            vec.push(i as i32);
            weights.push(1);
        }
        WeightedQuickUnionUF { count, sz: weights, id: vec }
    }

    // Returns the number of components.
    fn count(&self) -> usize {
        self.count
    }
    // is two points are achievable
    fn connected(&self, p: i32, q: i32) -> bool {
        self.find(p) == self.find(q)
    }
    //  Returns the component identifier for the component containing site {@code p}
    fn find(&self, pp: i32) -> i32 {
        let mut p: usize = pp as usize;
        while p as i32 != self.id[p] {
            p = self.id[p] as usize;
        }
        p as i32
    }
    /**
    * Merges the component containing site {@code p} with the
    * the component containing site {@code q}.
    *
    * @param  p the integer representing one site
    * @param  q the integer representing the other site
    */
    fn union(&mut self, p: i32, q: i32) {
        let p_id: usize = self.find(p) as usize;
        let q_id = self.find(q) as usize;
        if p_id == q_id {
            return;
        }
        if self.sz[p_id] < self.sz[q_id] {
            self.id[p_id] = q_id as i32;
            self.sz[q_id] += self.sz[p_id];
        } else {
            self.id[q_id] = p_id as i32;
            self.sz[p_id] += self.sz[q_id];
        }
        self.count -= 1;
    }
}

pub fn run() -> Result<()> {
    let file = File::open("test-examples/uf.txt")?;
    let mut buffer = BufReader::new(file);

    let mut first_line = String::new();
    buffer.read_line(&mut first_line)?;
    let count = first_line.trim_right().parse::<usize>().unwrap();
    let mut uf: QuickFindUF = QuickFindUF::new(count);
    let mut quuf = QuickUnionUF::new(count);
    let mut wquuf = WeightedQuickUnionUF::new(count);
    for line in buffer.lines() {
        let connection: Vec<i32> = line?
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        uf.union(connection[0], connection[1]);
        quuf.union(connection[0], connection[1]);
        wquuf.union(connection[0], connection[1]);
    }
    assert!(uf.connected(6, 7));
    assert!(quuf.connected(6, 7));
    assert!(wquuf.connected(6, 7));
    assert_eq!(uf.count(), 2);
    assert_eq!(quuf.count(), 2);
    assert_eq!(wquuf.count(), 2);
    println!("{:?}", uf);
    println!("{:?}", quuf);
    println!("{:?}", wquuf);
    println!();
    Ok(())
}

