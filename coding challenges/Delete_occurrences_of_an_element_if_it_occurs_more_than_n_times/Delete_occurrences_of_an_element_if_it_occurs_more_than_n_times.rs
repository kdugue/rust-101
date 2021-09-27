// my solution
// Time: O(n^2)
// Space: O(n)

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    // create associated vectors
    // element num is vector has a corresponding
    // frequency count in num_count
    let mut nums = Vec::new();
    let mut num_count = Vec::new();
    let mut result = Vec::new();

    // cloned() is needed to not add num references in result vector
    for e in lst.into_iter().cloned() {
        let elem = nums.iter().position(|&r| r == e);
        if let None = elem {
            nums.push(e);
            num_count.push(1);
            result.push(e);
        } else {
            let elem_index = nums.iter().position(|&r| r == e).unwrap();
            let mut current_count = num_count[elem_index];
            if current_count < n {
                current_count = current_count + 1;
                num_count[elem_index] = current_count;
                result.push(e);
            }
        }
    }

    result
}

// efficient solution
// Time: O(n)
// Space: O(n)
fn delete_nth_efficient(lst: &[u8], n: usize) -> Vec<u8> {
    // import standar library from hash map
    use std::collections::HashMap;

    let mut num_frequencies = HashMap::new();

    let mut result = Vec::new();

    // iterate through numbers in vector
    // can also do for num in lst.clone()
    // but is not effieicnet
    for num in lst {
        // * for accessing num_frequences
        // we only need to access and don't need
        // to create a copy (don't use clone)
        let count = *num_frequencies
            .entry(num)
            .and_modify(|x| *x += 1)
            .or_insert(1);

        if count <= n {
            // need to use * because it derefences
            // allows you to access the referred-tp value
            result.push(*num)
        }
    }

    result
}
