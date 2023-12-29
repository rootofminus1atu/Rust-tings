#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_list = ListNode::new(0);
        let mut current = &mut new_list;
    
        while let (Some(node1), Some(node2)) = (&l1, &l2) {
            if node1.val < node2.val {
                current.next = l1.take();
                current = current.next.as_mut().unwrap();
                l1 = current.next.take()
            } else {
                current.next = l2.take();
                current = current.next.as_mut().unwrap();
                l2 = current.next.take();
            }
        }

        current.next = l1.or(l2);

        new_list.next
    }
}





fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(4))),
        })),
    }));
    println!("{:?}", l1);

    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode::new(3)))
    }));
    println!("{:?}", l2);

    let res = Solution::merge_two_lists(l1, l2);
    println!("{:?}", res);
}