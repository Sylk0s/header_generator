use std::io;
use physics_header::Header;

fn main() {
    let config = physics_header::load_config();

    let mut helper = String::new();
    let mut program_name = String::new();
    let mut due_day = String::new();
    let mut due_month = String::new();
    let mut purpose = String::new();
    let mut bugs = String::new();

    println!("Sources used for help: ");
    io::stdin().read_line(&mut helper).expect("Line read failed");   
    println!("Name of the program: ");
    io::stdin().read_line(&mut program_name).expect("Line read failed");
    println!("Day the assignment is due: ");
    io::stdin().read_line(&mut due_day).expect("Line read failed");
    println!("Month the assignment is due: ");
    io::stdin().read_line(&mut due_month).expect("Line read failed");
    println!("Describe the program: ");
    io::stdin().read_line(&mut purpose).expect("Line read failed");
    println!("Describe any bugs: ");
    io::stdin().read_line(&mut bugs).expect("Line read failed");
    
    // lazy hardcoding class year because I can't be bothered and wont use this after this class is over in May
    let due = due_month.trim().to_owned() + "/" + &due_day.trim() + "/" + "2022";
    let header = Header::new(config,helper,program_name,due,purpose,bugs);
    println!("{}",header);
}