// the length of the string should be greater than 1 byte
// to see the deallocations!
fn giving_ownership() -> String {
    let string = String::from("Hello world!");
    println!("giving_ownership: {:p}", string.as_ptr());
    string
}

fn taking_ownership() {
    let string = giving_ownership();
    println!("taking_ownership: {:p}", string.as_ptr());
}

fn passing_ownership() {
    let string = String::from("Hello world!");
    println!("passing_ownership before: {:p}", string.as_ptr());
    let string = taking_and_returning_ownership(string);
    println!("passing_ownership after: {:p}", string.as_ptr());
}

fn taking_and_returning_ownership(string: String) -> String {
    println!("taking_and_returning_ownership: {:p}", string.as_ptr());
    string
}

fn passing_ownership_with_ref() -> std::vec::Vec<*const u8> {
    let mut pointers: Vec<*const u8> = Vec::new();
    let string = String::from("Hello world!");
    pointer_utils("passing_ownership_with_ref before", string.to_string(), &mut pointers);
    borrowing_ownership_with_ref(&string, &mut pointers);
    pointer_utils("passing_ownership_with_ref after", string.to_string(), &mut pointers);
    pointers
}

fn borrowing_ownership_with_ref(string: &String, pointers: &mut Vec<*const u8>) {
    pointer_utils("borrowing_ownership_with_ref", string.to_string(), pointers);
}

fn mut_simple() -> std::vec::Vec<*const u8> {
    let mut pointers: Vec<*const u8> = Vec::new();
    let mut string = String::from("Hello world!");
    pointer_utils("mut_simple before", string.to_string(), &mut pointers);
    string.push_str(".");
    pointer_utils("mut_simple after", string.to_string(), &mut pointers);
    pointers
}

fn mut_ref_owner() -> std::vec::Vec<*const u8> {
    let mut pointers: Vec<*const u8> = Vec::new();
    let mut string = String::from("Hello world!");
    pointer_utils("mut_ref_owner before", string.to_string(), &mut pointers);
    // mutate
    mut_ref_borrower(&mut string, &mut pointers);
    
    pointer_utils("mut_ref_owner after", string.to_string(), &mut pointers);
    pointers
}

fn mut_ref_borrower(string: &mut String, pointers: &mut Vec<*const u8>) {
    pointer_utils("mut_ref_borrower", string.to_string(), pointers);
    // mutate the string
    string.push_str(".");
}

fn pointer_utils(text: &str, string: String, pointers: &mut Vec<*const u8>) {
    let pointer = string.as_ptr();
    println!("{}: {:p}", text, pointer);
    pointers.push(pointer);
}


fn main() {
    taking_ownership();
    passing_ownership();
    passing_ownership_with_ref();
    mut_simple();
    mut_ref_owner();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mut_ref_owner_test() {
        let results = mut_ref_owner();
        let p0 = results[0];
        let p1 = results[1];
        let p2 = results[2];
        assert_eq!(p0, p1);
        assert_ne!(p1, p2);
    }

    #[test]
    fn mut_simple_test() {
        let results = mut_simple();
        let p0 = results[0];
        let p1 = results[1];
        assert_ne!(p0, p1);
    }

    #[test]
    fn passing_ownership_with_ref_test() {
        let results = passing_ownership_with_ref();
        let p0 = results[0];
        let p1 = results[1];
        let p2 = results[2];
        assert_eq!(p0, p1);
        assert_eq!(p1, p2);
    }
}
