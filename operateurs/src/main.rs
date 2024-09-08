fn main() {
    
    // -- LES OPERATEURS ARITHMETIQUES -- // 

    let x = 10;
    let y = 10;
    println!("{} + {} = {}", x, y, x + y); // affiche "10 + 10 = 20"
    println!("{} - {} = {}", x, y, x - y); 
    println!("{} * {} = {}", x, y, x * y);
    println!("{} / {} = {}", x, y, x / y);
    println!("{} % {} = {}", x, y, x % y);

    println!("{} + {} = {}", 5, 5, 5 + 5);
    println!("{} + {} = {}", x, y, 20 - 5);

    // 17 / 5 = 3 il reste 2
    // 17 % 5 = 2 <----- le reste 

    // [ opérateurs ]

    // bool : true/false
    // == > >= < <= !=

    let i: i8 = 5;
    let j: i8 = 10;
    println!("{}", i != j); //true
    println!("{}", i == j); // false
    println!("{}", i >= j); // false
    println!("{}", i > j); // false
    println!("{}", i < j); // true
    println!("{}", i <= j); // true

    println!("{}", "abc" > "abcd"); // false

    // impossible de comparer deux types différents 
    // println!("{}", 3 > 3.14); -> error

    let mut z: i8 = 11;
    z += 1;
    println!("{}", z); // affiche 12
    z -= 4; // 8
    z *= 2; // 16
    z /= 2; // 8
    println!("{}", z); // affiche 8

}
