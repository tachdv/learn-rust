fn main() {

    // -- VARIABLES -- //

    // [ variables let ] :

    let mut x = 10; // grâce au mot clé 'mut', x deviens modifiable
    let y = 5;
    println!("x vaut {x}"); // affiche 10
    println!("y vaut {y}"); // affiche 5
    x = 11; // nouvelle attribution de valeur 
    println!("x vaut {x}"); // affiche 11
    // y = 11; --> erreur : pas de mot clé 'mut' donc non modifiable 

    // [ shadowing ] :

    let i = 10;
    println!("i vaut {i}"); // affiche 10
    let i = i + 1; // écrase la dernière variable pour faire une nouvelle variable 'i'
    println!("i vaut {i}"); // affiche 11

    {
        let i = i - 6;
        println!("i vaut {i}"); // affiche 5
    }

    println!("i vaut {i}"); // affiche 11 car dans le block du dessus la valeur est unique à celui ci 

    // [ variables constantes ] :

    const X_CONST: i32 = 10; // obliger d'attribuer un type à une constante et par convention on les écrit en majuscules 
    println!("La valeur de X_CONST vaut {X_CONST}"); // affiche 10

    println!("{}, {}", "Hello", "world !"); // affiche "Heello, world!"

}
