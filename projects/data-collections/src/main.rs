fn main() {
    // ==> tuples <==
    // tuple of lenght 3
    let tuple_e = ('E', 5i32, true);
    println!("Is '{}' the {}th letter of alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);

    // ==> structs <==
    
    // Classic struct with named fields
    struct Student {
        name: String,
        level: u8,
        remote: bool
    }

    // Tuple struct with data types only
    struct Grades (
        char,
        char,
        char,
        char,
        f32
    );

    // Unit Struct
    // struct Unit;

    // ==>> Instantiate Struct
    // Instantiate classic struct, specify fields in random order, or in specified order
    let user_1 = Student {
        name: String::from("Victor"),
        remote: true,
        level: 5
    };

    let user_2 = Student {
        name: String::from("Jhon doe"),
        level: 2,
        remote: false
    };

    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1 = Grades('A', 'B', 'A', 'A', 3.75);
    let mark_2 = Grades('A', 'B', 'B', 'A', 3.25);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3,  mark_1.4);
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3,  mark_2.4);
}
