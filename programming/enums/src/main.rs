// This is enums in Rust

//#[derive(Debug)] // This attribute allows us to print the enum variants using {:?}
fn main() {
    // Define a enum for IP address
    // enums that saves data directly in the variants 

    #[allow(dead_code)]
    enum IpAddr {
        V4(u8, u8, u8, u8), // This variant can hold a String data
        V6(String),// This variant can hold a String data
    }
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));
  
//     // 
//     enum IpAddKind {
//         V4,
//         V6
//     }
//     // Sruct
//     struct IpvAddr {
//         kind: IpAddKind,
//         address: String,
//     }

//     let home = IpvAddr {
//         kind : IpAddKind::V4,
//         address: String::from("127.0.0.1"),
//     }
//     let loopback = IpvAddr {
//         kind : IpAddKind::V6,
//         address: String::from("::1"),
//     };
//     // Using the Enums to store data directly
//     enum IpAddr2 {
//         V4(String),
//         V6(String),

// }
}