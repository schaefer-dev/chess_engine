#[derive(Debug)]
#[derive(Hash)]
pub struct Field {
    pub file: char,
    pub rank: u8,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        (self.file == other.file) && (self.rank == other.rank)
    }
}
impl Eq for Field {}

impl Field {
    pub fn new(file: char, rank: u8) -> Option<Field> {
        let file = file.to_ascii_uppercase();
        if file > 'H' || file < 'A' {
            None
        } else {
            if rank > 8 {
                None
            } else {
                Some(Field { file, rank })
            }
        }
    }

    pub fn get_top_neighbour(&self) -> Option<Field> {
        if self.rank == 8 {
            None
        } else {
            Field::new(self.file, self.rank + 1)
        }
    }

    pub fn get_bottom_neighbour(&self) -> Option<Field> {
        if self.rank == 1 {
            None
        } else {
            Field::new(self.file, self.rank - 1)
        }
    }

    pub fn get_right_neighbour(&self) -> Option<Field> {
        if self.file == 'H' {
            None
        } else {
            let new_file = std::char::from_u32(self.file as u32 + 1).unwrap_or(self.file);
            Field::new(new_file, self.rank)
        }
    }
    pub fn get_left_neighbour(&self) -> Option<Field> {
        if self.file == 'A' {
            None
        } else {
            let new_file = std::char::from_u32(self.file as u32 - 1).unwrap_or(self.file);
            Field::new(new_file, self.rank)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::board;
    use crate::field::Field;

    #[test]
    fn valid_field_creation_01() {
        let field1 = Field::new('a', 1);
        match field1 {
            Some(field) => {
                assert_eq!(field.file, 'A');
                assert_eq!(field.rank, 1);
            },
            None => panic!("Field was not created correctly")
        }
    }

    #[test]
    fn valid_field_creation_02() {
        let field1 = Field::new('h', 8);
        match field1 {
            Some(field) => {
                assert_eq!(field.file, 'H');
                assert_eq!(field.rank, 8);
            },
            None => panic!("Field was not created correctly")
        }
    }

    #[test]
    fn invalid_field_creation_01() {
        let field1 = Field::new('i', 1);
        match field1 {
            Some(field) => {
                panic!("Field was created despite being invalid")
            },
            None => ()
        }
    }

    #[test]
    fn invalid_field_creation_02() {
        let field1 = Field::new('d', 9);
        match field1 {
            Some(field) => {
                panic!("Field was created despite being invalid")
            },
            None => ()
        }
    }

    #[test]
    fn valid_field_neighbours() {
        let field1 = Field::new('c', 3).unwrap();
        match field1.get_bottom_neighbour() {
            Some(field) => {
                assert_eq!(field.file, 'C');
                assert_eq!(field.rank, 2);
            },
            None => panic!("Neighbour was not given correctly")
        }

        match field1.get_top_neighbour() {
            Some(field) => {
                assert_eq!(field.file, 'C');
                assert_eq!(field.rank, 4);
            },
            None => panic!("Neighbour was not given correctly")
        }

        match field1.get_left_neighbour() {
            Some(field) => {
                assert_eq!(field.file, 'B');
                assert_eq!(field.rank, 3);
            },
            None => panic!("Neighbour was not given correctly")
        }

        match field1.get_right_neighbour() {
            Some(field) => {
                assert_eq!(field.file, 'D');
                assert_eq!(field.rank, 3);
            },
            None => panic!("Neighbour was not given correctly")
        }
    }

    #[test]
    fn invalid_field_neighbours() {
        let field1 = Field::new('a', 1).unwrap();
        match field1.get_left_neighbour() {
            Some(field) => {
                panic!("Field was created despite being invalid")
            },
            None => ()
        }
        match field1.get_bottom_neighbour() {
            Some(field) => {
                panic!("Field was created despite being invalid")
            },
            None => ()
        }

        let field1 = Field::new('h', 8).unwrap();
        match field1.get_top_neighbour() {
            Some(field) => {
                panic!("Field was created despite being invalid")
            },
            None => ()
        }
        match field1.get_right_neighbour() {
            Some(field) => {
                panic!("Field was created despite being invalid")
            },
            None => ()
        }
    }
}