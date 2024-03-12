#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}


pub fn merge_two_lists_1(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut r: &mut Option<Box<ListNode>> = &mut list1;
    while list2.is_some() {
        if r.is_none() || list2.as_ref()?.val < r.as_ref()?.val {
            std::mem::swap(r, &mut list2);
        }

        r = &mut r.as_mut()?.next;
    }

    list1
}

pub fn merge_two_lists_2(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (Some(l1), Some(l2)) => {
            let (val, next_l1, next_l2) = match l1.val <= l2.val {
                true => (l1.val, l1.next, Some(l2)),
                _ => (l2.val, Some(l1), l2.next)
            };

            Some(
                Box::new(
                    ListNode { val, next: merge_two_lists_2(next_l1, next_l2) }
                )
            )
        }
    }
}