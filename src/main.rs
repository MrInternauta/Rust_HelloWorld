fn main() {
    println!("Hello, world!");
    let name = "Felipe de Jesus";
    println!("Hello {}.", name);
    let mut b_number: i32=10;
    println!("This is b mutable variable {}.", b_number);
    b_number = 100;
    println!("This is b mutable variable {}.", b_number);
    let a_number: i32=2;
    println!("This is a kind of constant {}.", a_number);
    let a_number: i32= a_number * 10;
    println!("This is variable use the same name but is other {}.", a_number);
    
    let age: u8 = 23;
    println!("My age is {}.", age);

    let bool: bool = true;
    println!("This is a boolean {}.", bool);

    let float: f32 = 2.5;
    println!("This is a float {}.", float);


    println!("Debbuging!");

    let name: &str = "Felipe de Jesus";
    println!("My name is {}.", name);

    let mut last_name: String = "Garcia".to_string();
    println!("My last name is {}.", last_name);
    last_name = "Rodriguez".to_string();
    println!("My last name is {}.", last_name);

    let char: char = 'a';
    println!("This is a char {}.", char);

    let tuple: (i32, f32, char, &str) = (1, 2.5, 'a',  "Hello");
    println!("This is first {}.", tuple.0);
    println!("This is last {}.", tuple.0);
    // Declare a tuple of three elements
    let tuple_e = ('E', 5i32, true);

    // Use tuple indexing and show the values of the elements in the tuple
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);



    struct Student { name: String, level: u8, remote: bool }

    // Tuple struct with data types only
    struct Grades(char, char, char, char, f32);

    // Unit struct
    struct Unit;
    //todo!("Display the message by using the println!() macro");

    // Instantiate classic struct, specify fields in random order, or in specified order
    let user_1 = Student { name: String::from("Constance Sharma"), remote: true, level: 2 };
    let user_2 = Student { name: ("Dyson Tan").to_string(), level: 5, remote: false };

    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
            user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
            user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);

}
