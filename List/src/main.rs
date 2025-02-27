pub mod list;
pub use list::*;
fn main() {
    let mut node = ListNode { val: 0, next: None };
    print!("Node 1: ");

    let mut node2 = ListNode { val: 1, next: None };
    for i in 1..10 {
        node.add_node(i);
    }
    node.display();
    println!();
    print!("Node 2: ");
    for i in 2..5 {
        node2.add_node(i);
    }
    node2.display();
    println!();
    print!("Result: ");
    let node = ListNode::add_list(Some(Box::new(node)), Some(Box::new(node2)));
    node.unwrap().display();
}
