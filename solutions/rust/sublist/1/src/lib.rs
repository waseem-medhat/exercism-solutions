#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if first_list.len() > second_list.len() && contains(first_list, second_list) {
        Comparison::Superlist
    } else if second_list.len() > first_list.len() && contains(second_list, first_list) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

fn contains(first_list: &[i32], second_list: &[i32]) -> bool {
    match second_list {
        [] => true,
        _ => first_list
            .windows(second_list.len())
            .any(|window| window == second_list),
    }
}
