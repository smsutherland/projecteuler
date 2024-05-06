use euler_util::factor_count;

fn union_counts<T: PartialEq + Clone>(master_list: &mut Vec<(usize, T)>, new_list: &[(usize, T)]) {
    for (count, item) in new_list {
        'block: {
            for (master_count, master_item) in master_list.iter_mut() {
                if master_item == item {
                    *master_count = Ord::max(*master_count, *count);
                    break 'block;
                }
            }
            master_list.push((*count, item.clone()));
        }
    }
}

fn main() {
    let mut factors = Vec::new();
    for num in 2..=20 {
        let these_factors = factor_count(num);
        union_counts(&mut factors, &these_factors);
    }
    let mut prod = 1;
    for (i, prime) in factors {
        prod *= prime.pow(i as u32);
    }
    println!("{prod}");
}
