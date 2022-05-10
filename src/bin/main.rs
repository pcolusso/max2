use max2::{fold_max2, sort_max2};

fn main() {
    let to_sort = [8, 4, 1, 9, 2, 3, 6, 32, 12, 11, 19, 1829];

    println!("Sort Max: {:?}", sort_max2(to_sort));
    println!("Sort Fold: {:?}", fold_max2(to_sort));
}