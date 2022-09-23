mod structs;
use structs::*;

//problems #1. Two Sum ~ Easy
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

//problems #9. Palindrome Number ~ Easy
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

//problems #13. Roman to Integer ~ Easy
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

//problems #14. Longest Common Prefix ~ Easy
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

//problems #21. Merge Two Sorted List ~ Easy
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>, 
    list2: Option<Box<ListNode>>,) -> Option<Box<ListNode>> {

        match (list1, list2) {
            (None, None) => None,
            (Some(x), None) | (None, Some(x)) => Some(x),
            (Some(mut x),Some(mut y)) => {
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


//problems #26. Remove Duplicates from Sorted Array ~ Easy
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

//problems #557. Reverse Words in a string III ~ Easy
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
#[cfg(test)]
mod tests;
