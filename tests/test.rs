use homework_1::hashset::Hashset;
use homework_1::treeset::Treeset;
use homework_1::listset::Listset;
use homework_1::arrayset::Arrayset;


#[test]
fn test_hashset() {
    let mut ds = Hashset::new();

    let ans1 = ds.insert(5);
    let ans2 = ds.find(5);
    let ans3 = ds.insert(5);
    let ans4 = ds.insert(1_000);
    let ans5 = ds.find(1_000);
    let ans6 = ds.find(3);
    let ans7 = ds.remove(5);
    let ans8 = ds.remove(99);
    let ans9 = ds.find(5);
    let ans10 = ds.remove(1_000);
    let ans11 = ds.find(1_000);

    assert_eq!(ans1, true);
    assert_eq!(ans2, true);
    assert_eq!(ans3, false);
    assert_eq!(ans4, true);
    assert_eq!(ans5, true);
    assert_eq!(ans6, false);
    assert_eq!(ans7, true);
    assert_eq!(ans8, false);
    assert_eq!(ans9, false);
    assert_eq!(ans10, true);
    assert_eq!(ans11, false);
}

#[test]
fn test_treeset() {
    let mut ds = Treeset::new();

    let ans1 = ds.insert(5);
    let ans2 = ds.find(5);
    let ans3 = ds.insert(5);
    let ans4 = ds.insert(1_000);
    let ans5 = ds.find(1_000);
    let ans6 = ds.find(3);
    let ans7 = ds.remove(5);
    let ans8 = ds.remove(99);
    let ans9 = ds.find(5);
    let ans10 = ds.remove(1_000);
    let ans11 = ds.find(1_000);

    assert_eq!(ans1, true);
    assert_eq!(ans2, true);
    assert_eq!(ans3, false);
    assert_eq!(ans4, true);
    assert_eq!(ans5, true);
    assert_eq!(ans6, false);
    assert_eq!(ans7, true);
    assert_eq!(ans8, false);
    assert_eq!(ans9, false);
    assert_eq!(ans10, true);
    assert_eq!(ans11, false);
}

#[test]
fn test_listset() {
    let mut ds = Listset::new();

    let ans1 = ds.insert(5);
    let ans2 = ds.find(5);
    let ans3 = ds.insert(5);
    let ans4 = ds.insert(1_000);
    let ans5 = ds.find(1_000);
    let ans6 = ds.find(3);
    let ans7 = ds.remove(5);
    let ans8 = ds.remove(99);
    let ans9 = ds.find(5);
    let ans10 = ds.remove(1_000);
    let ans11 = ds.find(1_000);

    assert_eq!(ans1, true);
    assert_eq!(ans2, true);
    assert_eq!(ans3, false);
    assert_eq!(ans4, true);
    assert_eq!(ans5, true);
    assert_eq!(ans6, false);
    assert_eq!(ans7, true);
    assert_eq!(ans8, false);
    assert_eq!(ans9, false);
    assert_eq!(ans10, true);
    assert_eq!(ans11, false);
}

#[test]
fn test_arrayset() {
    let mut ds = Arrayset::new();

    let ans1 = ds.insert(5);
    let ans2 = ds.find(5);
    let ans3 = ds.insert(5);
    let ans4 = ds.insert(1_000);
    let ans5 = ds.find(1_000);
    let ans6 = ds.find(3);
    let ans7 = ds.remove(5);
    let ans8 = ds.remove(99);
    let ans9 = ds.find(5);
    let ans10 = ds.remove(1_000);
    let ans11 = ds.find(1_000);

    assert_eq!(ans1, true);
    assert_eq!(ans2, true);
    assert_eq!(ans3, false);
    assert_eq!(ans4, true);
    assert_eq!(ans5, true);
    assert_eq!(ans6, false);
    assert_eq!(ans7, true);
    assert_eq!(ans8, false);
    assert_eq!(ans9, false);
    assert_eq!(ans10, true);
    assert_eq!(ans11, false);
}