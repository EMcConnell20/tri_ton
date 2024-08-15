#[test]
fn tri_fall() {
    // Tri-Fall
    tri!(Some(5) => None <> ());
    tri!(5 => [..6] <> ());
    tri!(tri_fail_and_return(Some(true)) => Ok(_) <> ());
    
    for person in PEOPLE {
        let name = tri!(person.name => Some(name) <> "?");
        tri!(person.age => Some[age @ ..120] <> 0);
        tri!(person.height => Some[mut height @ (..7, ..12)] <> (0, 0));
        
        // Redundant, but uses mutability.
        height.0 += 1;
        
        print_person(name, age, height);
    }
}

#[test]
fn tri_until() {
    let mut number: u8 = 0;
    
    println!("Using %> Operator\n");
    
    // Tri-Until
    println!("Current Value: {number}");
    tri!(number => [5] %> number += 1);
    println!("Current Value: {number}");
    tri!(number => [5] %> number += 1);
    println!("Current Value: {number}\n");
}

#[test]
fn tri_while() {
    let mut number: u8 = 0;
    
    println!("Using >> Operator\n");
    
    // Tri-While
    println!("Current Value: {number}");
    tri!(number => [..5] >> number += 1);
    println!("Current Value: {number}");
    tri!(number => [..5] >> number += 1);
    println!("Current Value: {number}\n");
    
    println!("Using >> Operator With Final Bind\n");
    
    tri! {
        number_mangler(Some(value)) =>
        Some[value = 0] >>
        { println!("Current Value: {value}") }
    }
    
    println!("Final Value: {value}\n");
    
    // Abstract
    tri!(None::<()> => Some(_) >> println!("This Line Executed Once"));
}

fn tri_fail_and_return(item: Option<bool>) -> Result<&'static str, &'static str> {
    tri!(item => Some(a @ true) -> "Item was either None or False.");
    tri!(item => Some[var_name] #> Err("Item was None."));
    
    if var_name { Ok("Item was True.") }
    else { Err("Item was False.") }
}

fn print_person(name: &str, age: u8, height: (u8, u8)) {
    println!("Name: {name}");
    println!("Age: {age}");
    println!("Height: {0}\'{1}\"", height.0, height.1);
    println!();
}

fn number_mangler(item: Option<u8>) -> Option<u8> {
    tri!(item => Some[var_name @ ..20] #> None);
    Some(var_name + 1)
}

#[derive(Copy, Clone, Debug)]
struct Person {
    name: Option<&'static str>,
    age: Option<u8>,
    height: Option<(u8, u8)>,
}

const PEOPLE: [Person; 3] = [
    Person {
        name: Some("Jerry Jones"),
        age: Some(37),
        height: None,
    },
    Person {
        name: Some("Ron"),
        age: Some(142),
        height: Some((5, 11)),
    },
    Person {
        name: None,
        age: Some(20),
        height: Some((9, 2)),
    },
];
