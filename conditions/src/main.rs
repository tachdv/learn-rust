fn main() {
    
    let x = -5;
    let condition = x > 0;

    if x > 0 { 
        println!("le nombre est positif"); //non executée car x = -5
    } 
    else {
        println!("le nombre est négatif"); //exécutée car x = -5
    }

    println!("{}", condition); // affiche true

    let age = 18;

    if age > 18 {
        println!("La personne est majeure");
    } 
    else if  age < 18{
        println!("La personnne est mineure");
    }
    else {
        println!("La personne vient d'atteindre la majorité"); // cette condition sera exécutée 
    }

    let i = 7;
    let j = if i > 5 { true } else { false };
    
    if j { println!("La condition est remplie ! ( j = {} )", j) } // exécutée

}
