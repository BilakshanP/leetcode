#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

pub fn delete_duplicates_1(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr: &mut Option<Box<ListNode>> = &mut head;

    while let Some(node) = curr {
        while let Some(next) = &mut node.next {
            if node.val != next.val {
                break;
            }

            node.next = next.next.take();
        }

        curr = &mut node.next
    }

    head
}

pub fn delete_duplicates_2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>>  {
    let mut curr_opt: Option<&mut Box<ListNode>> = head.as_mut();
    
    while let Some(curr) = curr_opt {
        let mut next_opt: Option<Box<ListNode>> = curr.next.take();
        
        while let Some(next) = next_opt.as_mut() {    
            if next.val == curr.val {
                next_opt  = next.next.take();
            } else {
                curr.next = next_opt;
                break;
            }
        }

        curr_opt = curr.next.as_mut();
    }

    head
}

pub fn delete_duplicates_3(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => head,
        Some(mut x) => {
            if let Some(y) = x.next {
                if y.val == x.val {
                    x.next = y.next;
                    return delete_duplicates_3(Some(x));
                } else {
                    x.next = delete_duplicates_3(Some(y));
                    return Some(x);
                }
            }

            Some(x)
        }
    }
}