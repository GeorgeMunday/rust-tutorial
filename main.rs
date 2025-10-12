fn main() {
    let day = 4; // variable 
    let mut count = 1; // variable that changes
    loop { // loop
        match day {    // match
        1 | 2 | 3 | 4 | 5 => println!("Weekday"), 
        6 | 7 => println!("Weekend"),         
        _ => println!("Invalid day"),             
        }

        if day == 3 {
            println!("yipee")
        }
        if count == 2 {
            break
        }
        count += 1
    }
}
