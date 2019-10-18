type Process = (String, i32);
type Processes = Vec<(String, i32)>;

struct Queue {
    data: Processes,
    head: usize,
    tail: usize,
    max: usize
}

impl Queue {
    fn new(size: usize) -> Queue {
        Queue {
            data: vec![(String::default(), 0); size],
            head: 0,
            tail: 0,
            max: size
        }
    }
    fn is_empty(&self) -> bool {
        self.head == self.tail
    }
    fn is_full(&self) -> bool {
        self.head == (self.tail + 1) % self.max
    }
    fn enqueue(&mut self, p: &Process) {
        if self.is_full() {
            panic!("queue overflow");
        }
        self.data[self.tail] = p.clone();
        if self.tail + 1 == self.max {
            self.tail = 0;
        } else {
            self.tail += 1;
        }
    }
    fn dequeue(&mut self) -> Process {
        if self.is_empty() {
            panic!("queue underflow");
        }
        let p = self.data[self.head].clone();
        if self.head + 1 == self.max {
            self.head = 0;
        } else {
            self.head += 1;
        }

        p
    }
}

pub fn run(n: usize, q: i32, p: Processes) -> Processes {
    let mut queue = Queue::new(n+1);
    let mut results: Processes = Vec::new();
    let mut p_time = 0;
    for i in 0..n {
        queue.enqueue(&p[i]);
    }
    while !queue.is_empty() {
        let mut process = queue.dequeue();
        if process.1 <= q {
            results.push((process.0, p_time+process.1));
            p_time += process.1;
        } else {
            process.1 -= q;
            queue.enqueue(&process);
            p_time += q;
        }
    }

    results
}
