use solution::*;

#[test]
fn test_room_exists() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    let result = dungeon.add_room("Entrance");
    let room = String::from("Entance");
    assert!(matches!(result.unwrap_err(), Errors::DuplicateRoom(room)));
}

#[test]
fn test_room_not_found() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    let result = dungeon.get_room("Entranc");
    let room = String::from("Entanc");
    assert!(matches!(result.unwrap_err(), Errors::UnknownRoom(room)));
}

#[test]
fn test_set_link_to_room_not_found() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    let result = dungeon.set_link("Entrance", Direction::North, "Hallway");
    let room = String::from("Hallway");
    assert!(matches!(result.unwrap_err(), Errors::UnknownRoom(room)));
}

#[test]
fn test_from_reader() {
    const INPUT_1: &str = "
# Rooms
- Entrance
- Hallway

## Links
- Entrance -> East -> Hallway
";

    const INPUT_2: &str = "
## Rooms

- Entrance
- Hallway

## Links
- Entrance -> East -> Hallway
";

    const INPUT_3: &str = "
## Rooms
- Entrance
- Hallway


## Links
- Entrance -> East -> Hallway
";

    const INPUT_4: &str = "
## Rooms
- Entrance
- Hallway

## Links
- Entrance -< East -> Hallway
";

    const INPUT_5: &str = "
## Rooms
- Entrance
- Entrance

## Links
- Entrance -> East -> Hallway
";

    let dungeon = Dungeon::from_reader(INPUT_1.trim().as_bytes());
    assert!(matches!(dungeon.unwrap_err(), Errors::LineParseError{line_number: 1}));

    let dungeon = Dungeon::from_reader(INPUT_2.trim().as_bytes());
    assert!(matches!(dungeon.unwrap_err(), Errors::LineParseError{line_number: 3}));

    let dungeon = Dungeon::from_reader(INPUT_3.trim().as_bytes());
    assert!(matches!(dungeon.unwrap_err(), Errors::LineParseError{line_number: 5}));

    let dungeon = Dungeon::from_reader(INPUT_4.trim().as_bytes());
    assert!(matches!(dungeon.unwrap_err(), Errors::LineParseError{line_number: 6}));

    let dungeon = Dungeon::from_reader(INPUT_5.trim().as_bytes());
    let room = String::from("Entance");
    assert!(matches!(dungeon.unwrap_err(), Errors::DuplicateRoom(room)));
}

#[test]
fn test_get_path() {
    const TEST_INPUT_1: &str = "
## Rooms
- R1
- R2
- R3
- R4

## Links
- R1 -> East -> R2
- R1 -> West -> R3
- R3 -> East -> R4
";

    const TEST_INPUT_2: &str = "
## Rooms
- R1
- R2
- R3
- R4

## Links
- R1 -> East -> R2
- R1 -> West -> R3
- R3 -> West -> R4
";

    let dungeon = Dungeon::from_reader(TEST_INPUT_1.trim().as_bytes()).unwrap();

    let path = dungeon.find_path("R1", "R4");
    println!("{:?}", path);
    assert!(matches!(path, Ok(None)));

    let dungeon = Dungeon::from_reader(TEST_INPUT_2.trim().as_bytes()).unwrap();

    let path = dungeon.find_path("R1", "R4");
    assert_eq!(path.unwrap().unwrap().len(), 3);
}
