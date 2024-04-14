// 203. Remove Linked List Elements

fn main() {
    // Create a linked list
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: None,
                            })),
                        })),
                    })),
                })),
            })),
        })),
    }));

    let val = 6;

    // Remove all elements with value val
    let result = Solution::remove_elements(head, val);

    // Print the result of the linked list
    let mut walker = &result;
    print!("[");
    loop {
        match walker {
            None => break,
            Some(node) => {
                print!("{}", node.val);
                walker = &node.next;
                if walker.is_some() {
                    print!(", ");
                }
            },
        }
    }
    println!("]");
}

// Definition for singly-linked list.
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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut walker = &mut head;

        loop {
            match walker {
                None => break,
                Some(node) if node.val == val => {
                    *walker = node.next.take();
                },
                Some(node) => {
                    walker = &mut node.next;
                },
            }
        }
        head
    }
}