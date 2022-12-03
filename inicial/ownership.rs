fn main() {
    ownership_1();
    ownership_2();
    ownership_3();
}

fn ownership_1() {
    let uma_string = String::from("Fernando");
    let outra_string = rouba_1(uma_string);
    println!("{}", outra_string);
}

fn rouba_1(string: String) -> String {
    println!("{}", string);
    string
}

fn ownership_2() {
    let uma_string = String::from("Fernando");
    rouba_2(&uma_string);
    println!("{}", uma_string);
}

fn rouba_2(string: &String) {
    println!("{}", string);
}

fn ownership_3() {
    let mut uma_string = String::from("Fernando");
    rouba_3(&mut uma_string);
    println!("{}", uma_string);
}

fn rouba_3(string: &mut String) {
    string.push_str(" Rosa");
    println!("{}", string);
}
