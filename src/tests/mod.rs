use crate::*;

//problems #1. Two Sum ~ Easy
#[test]
fn two_sum_works() {
    let nums: Vec<i32> = vec![2, 7, 11, 15, 6];
    let target: i32 = 9;

    assert_eq!(two_sum(nums, target), [0, 1]);
}

//problems #9. Palindrome ~ Easy
#[test]
fn is_palindrome_work() {
    assert!(is_palindrome(121));
    assert!(!is_palindrome(321));
}

//problems #13. Roman to Integer ~ Easy
#[test]
fn roman_to_int_work() {
    let num1 = roman_to_int("VII".to_string());
    let num2 = roman_to_int("clxxiv".to_string());
    assert_eq!(num1, 7);
    assert_eq!(num2, 174);
}
#[test]
#[should_panic(expected = "Contains non-roman number [K]")]
fn roman_should_panic() {
    roman_to_int("gghfk".to_string());
}

//problems #14. Longet Common Prefix ~ Easy
#[test]
fn longest_common_prefix_worked() {
    let strs_success = vec![
        "plan".to_string(),
        "play".to_string(),
        "plot".to_string(),
        "plague".to_string(),
    ];
    let strs_fail1 = vec!["play".to_string(), "game".to_string(), "no".to_string()];
    let strs_fail2 = vec![
        "flower".to_string(),
        "reflow".to_string(),
        "doflamingo".to_string(),
    ];
    assert_eq!(longest_common_prefix(strs_success), "pl");
    assert_eq!(longest_common_prefix(strs_fail1), "");
    assert_ne!(longest_common_prefix(strs_fail2), "fl")
}

//problem #20. Valid Parentheses ~ Easy
#[test]
fn is_valid_worked() {
    let s_true1 = "()[]{}".to_string();
    let s_true2 = "({[]})".to_string();
    let s_false1 = "(]".to_string();
    let s_false2 = "((".to_string();

    assert_eq!(is_valid(s_true1), true);
    assert_eq!(is_valid(s_true2), true);
    assert_ne!(is_valid(s_false1), true);
    assert_ne!(is_valid(s_false2), true);
}

//problem #21. Merge Two Sorted Lists
#[test]
fn merge_two_lists_worked() {
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { 
                val: 4, 
                next: None })),})),})
    );
    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { 
                val: 4, 
                next: None })),})),})
    );

    let expected_result = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { 
                val: 2, 
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { 
                            val: 4, 
                            next: None })),})),})),})),})),})
    );

    assert_eq!(merge_two_lists(list1, list2), expected_result)
}

//problem #26. Remove Duplicates from Sorted Array ~ Easy
#[test]
fn remove_duplicates_worked(){
    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    let expected_result = vec![0,1,2,3,4];
    assert_eq!(remove_duplicates(&mut nums), expected_result.len() as i32);
}

//problem #27. Remove element from array ~ Easy
#[test]
fn remove_element_worked(){
    let mut nums = vec![0,1,2,2,3,0,4,2];
    let val = 2;
    let output= 5;
    let _output_nums = [0,1,4,0,3,];

    assert_eq!(remove_element(&mut nums, val), output);
}

//problem #35. Search Insert Position ~ Easy
#[test]
fn search_insert_worked(){
    let (nums1, target1, expect1) = 
        (vec![1,3,5,6], 5, 2);
    
    let (nums2, target2, expect2) =
        (vec![1,3,5,6], 2, 1);

    assert_eq!(search_insert(nums1, target1), expect1);
    assert_eq!(search_insert(nums2, target2), expect2);

}

//Problem #58. Length of Last Word ~ Easy
#[test]
fn length_of_last_word_worked(){
    let (s1, expect1) = ("Hello World".to_string(), 5);
    let (s2, expect2) = ("   fly me   to   the moon  ".to_string(),4);

    assert_eq!(length_of_last_word(s1), expect1);
    assert_eq!(length_of_last_word(s2), expect2);
}

//problems #66. Plus One
#[test]
pub fn plus_one_worked(){
    let (digits1, expect1) = (vec![4,3,2,1],vec![4,3,2,2]);
    let (digits2, expect2) = (vec![9,9,9], vec![1,0,0,0]);

    assert_eq!(plus_one(digits1), expect1);
    assert_eq!(plus_one(digits2), expect2);
}

//problem #67. Add Binary ~ Easy
#[test]
pub fn add_binary_worked(){
    let (a1, b1, expect1) = ("1010".to_string(), "1011".to_string(), "10101".to_string());
    let (a2, b2, expect2) = ("1010".to_string(), "011".to_string(), "1101".to_string());
    let (a3, b3, expect3) = ("101111".to_string(),
    "10".to_string(),"110001".to_string());

    assert_eq!(add_binary(a1, b1),expect1);
    assert_eq!(add_binary(a2, b2),expect2);
    assert_eq!(add_binary(a3, b3),expect3);
}

//problem #557. Reverse Words in a string ~ Easy
#[test]
fn reverse_words_worked() {
    let s = "Let's take LeetCode contest".to_string();
    assert_eq!(reverse_words(s), "s'teL ekat edoCteeL tsetnoc")
}

// problem #1680. Concatenation of Consecutive Binary Numbers
#[test]
fn concatenated_binary_worked() {
    let input1 = 3 ;
    let expect1 = 27;
    let input2 = 12 ;
    let expect2 = 505379714;

    assert_eq!(concatenated_binary(input1), expect1);
    assert_eq!(concatenated_binary(input2), expect2);
}
