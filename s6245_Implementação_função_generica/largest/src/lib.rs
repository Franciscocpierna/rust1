fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[test]
fn test_largest() {
    let list = vec![1, 3, 2, 5, 4];
    assert_eq!(largest(&list), 5);

    let list2 = vec!['z', 'y', 'x'];
    assert_eq!(largest(&list2), 'z');
}
