fn main() {
    println!("Hello, world!");

    let v1 = vec![1, 9, 2, 3, 1, 7, 2, 8];

    let merged = merge_sort(v1);

    dbg!(&merged);
}




fn merge<T: Ord + Clone>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
    let mut p = 0;
    let mut q = 0;
    let total_len = v1.len() + v2.len();
    let mut new_vec = Vec::with_capacity(total_len);

    for i in 0..total_len {
        if p >= v1.len() {
            new_vec.push(v2[q].clone());
            q += 1;
        } else if q >= v2.len() {
            new_vec.push(v1[p].clone());
            p += 1;
        } else if v1[p] <= v2[q] {
            new_vec.push(v1[p].clone());
            p += 1;
        } else {
            new_vec.push(v2[q].clone());
            q += 1;
        }
    }

    new_vec
}

fn better_merge<T: Ord + Clone>(v1: &[T], v2: &[T]) -> Vec<T> {
    let mut iter1 = v1.iter();
    let mut iter2 = v2.iter();
    let mut new_vec = Vec::with_capacity(v1.len() + v2.len());

    let mut next1 = iter1.next();
    let mut next2 = iter2.next();

    while let (Some(a), Some(b)) = (next1, next2) {
        if a <= b {
            new_vec.push(a.clone());
            next1 = iter1.next();
        } else {
            new_vec.push(b.clone());
            next2 = iter2.next();
        }
    }

    // remaining elems from iter1
    new_vec.extend(next1.into_iter().chain(iter1).cloned());
    // remaining elems from iter2
    new_vec.extend(next2.into_iter().chain(iter2).cloned());

    new_vec
}

fn cursed_better_merge<T: Ord + Clone>(v1: &[T], v2: &[T]) -> Vec<T> {
    let mut iter1 = v1.iter();
    let mut iter2 = v2.iter();
    let mut new_vec = Vec::with_capacity(v1.len() + v2.len());

    let mut next1 = iter1.next();
    let mut next2 = iter2.next();

    // let mut push_and_update = |val: &T, next: &mut Option<&T>, iter: &mut std::slice::Iter<'_, T>| {
    //     new_vec.push(val.clone());
    //     *next = iter.next();
    // };

    fn push_and_update<'a, T: Clone>(
        new_vec: &mut Vec<T>,
        val: &'a T,
        next: &mut Option<&'a T>,
        iter: &mut std::slice::Iter<'a, T>,
    ) {
        new_vec.push(val.clone());
        *next = iter.next();
    }

    loop {
        match (next1, next2) {
            (Some(a), Some(b)) => {
                if a <= b {
                    push_and_update(&mut new_vec, a, &mut next1, &mut iter1);
                } else {
                    push_and_update(&mut new_vec, b, &mut next2, &mut iter2);
                }
            }
            (Some(a), None) => {
                push_and_update(&mut new_vec, a, &mut next1, &mut iter1);
            },
            (None, Some(b)) => {
                push_and_update(&mut new_vec, b, &mut next1, &mut iter2);
            },
            (None, None) => break
        }
    }

    todo!()
}



fn merge_sort<T: Ord + Clone>(v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }

    let mid = v.len() / 2;
    let (v1, v2) = v.split_at(mid);

    let v1_sorted = merge_sort(v1.to_vec());
    let v2_sorted = merge_sort(v2.to_vec());

    better_merge(&v1_sorted, &v2_sorted)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_better_merge() {
        let v1 = vec![1, 3, 7, 9];
        let v2 = vec![2, 3, 4, 8];

        let merged = better_merge(&v1, &v2);

        let expected = vec![1, 2, 3, 3, 4, 7, 8, 9];
        assert_eq!(merged, expected);
    }

    #[test]
    fn test_merge_sort() {
        let v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let sorted = merge_sort(v);
        let expected = vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9];
        assert_eq!(sorted, expected);
    }
}