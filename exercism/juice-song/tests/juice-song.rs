use juice_song as juice;

#[test]
fn test_verse_0() {
    assert_eq!(juice::verse(0), "No more bottles of juice on the wall, no more bottles of juice.\nGo to the store and buy some more, 99 bottles of juice on the wall.\n");
}

#[test]
#[ignore]
fn test_verse_1() {
    assert_eq!(juice::verse(1), "1 bottle of juice on the wall, 1 bottle of juice.\nTake it down and pass it around, no more bottles of juice on the wall.\n");
}

#[test]
#[ignore]
fn test_verse_2() {
    assert_eq!(juice::verse(2), "2 bottles of juice on the wall, 2 bottles of juice.\nTake one down and pass it around, 1 bottle of juice on the wall.\n");
}

#[test]
#[ignore]
fn test_verse_8() {
    assert_eq!(juice::verse(8), "8 bottles of juice on the wall, 8 bottles of juice.\nTake one down and pass it around, 7 bottles of juice on the wall.\n");
}

#[test]
#[ignore]
fn test_song_8_6() {
    assert_eq!(juice::sing(8, 6), "8 bottles of juice on the wall, 8 bottles of juice.\nTake one down and pass it around, 7 bottles of juice on the wall.\n\n7 bottles of juice on the wall, 7 bottles of juice.\nTake one down and pass it around, 6 bottles of juice on the wall.\n\n6 bottles of juice on the wall, 6 bottles of juice.\nTake one down and pass it around, 5 bottles of juice on the wall.\n");
}

#[test]
#[ignore]
fn test_song_3_0() {
    assert_eq!(juice::sing(3, 0), "3 bottles of juice on the wall, 3 bottles of juice.\nTake one down and pass it around, 2 bottles of juice on the wall.\n\n2 bottles of juice on the wall, 2 bottles of juice.\nTake one down and pass it around, 1 bottle of juice on the wall.\n\n1 bottle of juice on the wall, 1 bottle of juice.\nTake it down and pass it around, no more bottles of juice on the wall.\n\nNo more bottles of juice on the wall, no more bottles of juice.\nGo to the store and buy some more, 99 bottles of juice on the wall.\n");
}
