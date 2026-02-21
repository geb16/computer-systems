/** 
 * This is struct in Rust, which is a custom data type that allows you to group related data together.
 * In this example, we have a BankAccount struct with three fields: account_number, balance, and owner_name.
 * We also have methods for depositing money, withdrawing money, and checking the balance of the account.
 * The main function demonstrates how to create an instance of the BankAccount struct, 
 * perform some operations on it, and print the results.
 */


 fn main(){
    // tuple:
    //let rec = (200, 500);

    // Struct
    #[allow(dead_code)]
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    // another struct
    #[allow(dead_code)]
    struct User{
        active: bool,
        username: String,
        email: String,
        age: u32,
        sign_in_count: u64
    }

    // instanciating the struct
    let mut user1 = User {
        active: true,
        username: String::from("john_doe"),
        email: String::from("john@example.com"),
        age: 30,
        sign_in_count: 1
    };
    user1.email = String::from("john_updated@example.com");
    println!("User email: {}", user1.email);

    // Return a struct from a function
    fn build_user(username: String, email: String) -> User {
        User {
            active: true,
            username,
            email,
            age: 0,
            sign_in_count: 0
        }
    }
    let _user_from_fn = build_user(String::from("jane_doe"), String::from("jane@example.com"));
    
    // Create instances from other instances
    let user2 = User {
        email: String::from("jane@example.com"),
        ..user1 // ←
    };
    println!("User2 email: {}", user2.email);

    // Tuple struct
    #[allow(dead_code)] // means that the struct is not used in the code, but we want to keep it for future use
    struct Color(i32, i32, i32);
    #[allow(dead_code)]
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _white = Color(255, 255, 255);
    let _origin = Point(0, 0, 0);
    
    // Unit-Like Struct
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}