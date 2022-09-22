// Definition for singly-linked list.
//Struct for problems 21. singly linked list
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
impl Iterator for ListNode {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = &self.next {
            return Some(node.val)
        }
        None
    }
}