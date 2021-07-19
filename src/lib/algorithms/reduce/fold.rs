// note: 
// reduce requires non-empty collections;
// fold (either left or right) can work with empty collections      
pub fn iterator_fold() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let sum: i32 = nums.iter().fold(
        0,
        |accu, elem| accu + elem, // init accu elem
    );
    assert_eq!(3 + 1 + 4 + 1 + 5 + 9, sum);
}

#[test]
fn demo_iterator_fold() {
    iterator_fold();
}
