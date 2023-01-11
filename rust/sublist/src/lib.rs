use crate::Comparison::{Equal, Sublist, Superlist, Unequal};

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + std::fmt::Debug>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Equal;
    }
    if _first_list.is_empty() {
        return Sublist;
    } else if _second_list.is_empty() {
        return Superlist;
    }
    let (longer, shorter) = if _first_list.len() >= _second_list.len() {
        (_first_list, _second_list)
    } else {
        (_second_list, _first_list)
    };
    for chunk in longer.windows(shorter.len()) {
        if chunk.eq(shorter) {
            return if _first_list.len() >= _second_list.len() {
                Superlist
            } else {
                Sublist
            };
        }
    }
    Unequal
}