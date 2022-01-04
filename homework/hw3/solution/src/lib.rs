use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::str::FromStr;

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
#[derive(Clone, Copy, Hash, Debug)]
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
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
        }
    }
}

impl FromStr for Direction {
    type Err = Errors;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "North" => Ok(Direction::North),
            "South" => Ok(Direction::South),
            "East" => Ok(Direction::East),
            "West" => Ok(Direction::West),
            _ => Err(Errors::DirectionParseError(input.to_string())),
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl Eq for Direction {}

/// Една стая в подземията. Дефинира се само с име, макар че в по-интересна имплементация може да
/// държи item-и, противници...
///
#[derive(Debug)]
pub struct Room {
    pub name: String,
    // Каквито други полета ви трябват
    pub adj: HashMap<Direction, String>,
}

/// Контейнер за стаите и не само. Ще работим предимно със тази структура.
///
#[derive(Debug)]
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
                self.rooms.insert(
                    name.into(),
                    Room {
                        name: name.into(),
                        adj: HashMap::new(),
                    },
                );
                Ok(())
            }
            true => Err(Errors::DuplicateRoom(name.into())),
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

        self.rooms
            .get_mut(room_name)
            .unwrap()
            .adj
            .insert(direction, other_room_name.into());
        self.rooms
            .get_mut(other_room_name)
            .unwrap()
            .adj
            .insert(direction.opposite(), room_name.into());
        Ok(())
    }

    /// Четене на съседа на стаята с име `room_name` в посока `direction`. Тук има няколко
    /// варианта на изход:
    ///
    /// - Ако подадената стая не съществува, очакваме грешка `Errors::UnknownRoom`
    /// - Ако подадената стая няма съсед в тази посока, Ok(None) е смисления резултат
    /// - Иначе, чакаме reference към `Room` структурата на въпросния съсед, опакована в `Ok(Some(`.
    ///
    pub fn get_next_room(
        &self,
        room_name: &str,
        direction: Direction,
    ) -> Result<Option<&Room>, Errors> {
        match self.rooms.get(room_name) {
            Some(room) => match room.adj.get(&direction) {
                Some(other_room) => Ok(self.rooms.get(other_room)),
                None => Ok(None),
            },
            None => Err(Errors::UnknownRoom(room_name.into())),
        }
    }
}

/// Помощен enum за обозначаване на редовете, които четем.
///
#[derive(PartialEq)]
pub enum ReadingState {
    Rooms,
    Links,
    EmptyLine,
}

impl Dungeon {
    fn get_line_parts(
        line: &str,
        line_number: usize,
        state: &ReadingState,
    ) -> Result<Vec<String>, Errors> {
        let chars = line.chars().collect::<Vec<_>>();

        if chars[0..2].iter().collect::<String>() != "- " {
            return Err(Errors::LineParseError { line_number });
        }

        match state {
            ReadingState::Rooms => Ok(vec![chars[2..].iter().collect::<String>()]),
            ReadingState::Links => {
                let words: Vec<String> = chars[2..]
                    .iter()
                    .collect::<String>()
                    .split(" -> ")
                    .map(|s| String::from(s))
                    .collect();
                if words.len() != 3 {
                    return Err(Errors::LineParseError { line_number });
                }
                Ok(words.iter().map(|w| String::from(w.trim())).collect())
            }
            _ => Err(Errors::LineParseError { line_number }),
        }
    }

    /// Прочитаме структурата на dungeon от нещо, което имплементира `BufRead`. Това може да е
    /// файл, или, ако тестваме, може да е просто колекция от байтове.
    ///
    /// Успешен резултат връща новосъздадения dungeon, пакетиран в `Ok`.
    ///
    /// Вижте по-долу за обяснение на грешките, които очакваме.
    ///
    pub fn from_reader<B: BufRead>(reader: B) -> Result<Self, Errors> {
        let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

        if lines.clone().len() == 0_usize {
            return Err(Errors::LineParseError { line_number: 0 });
        }

        let mut dungeon = Dungeon::new();
        let mut reading_state = ReadingState::Rooms;

        for (i, line) in lines.iter().enumerate() {
            let line_number = i + 1;

            match reading_state {
                ReadingState::Rooms => {
                    if line_number == 1 {
                        if line != "## Rooms" {
                            return Err(Errors::LineParseError { line_number });
                        }
                        continue;
                    }
                    if line == "" {
                        reading_state = ReadingState::EmptyLine;
                        continue;
                    }
                    let parts = Dungeon::get_line_parts(line.as_str(), line_number, &reading_state)?;
                    dungeon.add_room(&parts[0])?;
                }
                ReadingState::EmptyLine => {
                    if line != "## Links" {
                        return Err(Errors::LineParseError { line_number });
                    }
                    reading_state = ReadingState::Links;
                }
                ReadingState::Links => {
                    let parts = Dungeon::get_line_parts(line.as_str(), line_number, &reading_state)?;
                    let dir = Direction::from_str(&parts[1])?;
                    dungeon.set_link(&parts[0], dir, &parts[2])?;
                }
            }
        }

        Ok(dungeon)
    }
}

impl Dungeon {
    /// Търси път от `start_room_name` до `end_room_name` и го връща във вектор, пакетиран във
    /// `Ok(Some(` ако намери.
    ///
    /// Ако няма път между тези две стаи, връща `Ok(None)`.
    ///
    /// Ако четенето на стаи в един момент върне грешка, очакваме да върнете грешката нагоре.
    ///
    pub fn find_path(
        &self,
        start_room_name: &str,
        end_room_name: &str,
    ) -> Result<Option<Vec<&Room>>, Errors> {
        if start_room_name == end_room_name {
            return Ok(Some(vec![self.get_room(start_room_name)?]));
        }

        let mut q = VecDeque::<&str>::new();
        let mut checked = HashMap::<&str, Option<&str>>::new();
        q.push_back(self.get_room(start_room_name)?.name.as_str());
        let mut parent = None;

        while q.len() > 0 {
            let current = q.pop_front().unwrap();

            if checked.contains_key(current) {
                continue;
            }

            checked.insert(current, parent);

            if current == end_room_name {
                break;
            }

            for (_, v) in self.get_room(current)?.adj.iter() {
                q.push_back(v);
            }

            parent = Some(current);
        }

        if !checked.contains_key(end_room_name) {
            return Ok(None);
        }

        let mut path = VecDeque::new();
        path.push_front(self.get_room(end_room_name)?);
        while parent != None {
            path.push_front(self.get_room(parent.unwrap())?);
            parent = checked[parent.unwrap()];
        }
        Ok(Some(path.into()))
    }
}

#[cfg(test)]
mod custom_tests {
    use super::*;

    #[test]
    fn test_dungeon_building() {
        let mut dungeon = Dungeon::new();

        assert!(dungeon.add_room("room1").is_ok());
        assert_eq!(dungeon.add_room("room2").unwrap(), ());
        assert!(dungeon.add_room("room2").is_err());

        assert!(dungeon.set_link("room1", Direction::South, "room2").is_ok());
        assert!(dungeon
            .set_link("room2", Direction::South, "room3")
            .is_err());

        assert_eq!(dungeon.get_room("room1").unwrap().name, "room1");

        assert_eq!(
            dungeon
                .get_next_room("room1", Direction::South)
                .unwrap()
                .unwrap()
                .name,
            "room2"
        );
        assert!(dungeon
            .get_next_room("room2", Direction::South)
            .unwrap()
            .is_none());
        assert!(dungeon.get_next_room("room3", Direction::South).is_err());
    }

    // Залепям ги за началото, за да не се налага да trim-вам всеки ред
    const TEST_INPUT_1: &str = "
## Rooms
- room1
- room2
- room3

## Links
- room1 -> East -> room2
- room2 -> West -> room3
- room3 -> North -> room3
";

    const TEST_INPUT_2: &str = "";
    const TEST_INPUT_3: &str = "Line 1 err";
    const TEST_INPUT_4: &str = "## Rooms\nLine 2 err";
    const TEST_INPUT_5: &str = "## Rooms\n- duplicate\n- duplicate";
    const TEST_INPUT_6: &str = "## Rooms\n- room1\n- room2\n\n## Links\n- room1 -> DIR -> room2";

    #[test]
    fn test_dungeon_parsing_1() {
        let dungeon = Dungeon::from_reader(TEST_INPUT_1.trim().as_bytes()).unwrap();

        assert_eq!(dungeon.get_room("room1").unwrap().name, "room1");
        assert_eq!(dungeon.get_room("room2").unwrap().name, "room2");
        assert_eq!(dungeon.get_room("room3").unwrap().name, "room3");
        assert!(dungeon.get_room("error").is_err());

        assert_eq!(
            dungeon
                .get_next_room("room1", Direction::East)
                .unwrap()
                .unwrap()
                .name,
            "room2"
        );
        assert_eq!(
            dungeon
                .get_next_room("room2", Direction::West)
                .unwrap()
                .unwrap()
                .name,
            "room3"
        );
        assert_eq!(
            dungeon
                .get_next_room("room3", Direction::East)
                .unwrap()
                .unwrap()
                .name,
            "room2"
        );
        assert_eq!(
            dungeon
                .get_next_room("room3", Direction::South)
                .unwrap()
                .unwrap()
                .name,
            "room3"
        );
    }

    #[test]
    fn test_dungeon_parsing_2() {
        assert!(matches!(Dungeon::from_reader(TEST_INPUT_2.as_bytes()).unwrap_err(), Errors::LineParseError { line_number: 0 }));
    }

    #[test]
    fn test_dungeon_parsing_3() {
        assert!(matches!(Dungeon::from_reader(TEST_INPUT_3.as_bytes()).unwrap_err(), Errors::LineParseError { line_number: 1 }));
    }

    #[test]
    fn test_dungeon_parsing_4() {
        assert!(matches!(Dungeon::from_reader(TEST_INPUT_4.as_bytes()).unwrap_err(), Errors::LineParseError { line_number: 2 }));
    }

    #[test]
    fn test_dungeon_parsing_5() {
        let _room = String::from("duplicate");
        assert!(matches!(Dungeon::from_reader(TEST_INPUT_5.as_bytes()).unwrap_err(), Errors::DuplicateRoom(_room)));
    }

    #[test]
    fn test_dungeon_parsing_6() {
        let _dir = String::from("DIR");
        assert!(matches!(Dungeon::from_reader(TEST_INPUT_6.as_bytes()).unwrap_err(), Errors::DirectionParseError(_dir)));
    }

    #[test]
    fn test_dungeon_path() {
        let dungeon = Dungeon::from_reader(TEST_INPUT_1.trim().as_bytes()).unwrap();

        assert_eq!(
            dungeon
                .find_path("room1", "room3")
                .unwrap()
                .unwrap()
                .iter()
                .map(|x| x.name.as_str())
                .collect::<Vec<&str>>(),
            vec!["room1", "room2", "room3"]
        );
        assert_eq!(
            dungeon
                .find_path("room2", "room3")
                .unwrap()
                .unwrap()
                .iter()
                .map(|x| x.name.as_str())
                .collect::<Vec<&str>>(),
            vec!["room2", "room3"]
        );
    }

    #[test]
    fn test_dungeon_path_2() {
        let dungeon = Dungeon::from_reader(TEST_INPUT_1.trim().as_bytes()).unwrap();
        assert!(matches!(dungeon.find_path("room2", "room1"), Ok(None)));
    }
}
