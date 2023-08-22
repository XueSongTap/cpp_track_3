struct Foo<'a> {
    x: &'a i32,
    y: &'a i32,
}

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