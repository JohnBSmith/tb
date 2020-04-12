
pub mod rng;

/// Combines the array literal with `.into()`.
/// # Examples
/// ```
/// use tb::array_from;
/// let a: &[String] = &array_from!["a", "b", "c", "d"];
/// ```
#[macro_export]
macro_rules! array_from {
    ($($item:expr),* $(,)?) => {[$($item.into(),)*]}
}

/// Combines the vector literal with `.into()`.
/// # Examples
/// ```
/// use tb::vec_from;
/// let a: Vec<String> = vec_from!["a", "b", "c", "d"];
/// ```
#[macro_export]
macro_rules! vec_from {
    ($($item:expr),* $(,)?) => {vec![$($item.into(),)*]}
}

/// Literal for `LinkedList`.
/// # Examples
/// ```
/// use std::collections::LinkedList;
/// use literal::list;
///
/// let a: LinkedList<i32> = list![1, 2, 3, 4];
#[macro_export]
macro_rules! list {
    ($($item:expr),* $(,)?) => {{
        let mut _list = LinkedList::new();
        $(_list.push_back($item);)*
        _list
    }}
}

/// Combines the list literal with `.into()`.
/// # Examples
/// ```
/// use std::collections::LinkedList;
/// use literal::list_from;
///
/// let a: LinkedList<String> = list_from!["a", "b", "c", "d"];
/// ```
#[macro_export]
macro_rules! list_from {
    ($($item:expr),* $(,)?) => {{
        let mut _list = LinkedList::new();
        $(_list.push_back($item.into());)*
        _list
    }}
}
