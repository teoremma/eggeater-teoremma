use std::env;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64) -> u64;
}

#[no_mangle]
#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    eprint!("error {}: ", errcode);
    match errcode {
        7 => eprintln!("invalid argument for ="),
        9 => eprintln!("invalid argument for arithmetic op"),
        11 => eprintln!("overflow"),
        _ => eprintln!("an error ocurred"),
    }
    std::process::exit(1);
}

#[no_mangle]
#[export_name = "\x01snek_print"]
// TODO: might need to return an u64 for some reason
fn snek_print(val: i64) -> i64 {
    if val % 2 == 0 { println!("{}", (val as i64) >> 1)}
    else if val == 1 { println!("false") }
    else if val == 3 { println!("true") }
    else { println!("Unknown value: {}", val) }
    return val;
}

fn parse_input(input: &str) -> u64 {
    if input == "true" { 3 }
    else if input == "false" { 1 }
    else if let Ok(i) = input.parse::<i64>() {
        if i <= (1i64 << 62 - 1) || i >= -(1i64 << 62) { (i as u64) << 1 }
        else { panic!("Invalid input: overflow {}", input) }
    }
    else { panic!("Invalid input: {}", input) }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let i: u64 = unsafe { our_code_starts_here(input) };
    snek_print(i as i64);
}