use solution::*;

#[test]
fn test_basic_1() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Hallway").unwrap();
    dungeon.set_link("Entrance", Direction::East, "Hallway").unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway");
}

const TEST_INPUT_1: &str = "
## Rooms
- Entrance
- Hallway

## Links
- Entrance -> East -> Hallway
";

#[test]
fn test_basic_2() {
    // .trim() за да премахнем първия и последния ред:
    let dungeon = Dungeon::from_reader(TEST_INPUT_1.trim().as_bytes()).unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_room("Hallway").unwrap().name, "Hallway");

    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway");
}

#[test]
fn test_basic_3() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Treasure Room").unwrap();
    dungeon.set_link("Entrance", Direction::West, "Treasure Room").unwrap();

    let path = dungeon.find_path("Entrance", "Treasure Room").unwrap().unwrap();
    assert!(path.len() > 0);
}

const TEST_INPUT_2: &str = "
## Rooms
- Entrance
- Hallway
- Bedroom
- Kitchen

## Links
- Entrance -> East -> Hallway
- Entrance -> West -> Bedroom
- Bedroom -> South -> Kitchen
- Entrance -> East -> Kitchen
";

const TEST_INPUT_3: &str = "
## Rooms
- Entrance
- Bedroom

## Links
- Entrance -> West -> Masters_Bedroom
- Bedroom -> South -> Bedroom
";

#[test]
fn test_1() {
    let dungeon = Dungeon::from_reader(TEST_INPUT_2.trim().as_bytes()).unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_room("Hallway").unwrap().name, "Hallway");
    assert_eq!(dungeon.get_room("Bedroom").unwrap().name, "Bedroom");
    assert_eq!(dungeon.get_room("Kitchen").unwrap().name, "Kitchen");

    assert_eq!(
        dungeon
            .get_next_room("Hallway", Direction::West)
            .unwrap()
            .unwrap()
            .name,
        "Entrance"
    );
    assert_eq!(
        dungeon
            .get_next_room("Entrance", Direction::East)
            .unwrap()
            .unwrap()
            .name,
        "Kitchen"
    );
}

#[test]
#[should_panic]
fn test_2() {
    let dungeon = Dungeon::from_reader(TEST_INPUT_2.trim().as_bytes()).unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_room("Hallway").unwrap().name, "Hallway");
    assert_eq!(dungeon.get_room("Bedroom").unwrap().name, "Bedroom");
    assert_eq!(dungeon.get_room("Kitchen").unwrap().name, "Kitchen");

    assert_eq!(
        dungeon
            .get_next_room("Entrance", Direction::East)
            .unwrap()
            .unwrap()
            .name,
        "Hallway"
    );
}

#[test]
#[should_panic]
fn test_3() {
    let dungeon = Dungeon::from_reader(TEST_INPUT_3.trim().as_bytes()).unwrap();

    assert_eq!(dungeon.get_room("Bedroom").unwrap().name, "Bedroom");
}

#[test]
fn test_4() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Hallway").unwrap();
    dungeon.add_room("Kitchen").unwrap();
    dungeon.add_room("Bedroom").unwrap();
    dungeon.add_room("Maze").unwrap();
    dungeon.add_room("Basement").unwrap();

    dungeon
        .set_link("Entrance", Direction::West, "Hallway")
        .unwrap();
    dungeon
        .set_link("Kitchen", Direction::North, "Hallway")
        .unwrap();
    dungeon
        .set_link("Hallway", Direction::South, "Bedroom")
        .unwrap();
    dungeon
        .set_link("Maze", Direction::East, "Bedroom")
        .unwrap();
    dungeon
        .set_link("Hallway", Direction::West, "Basement")
        .unwrap();

    let path = dungeon.find_path("Entrance", "Maze").unwrap().unwrap();

    let mut vec: Vec<&Room> = Vec::new();
    vec.push(dungeon.get_room("Entrance").unwrap());
    vec.push(dungeon.get_room("Hallway").unwrap());
    vec.push(dungeon.get_room("Bedroom").unwrap());
    vec.push(dungeon.get_room("Maze").unwrap());

    assert_eq!(path.len(), vec.len());

    assert_eq!(
        dungeon
            .get_next_room("Basement", Direction::East)
            .unwrap()
            .unwrap()
            .name,
        String::from("Hallway")
    );
}

#[test]
fn test_5() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Hallway").unwrap();
    dungeon.add_room("Kitchen").unwrap();
    dungeon.add_room("Bedroom").unwrap();
    dungeon.add_room("Maze").unwrap();
    dungeon.add_room("Basement").unwrap();

    dungeon
        .set_link("Entrance", Direction::West, "Hallway")
        .unwrap();
    dungeon
        .set_link("Kitchen", Direction::North, "Hallway")
        .unwrap();
    dungeon
        .set_link("Hallway", Direction::South, "Bedroom")
        .unwrap();
    dungeon
        .set_link("Maze", Direction::East, "Bedroom")
        .unwrap();
    dungeon
        .set_link("Hallway", Direction::West, "Basement")
        .unwrap();

    let path = dungeon.find_path("Entrance", "Maze").unwrap().unwrap();

    let mut names: Vec<String> = Vec::new();
    for room in &path {
        names.push(room.name.clone());
    }

    assert_eq!(names, vec!["Entrance", "Hallway", "Bedroom", "Maze"]);
}

#[allow(unused)]
const OWN_TESTS_INPUT_11: &str = "
## Rooms
- 1
- 2
- 3
- 4
- 5
- 6
- 7
- 8
- 9
- 10

## Links
- 1 -> East -> 2
- 10 -> West -> 6
- 9 -> North -> 6
- 3 -> North -> 8
- 4 -> South -> 8
- 4 -> East -> 7
- 7 -> South -> 2
- 5 -> East -> 9
- 5 -> North -> 3
";

#[test]
fn own_tests_test_11() {
    let dungeon: Dungeon = Dungeon::from_reader(OWN_TESTS_INPUT_11.trim().as_bytes()).unwrap();
    assert_eq!(dungeon.get_room("1").unwrap().name, "1");
    assert_eq!(dungeon.get_room("2").unwrap().name, "2");
    assert_eq!(dungeon.get_room("10").unwrap().name, "10");

    let result_path = dungeon.find_path("1", "10").unwrap().unwrap();
    let mut rooms: Vec<String> = Vec::new();
    for room in result_path {
        rooms.push(room.name.to_string());
    }
    assert_eq!(rooms, vec!["1", "2", "7", "4", "8", "3", "5", "9", "6", "10"]);
    /*
            4------7
            |      |
            8\1#---2/10#
            |         |
            3         6
            |         |
            5---------9
     */
}