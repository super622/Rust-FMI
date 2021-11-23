use solution::*;

fn string_cell_vec(s1: &str, s2: &str, s3: &str, s4: &str) -> Vec<Cell<String>> {
    [s1, s2, s3, s4]
        .into_iter()
        .map(String::from)
        .map(Cell)
        .collect::<Vec<Cell<String>>>()
}

#[test]
fn test_cell_multiply(){
    assert_eq!((Cell(3) * Cell(String::from("Max"))).0, String::from("MaxMaxMax"));

    assert_eq!((Cell(-3) * Cell(String::from("emoseerht"))).0, String::from("threesomethreesomethreesome"));

    assert_eq!((Cell(0) * Cell(String::from("alibaba"))).0, String::from(""));

    assert_eq!((Cell(-5) * Cell(String::from(" "))).0, String::from("     "));
}

#[test]
fn test_cell_add(){
    assert_eq!((Cell(7000000) + Cell(String::from("e naselenieto na BG"))).0, String::from("7000000 e naselenieto na BG"));
   
    assert_eq!((Cell(-700) + Cell(String::from("otonizak v atuban ohcvI"))).0, String::from("Ivcho nabuta v kazinoto 700"));

    assert_eq!((Cell(0) + Cell(String::from("fuck given"))).0, String::from("0 fuck given"));

    assert_eq!((Cell(-21) + Cell(String::from(" "))).0, String::from("  21"));

    assert_eq!((Cell(3) + ( Cell(6) + (Cell(0) + Cell(String::from("No scope"))))).0, String::from("3 6 0 No scope"));
}

#[test]
fn test_matrix(){
    let matrix1 = Matrix::new(&[-1, 0, -2, 0]);
    let matrix2 = Matrix::new(&[
        String::from(" "),
        String::from("test"),
        String::from("legna"),
        String::from(" "),
    ]);

    let result = Matrix::new(&[
        String::from("  1"),
        String::from("angel 2"),
        String::from("0 test"),
        String::from("0  "),
    ]);

    assert_eq!((matrix1 + matrix2).by_col(), result.by_row());

    let m1 = Matrix::new(&[0, -2, -1, 4]);
    let m2 = Matrix::new(&[
        String::from("longlonglonglonglonglongthing"),
        String::from("krow"),
        String::from("krow"),
        String::from("letsgo"),
    ]);

    assert_eq!((m1 * m2), String::from(" workwork work letsgoletsgoletsgoletsgo"));
}

#[test]
#[should_panic]
fn test_panic(){
    let mut cell = Cell(151262523) * Cell(String::from(""));
    assert_eq!(cell.0, "");

    cell = Cell(13) * (Cell(-1) + cell);
    assert_eq!(cell.0, "1 1 1 1 1 1 1 1 1 1 1 1 1");
}