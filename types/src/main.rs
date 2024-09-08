fn main() {

    // -- SCARLAR TYPES -- //

    // integers, floatting-point, booleans, characters 

    let x = 5; // i32
    // il y a aussi i8, i16, i64, i128

    let i: i8 = 5; // signé (peut être négatif)
    let j: u8 = 5; // non signé, ne peut être négatif
    // avec i8 la rangée est partagée : -125 > 127 ; on fais (2^8) / 2
    // avec u8, on peut aller au delà de 255 -> 2^8 = 256 (la range étant : 0 -> 255)

    // 16 : 16^2 = 65535 

    // impossible de calculer un i32 avec un i32, ce n'est pas le même type

    let h: i32 = 1_954_1354; //possiblité de mettre des tirets du bas sur les gros nombres pour que ce soit plus clair

    //les décimaux sont obligatoirement signé
    let dec = 2.5;
    println!("{}", dec - 1.0); // affiche 1.5

    let boolean: bool = true; 
    println!("{}", boolean); // affichera 'true'

    let ch1: char = 'z'; // char -> ''
    let ch2: &str = "z"; // str -> "" 

    // [ notations identifiant ]

    // base 10: 1_987_839
    // base 16 (hexa) : 0x15f
    // base 8 (octal) : 0o123
    //base 2 (binaire) : 0b101110001 
    
}
