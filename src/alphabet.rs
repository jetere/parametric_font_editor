use std::collections::HashMap;

pub fn default_alphabet() -> HashMap<char, Vec<usize>> {
    let mut map = HashMap::new();

    // 15-Box Matrix Layout:
    // Row 1:  1,  2,  3
    // Row 2:  4,  5,  6
    // Row 3:  7,  8,  9
    // Row 4: 10, 11, 12
    // Row 5: 13, 14, 15

    map.insert('A', vec![2, 4, 6, 7, 8, 9, 10, 12, 13, 15]);
    map.insert('B', vec![1, 2, 4, 6, 7, 8, 10, 12, 13, 14]);
    map.insert('C', vec![1, 2, 3, 4, 7, 10, 13, 14, 15]);
    map.insert('D', vec![1, 2, 4, 6, 7, 9, 10, 12, 13, 14]);
    map.insert('E', vec![1, 2, 3, 4, 7, 8, 9, 10, 13, 14, 15]);
    map.insert('F', vec![1, 2, 3, 4, 7, 8, 9, 10, 13]);
    map.insert('G', vec![1, 2, 3, 4, 7, 9, 10, 12, 13, 14, 15]);
    map.insert('H', vec![1, 3, 4, 6, 7, 8, 9, 10, 12, 13, 15]);
    map.insert('I', vec![1, 2, 3, 5, 8, 11, 13, 14, 15]);
    map.insert('J', vec![3, 6, 9, 10, 12, 13, 14]);
    map.insert('K', vec![1, 3, 4, 6, 7, 8, 10, 12, 13, 15]);
    map.insert('L', vec![1, 4, 7, 10, 13, 14, 15]);
    map.insert('M', vec![1, 3, 4, 5, 6, 7, 9, 10, 12, 13, 15]);
    map.insert('N', vec![1, 3, 4, 6, 7, 9, 10, 12, 13, 15]);
    map.insert('O', vec![1, 2, 3, 4, 6, 7, 9, 10, 12, 13, 14, 15]);
    map.insert('P', vec![1, 2, 3, 4, 6, 7, 8, 9, 10, 13]);
    map.insert('Q', vec![1, 2, 3, 4, 6, 7, 9, 10, 12, 13, 14, 15]);
    map.insert('R', vec![1, 2, 3, 4, 6, 7, 8, 9, 10, 12, 13, 15]);
    map.insert('S', vec![1, 2, 3, 4, 7, 8, 9, 12, 13, 14, 15]);
    map.insert('T', vec![1, 2, 3, 5, 8, 11, 14]);
    map.insert('U', vec![1, 3, 4, 6, 7, 9, 10, 12, 13, 14, 15]);
    map.insert('V', vec![1, 3, 4, 6, 7, 9, 10, 12, 14]);
    map.insert('W', vec![1, 3, 4, 6, 7, 9, 10, 12, 13, 14, 15]);
    map.insert('X', vec![1, 3, 4, 6, 8, 10, 12, 13, 15]);
    map.insert('Y', vec![1, 3, 4, 6, 7, 8, 9, 11, 14]);
    map.insert('Z', vec![1, 2, 3, 6, 8, 10, 13, 14, 15]);

    map
}
