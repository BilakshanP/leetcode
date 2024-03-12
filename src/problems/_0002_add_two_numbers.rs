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

pub fn add_two_numbers_1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn add_two_numbers_internal(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carry == 0 {
            return None;
        }

        let l1 = l1.unwrap_or(Box::new(ListNode::new(0)));
        let l2 = l2.unwrap_or(Box::new(ListNode::new(0)));

        let sum = l1.val + l2.val + carry;
        let mut current_node = ListNode::new(sum % 10);
        current_node.next = add_two_numbers_internal(l1.next, l2.next, sum / 10);

        Some(Box::new(current_node))
    }

    add_two_numbers_internal(l1, l2, 0)
}


pub fn add_two_numbers_2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(n1), Some(n2)) => {
            let sum = n1.val + n2.val;
            
            if sum < 10 {
                Some(Box::new(ListNode { val: sum, next: add_two_numbers_2(n1.next, n2.next)}))
            } else {
                let carry = Some(Box::new(ListNode::new(1)));

                Some(Box::new(ListNode { val: sum - 10, next: add_two_numbers_2(add_two_numbers_2(carry, n1.next), n2.next)}))
            }
        }
    }
}