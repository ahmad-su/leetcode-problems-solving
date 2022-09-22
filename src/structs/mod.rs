// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// impl ListNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }

//   pub fn next<'a>(&'a self) -> Option<&'a Box<ListNode>> {
//     self.next.as_ref()
//   }
// }