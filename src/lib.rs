mod structs;

use structs::*;

//problem #1. Two Sum ~ Easy
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let table: HashMap<usize, &i32> = HashMap::from_iter(nums.iter().enumerate());
    // for (i, v) in nums.iter().enumerate() {

    // }
    let mut result = Vec::with_capacity(1);
    for i in 0..nums.len() {
        if let Some(idx) = table.iter().find_map(|(k, v)| {
            if **v == (target - nums[i]) {
                Some(*k)
            } else {
                None
            }
        }) {
            result.push(i as i32);
            result.push(idx as i32);
        }
    }
    result[0..2].to_vec()
}

//problem #9. Palindrome Number ~ Easy
pub fn is_palindrome(x: i32) -> bool {
    let num = x.to_string();
    let num_reverse =
        String::from_utf8(num.as_bytes().to_owned().into_iter().rev().collect()).unwrap();
    //in case leetcode has from_iter func, use this instead:
    // let num_reverse =
    //     String::from_iter(num.clone().chars().rev());
    if num == num_reverse {
        true
    } else {
        false
    }
}

//problem #13. Roman to Integer ~ Easy
pub fn roman_to_int(s: String) -> i32 {
    use std::collections::HashMap;
    //create dictionary to store roman values
    let mut roman_lib: HashMap<char, i32> = HashMap::with_capacity(7);

    roman_lib.extend(
        [
            ('I', 1 as i32),
            ('V', 5 as i32),
            ('X', 10 as i32),
            ('L', 50 as i32),
            ('C', 100 as i32),
            ('D', 500 as i32),
            ('M', 1000 as i32),
        ]
        .into_iter()
        .to_owned(),
    );

    //variable to store sum
    let mut sum: i32 = 0;
    //store previous number
    let mut prev: i32 = 0;
    // convert s to chars in uppercase, reverse iterate,
    //convert from right to left
    for char in s.to_uppercase().chars().rev() {
        if let Some(num) = roman_lib.get(&char) {
            if *num < prev {
                sum -= num;
            } else {
                sum += num;
            }
            prev = *num;
        } else {
            panic!("Contains non-roman number [{}]", char)
        }
    }
    sum
}

//problem #14. Longest Common Prefix ~ Easy
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut chars = String::new();
    for (i, ch) in strs[0].chars().enumerate() {
        if strs
            .iter()
            .map(|str| str.chars().nth(i))
            .all(|x| x == Some(ch))
        {
            chars = chars + String::from(ch).as_ref();
        } else {
            return chars;
        }
    }
    chars
}

//problem #20. Valid Parentheses ~ Easy
pub fn is_valid(s: String) -> bool {
    if s.len() % 2 == 1 {
        return false;
    }
    let mut stack = Vec::<char>::new();
    for ch in s.chars() {
        match ch {
            '(' => {
                stack.push(ch);
            }
            '{' => stack.push(ch),
            '[' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => return false,
        }
    }
    if stack.len() != 0 {
        return false;
    }
    true
}

//problem #21. Merge Two Sorted List ~ Easy
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(x), None) | (None, Some(x)) => Some(x),
        (Some(mut x), Some(mut y)) => {
            if x.val <= y.val {
                x.next = merge_two_lists(Some(y), x.next);
                Some(x)
            } else {
                y.next = merge_two_lists(Some(x), y.next);
                Some(y)
            }
        }
    }
}

//problem #26. Remove Duplicates from Sorted Array ~ Easy
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

//problem #27. Remove Element from array ~ Easy
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|x| x != &val);
    nums.len() as _
}

//problem #35. Search Insert Position ~ Easy
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(n) => n as _,
        Err(n) => n as _,
    }
}

//Problem #58. Length of Last Word ~ Easy
pub fn length_of_last_word(s: String) -> i32 {
    let mut word = String::new();
    for ch in s.chars().rev() {
        if ch == ' ' {
            if word.len() != 0 {
                break;
            }
        } else {
            word.push(ch)
        }
    }
    word.len() as _
}

//problems #66. Plus One
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    let mut digits = digits;
    for n in &mut digits.iter_mut().rev() {
        if *n + carry >= 10 {
            *n = 10 - (*n + carry);
            carry = 1;
        } else {
            *n = *n + carry;
            carry = 0;
            break;
        }
    }
    if carry == 1 {
        let mut carry = vec![carry];
        carry.append(&mut digits);
        carry
    } else {
        digits
    }
}

//problem #557. Reverse Words in a string III ~ Easy
pub fn reverse_words(s: String) -> String {
    let mut string_vec = Vec::<String>::new();
    let mut str_buf = String::new();
    let mut s = s;

    while let Some(ch) = s.pop() {
        if ch == ' ' {
            str_buf.push(ch);
            string_vec.push(str_buf.clone());
            str_buf.clear();
        } else {
            str_buf.push(ch);
        }
    }
    str_buf.push(' ');
    string_vec.push(str_buf.clone());
    str_buf.clear();

    while let Some(str) = string_vec.pop() {
        str_buf = str_buf + str.as_ref();
    }
    let final_len = str_buf.len() - 1;
    str_buf[..final_len].to_string()
}

// [Not solved]
// problem #1680. Concatenation of Consecutive Binary Numbers ~ Medium
// You need binary shifting, below code doesn't work on high num.
// todo: refactor using binary shifting technique,
pub fn concatenated_binary(n: i32) -> i32 {
    let mut str = String::new();
    for i in 1..=n {
        str.push_str(format!("{i:b}").as_str());
    }
    //use u128 instead so it doesn't overflow
    let num = u128::from_str_radix(&str[..], 2).unwrap();
    str.clear();
    (num % 1000000007) as i32
}
#[cfg(test)]
mod tests;
