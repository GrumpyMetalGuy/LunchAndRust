fn taking_ownership(a_bunch_of_numbers: Vec<i32>) -> () {
    println!("This is the list {:?}", a_bunch_of_numbers);
}

fn borrowing_ownership(a_bunch_of_numbers: &Vec<i32>) -> () {
    println!("This is the list {:?}", a_bunch_of_numbers);
}

fn modifying_ownership(a_bunch_of_numbers: &mut Vec<i32>) -> () {
    a_bunch_of_numbers.push(17);

    println!("This is the list {:?}", a_bunch_of_numbers);
}

fn main() {
    let x = vec![14, 15, 16];

    taking_ownership(x);

    // Uncommenting this will work, as the function borrowing_ownership only borrows the param
    // borrowing_ownership(&x);

    // Uncommenting this and making x mutable will work, as the function modifying_ownership gets a mutable reference
    // modifying_ownership(&mut x);

    // The following will fail to compile if we call taking_ownership, as it has done what it says on the tin!
    // println!("Printing {:?} now", x);
}
