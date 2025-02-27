// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn as_string(node: &Box<ListNode>) -> String {
        if node.next.is_none() {
            return format!("{}", node.val);
        }
        if let Some(ref next_node) = node.next {
            format!("{} -> {}", node.val, ListNode::as_string(next_node))
        } else {
            String::from("")
        }
    }

    pub fn add(&mut self, val: i32) {
        let mut current_node = &mut self.next;

        while let Some(ref mut node) = current_node {
            if node.next.is_none() {
                node.next = Some(Box::new(ListNode::new(val)));
                break;
            }
            current_node = &mut node.next;
        }
    }

    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut current = &mut head;

        while let (Some(mut node1), Some(mut node2)) = (list1.take(), list2.take()) {
            if node1.val <= node2.val {
                list1 = node1.next.take();
                *current = Some(node1);
            } else {
                list2 = node2.next.take();
                *current = Some(node2);
            }
            current = &mut current.as_mut().unwrap().next;
        }

        *current = if list1.is_some() { list1 } else { list2 };

        head.unwrap().next
    }

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for list in lists {
            head = Self::merge_two_lists(head, list);
        }
        head
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut tail = &mut head;

    let mut list1 = list1;
    let mut list2 = list2;

    while let (Some(mut node1), Some(mut node2)) = (list1.take(), list2.take()) {
        if node1.val <= node2.val {
            list1 = node1.next.take();
            *tail = Some(node1);
        } else {
            list2 = node2.next.take();
            *tail = Some(node2);
        }
        tail = &mut tail.as_mut().unwrap().next;
    }
    *tail = if list1.is_some() { list1 } else { list2 };
    return head;
}
