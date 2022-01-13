use dungeon_map::Room;

fn main() {
    let room = Room {
        name: String::from("Living Room"),
        north: String::from("Balcony"),
        south: String::from("Bathroom"),
        east: String::from("Kitchen"),
        west: String::from("Entrance"),
    };

    println!("{}", room);
}
