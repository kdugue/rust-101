// My solution

fn dna_strand(dna: &str) -> String {
    let mut result = String::new();

    for ch in dna.chars() {
        if ch == 'A' {
            result.push('T');
        } else if ch == 'T' {
            result.push('A');
        } else if ch == 'C' {
            result.push('G');
        } else {
            result.push('C');
        }
    }

    result
}

// TODO: look at alternate solutions
