struct Foo<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

// struct Foo {
//     x: &i32,
//     y: &i32,
// }

fn main() {

    let x = 6;
    let m;                     

    {                          
        let y = 6;            
        let f = Foo { x: &x, y: &y };  
        m = f.x;             
    }                          

    println!("{}", m);        
}