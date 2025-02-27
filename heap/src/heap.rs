use std::collections::{BinaryHeap, HashMap};
struct KthLargest {
    k: i32,
    minheap: BinaryHeap<i32>
}

#[derive(PartialEq, Eq)]
struct ListWithPriority{
    priority: i32,
    list: Vec<i32>
}
impl Ord for ListWithPriority{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for ListWithPriority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut minheap = BinaryHeap::from(nums.iter().map(|&x| -x).collect::<Vec<i32>>());
        while minheap.len() > k as usize {
            minheap.pop();
        }
        Self {
            k,
            minheap
        }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        let last = -*self.minheap.peek().unwrap();
        if self.minheap.len() < self.k as usize{
            self.minheap.push(-val);
        }
        else if val > last {
            self.minheap.pop();
            self.minheap.push(-val);
        }

        -*self.minheap.peek().unwrap()
    }

    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        for stone in stones{
            heap.push(-stone);
        }
        let mut current: i32;
        while heap.len() > 1 {
            current = -heap.pop().unwrap();
            if current == -*heap.peek().unwrap(){
                heap.pop();
            } else {
                current -= -heap.pop().unwrap();
                heap.push(-current);
            }
        }

        heap.pop().unwrap_or_else(|| 0)
    }

    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();

        for point in points{
            heap.push(ListWithPriority{priority: point[0]^2 + point[1]^2, list: point});
            if heap.len() > k as usize{
                heap.pop();
            }
        }

        heap.into_iter().map(|e| e.list).collect()
    }

    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for num in nums{
            if heap.len() < k as usize{
                heap.push(-num);
            }
            else if *heap.peek().unwrap() > -num {
                heap.pop();
                heap.push(-num);
            } else {
                continue;
            }
        }
        -*heap.peek().unwrap()
    }

    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counter = HashMap::new();
        tasks.iter().for_each(|task| *counter.entry(task).or_insert(0) += 1);
        let max_count = *counter.values().max().unwrap();
        let max_tasks = counter.iter().filter_map(|(_, val)| if *val == max_count {Some(1)} else {None}).count();

        return tasks.len().max((max_count - 1) as usize * (n as usize + 1) + max_tasks) as i32;
    }
}
