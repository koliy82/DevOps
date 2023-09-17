use std::{env, f64};

//Definition for singly-linked list.
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

fn main() {
    let l1 = Some(Box::new(ListNode{val: 2, next: Some(Box::new(ListNode{val: 4, next: Some(Box::new(ListNode::new(3))) })) }));
    let l2 = Some(Box::new(ListNode{val: 5, next: Some(Box::new(ListNode{val: 6, next: Some(Box::new(ListNode::new(4))) })) }));
    //l1 =
    // [2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,9]
    // l2 =
    // [5,6,4,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,2,4,3,9,9,9,9]
    println!("{}", u128::max_value());
    println!("{}", f64::max(0.0, 0.0));
    //340282366920938463463374607431768211455
    //5642432432432432432432432432432432432432432432432432432439999
    //LeetCode 👍

    add_two_numbers(l1, l2);
}

//Input: l1 = [2,4,3], l2 = [5,6,4]
//Output: [7,0,8]
//Explanation: 342 + 465 = 807.
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let nums = get_nums(l1,0,1) + get_nums(l2,0,1);
    println!("{}", nums);
    println!("{:?}", Some(build_nodes(nums)));
    return Option::from(build_nodes(nums));
}

fn get_nums(opt: Option<Box<ListNode>>, mut nums:u128, mut index:u128) -> u128{
    match opt {
        None => {}
        Some(it) => {
            nums += it.val as u128 * index;
            index *= 10;
            nums = get_nums(it.next, nums, index);
        }
    }
    return nums;
}

fn build_nodes(num: u128) -> Box<ListNode>{
    let mut nodes: Box<ListNode> = Box::new(ListNode::new((num % 10) as i32));
    if num / 10 > 0 {
        nodes.next = Some(build_nodes(num / 10));
    }
    return nodes
}

// fn to_array(nums:i32) -> Vec<i32>{
//     return nums.to_string().chars().map(|d| d.to_digit(10).unwrap() as i32).collect();
// }

// fn list_add(ops: &Option<Box<ListNode>>, index:usize, arr:Vec<i32>) -> Option<Box<ListNode>> {
//     match ops {
//         None => {}
//         Some(it) => {
//             if index!=arr.len()+1 {
//                 Some(Box::new(ListNode{val: arr[index], next: list_add(&it.next, index+1, arr) }));
//             }
//         }
//     }
//     return ops.clone();
// }

// fn to_ops(arr:Vec<i32>) -> Option<Box<ListNode>> {
//     let first = *arr.first().unwrap();
//     let mut res = Some(Box::new(ListNode::new(first)));
//     println!("first {}", first);
//     for i in arr.iter() {
//         if i != &first{
//             println!("iterate {}", i);
//             res = list_add(&res, *i);
//         }
//     }
//     return res;
// }
//
// fn list_add(ops: &Option<Box<ListNode>>, num:i32) -> Option<Box<ListNode>>{
//     match ops {
//         None => {None}
//         Some(it) => {
//             println!("add {}", num);
//             Some(Box::new(ListNode{val: num, next: None}))
//         }
//     }
// }