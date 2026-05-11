# Quette
les type dans rust documenter et comprendre. '

##### Les entier  servent a stoker les nombres sans virgule.
***Les signé (Ils peuvent etre negatif)***
`i8, i16, i32, i64, i128, isize`

***Les nons signe ne peuvent que etre positif***
`u8, u16, u32, u64, u128, usize`

###### Les Flottants
Pour les nobres decimaux
`f32, f64`

##### Booléen
Deuxvaleurs possibles

##### Caractere
Unseul caracter unicode
toujours dans des simples cotes ''

#### Les types composé

##### Les Tuples
peuvent contenire different types
```rust
let t: (i32, f64, char) = (10, 3.14, 'R');
```
Acces 
```rust
let x = t.0;
let y = t.1;
```
##### Les Tableau
Meme type dans les valeurs et taille fixe
```rust
let arr: [i32; 3] = [1, 2, 3];
```
Acces
```rust
let first = arr[0];
```
#### Type Text
##### `&str` String slice
Chaine imuable souvant stoker dans le binaire.
```rust
let s: &str = "Bonjour";
```
##### `String`
Chaine dynamique
```rust
let mut s = String::from("Hello");
s.push_str(" world");
```

#### Type Personalises

##### struct (structure)
Permet de creer ses propres types
```rust
struct User {
    name: String,
    age: u32,
}
```
***Utilisation***
```rust
let u = User {
    name: String::from("Alex"),
    age: 20,
};
```
##### enum (enumeration)
Represente un ensemble de variante possibles
```rust
enum Status {
    Active,
    Inactive,
    Pending,
}
```
***Avec donnees***
```rust
enum Message {
    Quit,
    Move(i32, i32),
    Write(String),
}
```

#### Option et Result
##### Option <T>
Represente une valeur possible ou absente
```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;
```

##### Result<T, E>
Gestion des erreurs
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
***Exemple***
```rust
fn div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division par zéro".to_string())
    } else {
        Ok(a / b)
    }
}
```

#### Pointeur inteligents
Rust gère la mémoire sans garbage collector grâce à eux.

`Box<T>` → allocation heap simple
`Rc<T>` → partage de propriété
`RefCell<T>` → mutabilité intérieure

#### Reference et emprunt
```rust
let s = String::from("hello");
let r = &s; // référence
```
`&T `-> référence immuable
`&mut T` -> référence mutable