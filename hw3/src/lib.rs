use std::collections::HashMap;
use std::io::BufRead;
use std::collections::LinkedList;


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
#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Една стая в подземията. Дефинира се само с име, макар че в по-интересна имплементация може да
/// държи item-и, противници...
///
pub struct Room {
    pub name: String,
    pub neighbours: [Option<String>; 4],
    //[0-north, 1-south, 2-east, 3-west];
    // Каквито други полета ви трябват
}

/// Контейнер за стаите и не само. Ще работим предимно със тази структура.
///
pub struct Dungeon {
    pub rooms: HashMap<String,Room>
}

impl Dungeon {
    /// Конструиране на празен Dungeon, в който няма никакви стаи.
    ///
    pub fn new() -> Self {
        Dungeon{
            rooms: HashMap::new()
        }
    }

    /// Добавяне на стая към Dungeon с име `name`. Връща `Ok(())` при успех. Ако вече има стая с
    /// такова име, очакваме да върнете `Errors::DuplicateRoom` с името.
    ///
    pub fn add_room(&mut self, name: &str) -> Result<(), Errors> {
        match self.rooms.contains_key(name){
            true => return Err(Errors::DuplicateRoom(String::from(name))),
            false => {
                self.rooms.insert(String::from(name), self::Room { name: String::from(name), neighbours: [None, None, None, None] });
                return Ok(())
            }
        }
    }

    /// Прочитане на дадена стая -- когато извикаме `get_room`, очакваме reference към `Room`
    /// структурата с това име.
    ///
    /// Ако няма такава стая, очакваме `Errors::UnknownRoom` с подаденото име.
    ///
    pub fn get_room(&self, room_name: &str) -> Result<&Room, Errors> {
        match self.rooms.get(room_name) {
            None => return Err(Errors::UnknownRoom(String::from(room_name))),
            Some(room) => return Ok(room),
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

        let mut direction: Direction = direction;

        if self.rooms.contains_key(room_name) == false {
            return Err(Errors::UnknownRoom(String::from(room_name)));
        }
        else if self.rooms.contains_key(other_room_name) == false {
            return Err(Errors::UnknownRoom(String::from(other_room_name)));
        }
        else{
            match direction{
                Direction::North => {
                    self.rooms.get_mut(room_name).unwrap().neighbours[0] = Some(String::from(other_room_name));
                    direction = Direction::South; 
                },
                Direction::South => {
                    self.rooms.get_mut(room_name).unwrap().neighbours[1] = Some(String::from(other_room_name));
                    direction = Direction::North;
                },
                Direction::East => {
                    self.rooms.get_mut(room_name).unwrap().neighbours[2] = Some(String::from(other_room_name));
                    direction = Direction::West;
                },
                Direction::West => {
                    self.rooms.get_mut(room_name).unwrap().neighbours[3] = Some(String::from(other_room_name));
                    direction = Direction::East
                }
            }
            
            let position: usize = match direction {
                Direction::North => 0,
                Direction::South => 1,
                Direction::East => 2,
                Direction::West => 3,
            };
            
            self.rooms.get_mut(other_room_name).unwrap().neighbours[position] = Some(String::from(room_name));
            
            Ok(())
        }


    }
    // North,
    // South,
    // East,
    // West,
    
    /// Прочитаме структурата на dungeon от нещо, което имплементира `BufRead`. Това може да е
    /// файл, или, ако тестваме, може да е просто колекция от байтове.
    ///
    /// Успешен резултат връща новосъздадения dungeon, пакетиран в `Ok`.
    ///
    /// Вижте по-долу за обяснение на грешките, които очакваме.
    ///
    pub fn from_reader<B: BufRead>(reader: B) -> Result<Self, Errors> {
        let mut line = String::new();
        let mut dungeon: Dungeon = Dungeon::new();
        let mut counter: usize = 1;
        let mut cmd_flag: i32 = 0;
        let mut reader = reader;

        while reader.read_line(&mut line).unwrap() != 0 {
            if line == "## Rooms\n" && counter == 1 {
                cmd_flag += 1;
            }
            else if line == "\n" {
                counter += 1;
                line.clear();
                continue;    
            }
            else if line == "## Links\n" && cmd_flag == 1 {
                cmd_flag += 1;
            }
            else if cmd_flag == 1 && line.find("- ") == Some(0) && line.find(" -> ") == None {
                line = line.split_off(2);
                let popped: char = line.pop().unwrap();
                if popped != '\n' {
                    line.push(popped);  //checking if the line ends with \n
                }  
                
                dungeon.add_room(line.as_str())?;
            }
            else if cmd_flag == 2 && line.find("- ") == Some(0) && line.find(" ->") != None    {
                line = line.split_off(2);
                let popped: char = line.pop().unwrap();
                if popped != '\n' {
                    line.push(popped); //checking if the line ends with \n
                }  

                let parameters: Vec<&str> = line.split_terminator(" -> ").collect();
                let room_name: &str = parameters.get(0).unwrap();
                let direction: &str = parameters.get(1).unwrap();
                let other_room_name: &str = parameters.get(2).unwrap();

                let direction: Direction = match direction{
                    "East" => Direction::East,
                    "West" => Direction::West,
                    "North" => Direction::North,
                    "South" => Direction::South,
                    _       => unreachable!()
                };

                dungeon.set_link(room_name, direction, other_room_name)?;
            }
            else{
                return Err(Errors::LineParseError { line_number: counter });
            }

            counter += 1;
            line.clear();
        }

        Ok(dungeon)
    }
    

    /// Четене на съседа на стаята с име `room_name` в посока `direction`. Тук има няколко
    /// варианта на изход:
    ///
    /// - Ако подадената стая не съществува, очакваме грешка `Errors::UnknownRoom`
    /// - Ако подадената стая няма съсед в тази посока, Ok(None) е смисления резултат
    /// - Иначе, чакаме reference към `Room` структурата на въпросния съсед, опакована в `Ok(Some(`.
    ///
    pub fn get_next_room(&self, room_name: &str, direction: Direction) -> Result<Option<&Room>, Errors> {
        if self.rooms.contains_key(room_name) == false {
            return Err(Errors::UnknownRoom(String::from(room_name)));
        }
        else{
            let position: usize = match direction {
                Direction::North => 0,
                Direction::South => 1,
                Direction::East => 2,
                Direction::West => 3,
            };

            match &self.rooms.get(room_name).unwrap().neighbours[position]{
                Some(r) => return Ok(Some(self.rooms.get(&String::from(r)).unwrap())),
                None => return Ok(None)
            }
        }
    }

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
        end_room_name: &str
    ) -> Result<Option<Vec<&Room>>, Errors> {
        
        self.get_room(start_room_name)?;
        self.get_room(end_room_name)?;

        let mut path: Vec<&Room> = Vec::new();
        let mut parenting: HashMap<&str, &str> = HashMap::new();
        let mut visited: HashMap<&str,bool> = HashMap::new();
        let mut to_visit = LinkedList::new();
        to_visit.push_back(start_room_name);
        visited.insert(start_room_name, true);

        while to_visit.is_empty() != true {
            let room = to_visit.pop_front().unwrap();
            
            if room == end_room_name {
                break;
            }

            for neigh in &self.rooms.get(&String::from(room)).unwrap().neighbours{
                match neigh {
                    None => continue,
                    Some(a) => Some(a) 
                };
                
                if visited.contains_key(neigh.as_ref().unwrap().as_str()) == false {
                    to_visit.push_back(&neigh.as_ref().unwrap().as_str());
                    visited.insert(&neigh.as_ref().unwrap().as_str(), true);
                    parenting.insert(&neigh.as_ref().unwrap().as_str(), room);
                }
            }
        }

        if parenting.contains_key(end_room_name) == false {
            return Ok(None);
        }

        let mut curr = end_room_name;
        let room = self.get_room(curr).unwrap();

        path.push(room);
        while parenting.contains_key(curr) {
            curr = parenting.get(curr).unwrap();
            let neigh = self.get_room(curr).unwrap();

            path.push(neigh);
        }

        path.reverse();
        Ok(Some(path))
    }

}