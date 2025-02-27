// Definition for a binary tree node.
use std::cell::{Ref, RefCell};
use std::cmp::max;
use std::collections::VecDeque;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn get_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();
        return std::cmp::max(Self::get_depth(left), Self::get_depth(right)) + 1;
    }

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }
        let root = root.unwrap();
        let left = Self::invert_tree(root.borrow().left.clone());
        let right = Self::invert_tree(root.borrow().right.clone());

        root.borrow_mut().left = right;
        root.borrow_mut().right = left;

        Some(root)
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut maximum = 0;
        let root = root.unwrap();

        fn traverse(root: Option<Rc<RefCell<TreeNode>>>, mut maximum: i32) -> (i32, i32) {
            if root.is_none() {
                return (0, maximum);
            }

            let root = root.unwrap();
            let left = traverse(root.borrow().left.clone(), maximum);
            let right = traverse(root.borrow().right.clone(), maximum);

            maximum = max(maximum, (left.0 + right.0).abs());
            return (max(left.0, right.0) + 1, maximum);
        }
        //Replace placeholder return value with actual code later on

        let left = traverse(root.borrow().left.clone(), maximum.clone());
        let right = traverse(root.borrow().right.clone(), maximum.clone());
        maximum = max(left.1, right.1);

        return max(maximum, (left.0 + right.0).abs());
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn traverse(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if root.is_none() {
                return 0;
            }
            let root = root.unwrap();
            let left = traverse(root.borrow().left.clone());
            if left == -1 {
                return -1;
            }
            let right = traverse(root.borrow().right.clone());
            if right == -1 {
                return -1;
            }
            if (left - right).abs() <= 1 {
                max(left, right) + 1
            } else {
                -1
            }
        }
        traverse(root) != -1
    }

    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.is_none() && !q.is_none() || !p.is_none() && q.is_none() {
            return false;
        }
        let p = p.clone().unwrap();
        let q = q.clone().unwrap();
        let left = Self::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone());
        if !left {
            return left;
        }
        let right = Self::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone());
        if !right {
            return right;
        }
        return p.borrow().val == q.borrow().val;
    }

    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let arr: Rc<RefCell<Vec<Option<Rc<RefCell<TreeNode>>>>>> = Rc::new(RefCell::new(vec![]));

        fn traverse(
            root: Option<Rc<RefCell<TreeNode>>>,
            n: i32,
            arr: Rc<RefCell<Vec<Option<Rc<RefCell<TreeNode>>>>>>,
        ) {
            if root.is_none() {
                return;
            }
            let root = root.unwrap();
            if root.borrow().val == n {
                arr.borrow_mut().push(Some(root.clone()));
            }
            traverse(root.borrow().left.clone(), n, arr.clone());
            traverse(root.borrow().right.clone(), n, arr.clone());
        }

        traverse(root, sub_root.clone().unwrap().borrow().val, arr.clone());
        for node in arr.borrow().iter() {
            if Self::is_same_tree(node.clone(), sub_root.clone()) {
                return true;
            }
        }
        return false;
    }

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root.unwrap();
        let p = p.unwrap();
        let q = q.unwrap();

        if root.borrow().val == p.borrow().val || root.borrow().val == q.borrow().val {
            return Some(root);
        }

        if root.borrow().val > p.borrow().val && root.borrow().val > q.borrow().val {
            Self::lowest_common_ancestor(Some(root), Some(p), Some(q))
        } else if root.borrow().val < p.borrow().val && root.borrow().val < q.borrow().val {
            Self::lowest_common_ancestor(Some(root), Some(p), Some(q))
        } else {
            Some(root)
        }
    }

    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        if root.is_none() {
            return ans;
        }

        ans.push(vec![root.clone().unwrap().borrow().val]);
        let mut dq = VecDeque::new();
        dq.push_back(root);

        while !dq.is_empty() {
            let mut level = Vec::new();
            for _ in 0..dq.len() {
                let node = dq.pop_front().unwrap().unwrap();
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();

                if left.is_some() {
                    dq.push_back(left.clone());
                    level.push(left.unwrap().borrow().val);
                }

                if right.is_some() {
                    dq.push_back(right.clone());
                    level.push(right.unwrap().borrow().val);
                }
            }
            if !level.is_empty() {
                ans.push(level);
            }
        }

        ans
    }

    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if root.is_none(){
            return ans;
        }

        fn get_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            let mut ans = Vec::new();
            ans.push(vec![root.clone().unwrap().borrow().val]);
            let mut dq = VecDeque::new();
            dq.push_back(root);
            let mut level:Vec<i32>;

            while !dq.is_empty() {
                level = vec![];
                for _ in 0..dq.len(){
                    let node = dq.pop_front().unwrap().unwrap();
                    let left = node.borrow().left.clone();
                    let right = node.borrow().right.clone();
                    if let Some(node) = left{
                        level.push(node.borrow().val);
                        dq.push_back(Some(node));
                    }
                    if let Some(node) = right{
                        level.push(node.borrow().val);
                        dq.push_back(Some(node));
                    }
                }
                if level.is_empty(){
                    continue;
                }
                ans.push(level);
            }

            ans
        }
        let level_order = get_level_order(root);
        for list in level_order{
            ans.push(*list.last().unwrap());
        }
        ans
    }

    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn count_good_nodes(root: Option<Rc<RefCell<TreeNode>>>, largest: i32) -> i32 {
            if root.is_none(){
                return 0;
            }
            let root = root.unwrap();
            let left = count_good_nodes(root.borrow().left.clone(), max(largest, root.borrow().val));
            let right = count_good_nodes(root.borrow().right.clone(), max(largest, root.borrow().val));
            if root.borrow().val >= largest{
                return left + right + 1;
            }
            else {
                return left + right;
            }
        }
        count_good_nodes(root.clone(), root.unwrap().borrow().val)
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut count = k;
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        let mut root = root;

        'main: loop {
            while root.is_some() {
                stack.push(root.clone());
                root = root.unwrap().borrow().left.clone();
            }

            count -= 1;
            root = stack.pop().unwrap();
            if count == 0 {
                break 'main;
            }

            root = root.unwrap().borrow().right.clone();
        }

        root.unwrap().borrow().val
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut preorder = VecDeque::from(preorder);

        fn build(preorder:&mut VecDeque<i32>, inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() || inorder.is_empty() {
                return None;
            }

            let mut root = TreeNode::new(preorder.pop_front().unwrap());
            let index = inorder.iter().position(|&x| x == root.val).unwrap();

            root.left = build(preorder, &inorder[..index]);
            root.right = build(preorder, &inorder[index+1..]);

            Some(Rc::new(RefCell::new(root)))
        }

        build(&mut preorder, &inorder[..])
    }
}
