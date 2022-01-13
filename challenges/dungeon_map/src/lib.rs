use std::fmt;

pub struct Room {
    pub name: String,
    pub north: String,
    pub south: String,
    pub east: String,
    pub west: String,
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;

        let center_count = usize::max(self.south.chars().count(), usize::max(self.name.chars().count(), self.north.chars().count()));
        let padding_left = self.west.chars().count() + 3;
        let box_width = center_count + 2 + (center_count + 1) % 2;

        writeln!(f, "{:>padding_left$}[{:^box_width$}]", "", self.north, padding_left = padding_left, box_width = box_width)?;
        writeln!(f, "{:>padding_left$}{}", "", "|", padding_left = padding_left + box_width / 2 + 1)?;
        writeln!(f, "{:>padding_left$}+{:-^box_width$}+", "", "N", padding_left = padding_left, box_width = box_width)?;
        writeln!(f, "{} - |{:^box_width$}| - {}", self.west, self.name, self.east, box_width = box_width)?;
        writeln!(f, "{:>padding_left$}+{:-^box_width$}+", "", "S", padding_left = padding_left, box_width = box_width)?;
        writeln!(f, "{:>padding_left$}{}", "", "|", padding_left = padding_left + box_width / 2 + 1)?;
        write!(f, "{:>padding_left$}[{:^box_width$}]", "", self.south, padding_left = padding_left, box_width = box_width)?;

        Ok(())
    }
}

#[cfg(test)]
mod custom_tests {
    use super::*;

    #[test]
    fn test1() {
        let room = Room {
            name: String::from("Living Room"),
            north: String::from("Balcony"),
            south: String::from("Bathroom"),
            east: String::from("Kitchen"),
            west: String::from("Entrance"),
        };

        let expected = "
           [   Balcony   ]
                  |
           +------N------+
Entrance - | Living Room | - Kitchen
           +------S------+
                  |
           [  Bathroom   ]";

        assert_eq!(format!("{}", room), expected);
    }

    #[test]
    fn test2() {
        let room = Room {
            name: String::from("Дневна"),
            north: String::from("Тераса"),
            south: String::from("Баня"),
            east: String::from("Кухня"),
            west: String::from("Вход"),
        };

        let expected = "
       [ Тераса  ]
            |
       +----N----+
Вход - | Дневна  | - Кухня
       +----S----+
            |
       [  Баня   ]";

        assert_eq!(format!("{}", room), expected);
    }
}
