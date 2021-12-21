use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{BufRead};

/// Различните грешки, които ще очакваме да върнете като резултат от някои невалидни операции.
/// Повече детайли по-долу.
///
#[derive(Debug)]
pub enum Errors {
    DuplicateRoom(String),
    UnknownRoom(String),
    IoError(std::io::Error),
    LineParseError { line_number: usize },
    DirectionParseError(String),
}

/// Четирите посоки, в които може една стая да има съседи. Може да добавите още trait
/// имплементации, за да си улесните живота.
///
#[derive(Clone, Copy, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East  => Direction::West,
            Direction::West  => Direction::East,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East  => write!(f, "East"),
            Direction::West  => write!(f, "West"),
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool{
        self.to_string() == other.to_string()
    }
}
impl Eq for Direction {}

/// Една стая в подземията. Дефинира се само с име, макар че в по-интересна имплементация може да
/// държи item-и, противници...
///
pub struct Room {
    pub name: String,
    // Каквито други полета ви трябват
    pub adj: HashMap<Direction, String> 
}

/// Контейнер за стаите и не само. Ще работим предимно със тази структура.
///
pub struct Dungeon {
    // Каквито полета ви трябват
    rooms: HashMap<String, Room>,
}

impl Dungeon {
    /// Конструиране на празен Dungeon, в който няма никакви стаи.
    ///
    pub fn new() -> Self {
        Dungeon {
            rooms: HashMap::new(),
        }
    }

    /// Добавяне на стая към Dungeon с име `name`. Връща `Ok(())` при успех. Ако вече има стая с
    /// такова име, очакваме да върнете `Errors::DuplicateRoom` с името.
    ///
    pub fn add_room(&mut self, name: &str) -> Result<(), Errors> {
        match self.rooms.contains_key(name) {
            false => {
                self.rooms.insert(name.into(), Room { name: name.into(), adj: HashMap::new() }); 
                Ok(())
            },
            true => Err(Errors::DuplicateRoom(name.into()))
        }
    }

    /// Прочитане на дадена стая -- когато извикаме `get_room`, очакваме reference към `Room`
    /// структурата с това име.
    ///
    /// Ако няма такава стая, очакваме `Errors::UnknownRoom` с подаденото име.
    ///
    pub fn get_room(&self, room_name: &str) -> Result<&Room, Errors> {
        match self.rooms.get(room_name) {
            Some(room) => Ok(room),
            None => Err(Errors::UnknownRoom(room_name.into())),
        }
    }

    /// Добавяне на съсед на дадена стая. След извикването на функцията, очакваме стаята с име
    /// `room_name` да има връзка в посока `direction` със стаята с име `other_room_name`.
    ///
    /// Също така очакваме `other_room_name` да има връзка с `room_name` в *обратната* посока.
    ///
    /// Успешен резултат е `Ok(())`. В случай, че която от двете стаи не същестува, очакваме грешка
    /// `Errors::UnknownRoom` със съответното име на липсваща стая. Ако и двете липсват, спокойно
    /// върнете тази, която проверявате първо.
    ///
    pub fn set_link(
        &mut self,
        room_name: &str,
        direction: Direction,
        other_room_name: &str,
    ) -> Result<(), Errors> {
        if !self.rooms.contains_key(room_name) {
            return Err(Errors::UnknownRoom(room_name.into()));
        }
        if !self.rooms.contains_key(other_room_name) {
            return Err(Errors::UnknownRoom(other_room_name.into()));
        }

        self.rooms.get_mut(room_name).unwrap().adj.insert(direction, other_room_name.into());
        self.rooms.get_mut(other_room_name).unwrap().adj.insert(direction.opposite(), room_name.into());
        Ok(())
    }

    /// Четене на съседа на стаята с име `room_name` в посока `direction`. Тук има няколко
    /// варианта на изход:
    ///
    /// - Ако подадената стая не съществува, очакваме грешка `Errors::UnknownRoom`
    /// - Ако подадената стая няма съсед в тази посока, Ok(None) е смисления резултат
    /// - Иначе, чакаме reference към `Room` структурата на въпросния съсед, опакована в `Ok(Some(`.
    ///
    pub fn get_next_room(&self, room_name: &str, direction: Direction) -> Result<Option<&Room>, Errors> {
        match self.rooms.get(room_name) {
            Some(room) => {
                match room.adj.get(&direction) {
                    Some(other_room) => Ok(self.rooms.get(other_room)),
                    None => Ok(None)
                }
            },
            None => Err(Errors::UnknownRoom(room_name.into()))
        }
    }
}

impl Dungeon {
    /// Прочитаме структурата на dungeon от нещо, което имплементира `BufRead`. Това може да е
    /// файл, или, ако тестваме, може да е просто колекция от байтове.
    ///
    /// Успешен резултат връща новосъздадения dungeon, пакетиран в `Ok`.
    ///
    /// Вижте по-долу за обяснение на грешките, които очакваме.
    ///
    pub fn from_reader<B: BufRead>(reader: B) -> Result<Self, Errors> {
        let mut lines = reader.lines();
        if lines.by_ref().count() == 0_usize { return Err(Errors::LineParseError { line_number: 0 }); }

        let mut dungeon = Dungeon::new();

        #[derive(PartialEq)]
        pub enum ReadingState {
            Rooms,
            Links,
            EmptyLine,
        }

        let mut reading_state = ReadingState::Rooms;

        for (i,l) in lines.enumerate() {
            let line_number = i + 1;
            let line = l.unwrap();

            match line_number {
                1 => {
                    if line != "## Rooms" { return Err(Errors::LineParseError { line_number }); }
                }, 
                _ => {
                    match line.as_str() {
                        "" =>  { 
                            if reading_state != ReadingState::Rooms { return Err(Errors::LineParseError { line_number }); }
                            reading_state = ReadingState::EmptyLine;
                        },
                        _ => {
                            match reading_state {
                                ReadingState::Rooms => {
                                    todo!();
                                },
                                _ => {
                                    match line.as_str() {
                                        "## Links" => {
                                            if reading_state != ReadingState::EmptyLine { return Err(Errors::LineParseError { line_number }); }
                                            reading_state = ReadingState::Links;
                                        }
                                        _ => {
                                            if reading_state != ReadingState::Links { return Err(Errors::LineParseError { line_number }); }
                                            todo!();
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }

        Ok(dungeon)
    }
}

#[cfg(test)]
mod custom_tests {
    use super::*;

    // Първите два теста са копнати от условието
    #[test]
    fn test_link_1() {
        let mut dungeon = Dungeon::new();

        dungeon.add_room("Entrance").unwrap();
        dungeon.add_room("Hallway").unwrap();
        dungeon.set_link("Entrance", Direction::East, "Hallway").unwrap();

        assert_eq!(dungeon.get_room("Hallway").unwrap().name, "Hallway");
        assert_eq!(dungeon.get_next_room("Hallway", Direction::West).unwrap().unwrap().name, "Entrance");
    }
    
    #[test]
    fn test_list_2() {
        let mut dungeon = Dungeon::new();

        dungeon.add_room("Entrance").unwrap();
        dungeon.add_room("Hallway").unwrap();
        dungeon.add_room("Magic Lab").unwrap();

        dungeon.set_link("Entrance", Direction::East, "Hallway").unwrap();
        dungeon.set_link("Hallway", Direction::West, "Magic Lab").unwrap();

        assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway");
        assert_eq!(dungeon.get_next_room("Hallway", Direction::West).unwrap().unwrap().name, "Magic Lab");
    }
}
