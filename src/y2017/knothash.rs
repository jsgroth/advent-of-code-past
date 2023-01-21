pub fn compute_knot_hash(s: &str) -> Vec<u8> {
    let mut lengths: Vec<_> = s.chars().map(|c| c as usize).collect();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    let mut list: Vec<_> = (0..=255).into_iter().collect();

    let mut position = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        for &length in &lengths {
            circular_reverse(&mut list, position, length);
            position = (position + length + skip_size) % list.len();
            skip_size += 1;
        }
    }

    list.chunks(16)
        .map(|chunk| chunk.iter().copied().reduce(|a, b| a ^ b).unwrap())
        .collect()
}

pub fn circular_reverse(list: &mut Vec<u8>, mut i: usize, length: usize) {
    let mut j = (i + length + list.len() - 1) % list.len();

    for _ in 0..(length / 2) {
        list.swap(i, j);
        i = (i + 1) % list.len();
        j = (j + list.len() - 1) % list.len();
    }
}

pub fn to_hex_string(hash: &[u8]) -> String {
    hash.iter().copied().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_knot_hash() {
        assert_eq!(
            String::from("a2582a3a0e66e6e86e3812dcb672a272"),
            to_hex_string(&compute_knot_hash(""))
        );
        assert_eq!(
            String::from("33efeb34ea91902bb2f59c9920caa6cd"),
            to_hex_string(&compute_knot_hash("AoC 2017"))
        );
        assert_eq!(
            String::from("3efbe78a8d82f29979031a4aa0b16a9d"),
            to_hex_string(&compute_knot_hash("1,2,3"))
        );
        assert_eq!(
            String::from("63960835bcdc130f0b66d7ff4f6a5a8e"),
            to_hex_string(&compute_knot_hash("1,2,4"))
        );
    }
}
