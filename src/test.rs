
use xi_rope::Rope;

#[test]
fn test_add() 
{
    let mut a = Rope::from("hello.");
    a.edit(5..6, "!blah");

    println!("streng : {}", String::from(a));
   
}