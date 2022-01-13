use dungeon_map::*;

#[test]
fn test_short() {
    let room = Room {
        name: String::from("X"),
        north: String::from("N"),
        south: String::from("S"),
        east: String::from("E"),
        west: String::from("W"),
    };

    let expected = "
    [ N ]
      |
    +-N-+
W - | X | - E
    +-S-+
      |
    [ S ]";

    assert_eq!(format!("{}", room), expected);
}
