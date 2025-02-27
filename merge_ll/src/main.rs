pub mod linked_list;
pub use linked_list::*;

fn main() {
    let mut list1 = Some(Box::new(ListNode::new(1)));
    let mut list2 = Some(Box::new(ListNode::new(1)));

    let mut current1 = &mut list1;
    let mut current2 = &mut list2;

    for i in 2..5 {
        if i % 2 == 0 {
            if let Some(ref mut node) = current1 {
                node.next = Some(Box::new(ListNode::new(i)));
                current1 = &mut node.next;
            }
        } else {
            if let Some(ref mut node) = current2 {
                node.next = Some(Box::new(ListNode::new(i)));
                current2 = &mut node.next;
            }
        }

        if i == 4 {
            if let Some(ref mut node) = current2 {
                node.next = Some(Box::new(ListNode::new(i)));
                current2 = &mut node.next;
            }
        }
    }
    // if let Some(ref node) = list1.take() {
    //     println!("List 1: {}", ListNode::as_string(node));
    // }
    // if let Some(ref node) = list2.take() {
    //     println!("List 2: {}", ListNode::as_string(node));
    // }
    // while let Some(node) = list1.take() {
    //     print!("{}", node.val);
    // }
    let combo = merge_two_lists(list1, list2);
    if let Some(ref node) = combo {
        println!("Combo: {}", ListNode::as_string(node));
    }
}
