// https://leetcode.com/problems/split-linked-list-in-parts/
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    fn len(head: &Option<Box<ListNode>>) -> usize {
        match &head {
            None => 0,
            Some(head) => len(&head.next) + 1,
        }
    }
    let k = k as usize;
    let n = len(&head);
    let avg = n / k;
    let extra = n % k;

    let mut result = vec![None; k];
    result[0] = head;
    let mut temp = &mut result[0];

    for i in 1..k {
        let size = (if (i - 1) < extra { avg + 1 } else { avg });
        for j in 0..size {
            temp = &mut temp.as_mut().unwrap().next;
        }
        result[i] = temp.take();
        temp = &mut result[i];
    }
    result
}
