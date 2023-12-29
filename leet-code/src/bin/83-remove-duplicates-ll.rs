
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = head.as_mut();

        while let Some(node) = current {

            while let Some(next) = &mut node.next /* .as_mut() is fine too */{
                if node.val == next.val {
                    node.next = next.next.take()
                } else {
                    break;
                }
            }

            current = node.next.as_mut();
        }

        head
    }
}

fn main() {
    // create linked list head 1 1 2
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(2))),
        })),
    }));

    println!("{:?}", head);

    let res = Solution::delete_duplicates(head);

    println!("{:?}", res);
}