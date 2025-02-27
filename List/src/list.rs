use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Definition for singly-linked list.
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
    pub prev: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            prev: None,
            val,
        }
    }

    fn display(node: Option<Rc<RefCell<ListNode>>>) {
        let mut current = node.clone();
        while let Some(node) = current {
            let node = node.clone();
            print!("-> {} ", node.borrow().val);
            current = node.borrow().next.clone();
        }
    }
}
// impl ListNode {
//     pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
//         let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
//         let mut length = 0;
//         let mut current = dummy.as_ref();

//         // Calculate the length of the list
//         while let Some(node) = current {
//             current = node.next.as_ref();
//             length += 1;
//         }

//         let target = length - n; // Node to remove (1-based index)
//         let mut current = dummy.as_mut();

//         // Navigate to the node before the target
//         for _ in 0..target {
//             if let Some(node) = current {
//                 current = node.next.as_mut();
//             }
//         }

//         // Remove the target node
//         if let Some(node) = current {
//             node.next = node
//                 .next
//                 .as_mut()
//                 .and_then(|next_node| next_node.next.take());
//         }

//         dummy.unwrap().next
//     }

//     pub fn add_node(&mut self, val: i32) -> Result<(), Box<dyn Error>> {
//         let mut current = self;
//         while let Some(ref mut nextNode) = current.next {
//             current = nextNode;
//         }
//         current.next = Some(Box::new(ListNode::new(val)));

//         Ok(())
//     }

//     pub fn display(&self) -> Result<(), Box<dyn Error>> {
//         let mut current = self;
//         print!("{}", self.val);
//         while let Some(ref node) = current.next {
//             print!(" -> {}", node.val);
//             current = node;
//         }
//         Ok(())
//     }

//     pub fn add_list(
//         mut l1: Option<Box<ListNode>>,
//         mut l2: Option<Box<ListNode>>,
//     ) -> Option<Box<ListNode>> {
//         let mut dummy = ListNode { val: 0, next: None };
//         let mut carry = 0;
//         let mut sum = 0;
//         let mut current = &mut dummy;

//         while l1.is_some() || l2.is_some() || carry != 0 {
//             sum = match (l1.as_ref(), l2.as_ref()) {
//                 (Some(node1), Some(node2)) => node1.val + node2.val + carry,
//                 (Some(node), None) => node.val + carry,
//                 (None, Some(node)) => node.val + carry,
//                 _ => carry,
//             };

//             carry = sum / 10;
//             current.next = Some(Box::new(ListNode {
//                 val: sum % 10,
//                 next: None,
//             }));
//             current = current.next.as_mut().unwrap();

//             l1 = l1.and_then(|node| node.next);
//             l2 = l2.and_then(|node| node.next);
//         }

//         return dummy.next;
//     }
// }

// pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
//     for num in nums.clone() {
//         if nums[num.abs() as usize - 1] < 0 {
//             return num.abs();
//         }
//         nums[num.abs() as usize - 1] *= -1;
//     }
//     0
// }

pub struct LRUCache {
    cache: HashMap<i32, Rc<RefCell<ListNode>>>,
    max_capacity: i32,
    used_capacity: i32,
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            cache: HashMap::with_capacity(capacity as usize),
            max_capacity: capacity,
            used_capacity: 0,
            head: None,
            tail: None,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let node = self.cache.get(&key);
        if let Some(node) = node {
            let node = node.clone();
            self.make_head(node.clone());
            return node.borrow().val;
        }
        return -1;
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get(&key) {
            let node = node.clone();
            node.borrow_mut().val = value;
            self.make_head(node);
        } else {
            let node = Rc::new(RefCell::new(ListNode::new(value)));
            if self.used_capacity < self.max_capacity {
                self.used_capacity += 1;
            } else {
                let tail = self.tail.clone().unwrap();
                self.remove_node(tail.clone());
                self.cache.remove(&tail.borrow().val);
            }
            self.add_node(node.clone());
            self.cache.insert(key, node);
        }
    }

    fn add_node(&mut self, node: Rc<RefCell<ListNode>>) {
        if self.head.is_none() {
            self.head = Some(node);
            self.tail = self.head.clone();
        } else {
            let head = self.head.clone().unwrap();
            head.borrow_mut().prev = Some(node.clone());
            node.borrow_mut().next = Some(head);
            self.head = Some(node);
        }
    }

    fn remove_node(&mut self, node: Rc<RefCell<ListNode>>) {
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        if let Some(prev) = prev.clone() {
            prev.borrow_mut().next = next.clone();
        } else {
            self.head = next.clone();
        }

        if let Some(next) = next {
            next.borrow_mut().prev = prev.clone();
        } else {
            self.tail = prev.clone();
        }
    }

    fn make_head(&mut self, node: Rc<RefCell<ListNode>>) {
        self.remove_node(node.clone());
        self.add_node(node);
    }
}
