//! # My documentation Library
//!
//! Tests out rustdoc. This for example, is a crate doc comment.
//! For more information, (see the rust book comments page)[https://doc.rust-lang.org/book/comments.html] or the (rustdoc documentation)[https://doc.rust-lang.org/book/documentation.html]


/// Sample function that returns the integer 5.
///
/// # Examples
///
/// ```
/// let value: i32 = documentation::returnfive();
/// assert_eq!(value, 5);
/// ```
pub fn returnfive() -> i32 {
    5
}
