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
    fn longest_common_prefix_worked(){
        let strs_success = vec!["plan".to_string(), "play".to_string(), "plot".to_string(), "plague".to_string()];
        let strs_fail1 = vec!["play".to_string(),"game".to_string(),"no".to_string()];
        let strs_fail2 = vec!["flower".to_string(), "reflow".to_string(), "doflamingo".to_string()];
        assert_eq!(longest_common_prefix(strs_success), "pl");
        assert_eq!(longest_common_prefix(strs_fail1), "");
        assert_ne!(longest_common_prefix(strs_fail2), "fl")

    }

    //problem #20. Valid Parentheses ~ Easy
    #[test]
    fn is_valid_worked(){
        let s_true1 = "()[]{}".to_string();
        let s_true2 = "({[]})".to_string();
        let s_false1 = "(]".to_string();
        let s_false2 = "((".to_string();
        

        assert_eq!(is_valid(s_true1), true);
        assert_eq!(is_valid(s_true2), true);
        assert_ne!(is_valid(s_false1), true);
        assert_ne!(is_valid(s_false2), true);
    }

    //problem #557. Reverse Words in a string ~ Easy
    #[test]
    fn reverse_words_worked(){
        let s = "Let's take LeetCode contest".to_string();
        assert_eq!(reverse_words(s), "s'teL ekat edoCteeL tsetnoc")
    }