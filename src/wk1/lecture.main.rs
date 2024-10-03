fn main() {
    //integer
    let a = 32;
    println!("Value: {:?}", a);

    //array
    let b: [i32; 3] = [1, 2, 3];
    println!("Value: {:?}", b);

    //vector
    let mut c: Vec<i32> = vec![1, 2, 3];
    c.push(4);
    println!("Value: {:?}", c);
    println!("Size: {:?}", c.len());
    println!("Capacity: {:?}", c.capacity());

    //string
    let d = "Hello, world!";
    println!("Value: {:?}", d);

    //char
    let e = 'a';
    println!("Value: {:?}", e);

    //function
    let mut s: String = String::from("Hello, world!");
    s = test(s);

    //clone
    let s2 = s.clone();
    println!("Value: {:?}", s2);

    //mutable
    s.push_str(" Hello, Rust!");
    println!("Value: {:?}", s);

    let hello = &s[0..5];
    println!("Value: {:?}", hello);
    let world = &s[7..12];
    println!("Value: {:?}", world);

    s.push_str(" Hello, Rust!");
    println!("Value: {:?}", s);

    //pointer
    let ptr = s.as_ptr();
    println!("Pointer: {:p}", ptr);

    //tuples
    let t = (500, 6.4, 1);
    let (x, y, z) = t;
    println!("Value: {:?}", x);
    println!("Value: {:?}", y);
    println!("Value: {:?}", z);

    let t: (i32, f64, u128) = (500, 6.4, 1);
    let (x, y, z) = t;
    println!("Value: {:?}", x);
    println!("Value: {:?}", y);
    println!("Value: {:?}", z);

    //scope
    let mut scope = 10;
    {
        //start of scope
        scope = 11;

        let _scope = 1234;
        //end of scope
    }
    println!("Value: {}", scope);

    //hashmap
    use std::collections::HashMap;

    let mut test: HashMap<i32, HashMap<_, _>> = HashMap::new();
    let mut test2: HashMap<i32, f32> = HashMap::new();

    test.insert(1, test2);

    test.get_mut(&1).unwrap().insert(123, 123.4);

    println!("{:?}", test.get(&1).unwrap());
    println!("{:?}", test.get(&2));
}

fn test(var: String) -> String {
    println!("Value: {}", var);

    let ptr = var.as_ptr();
    println!("Pointer: {:p}", ptr);

    var
}

/*
int8,
int16,
int32,
int64,
int128,
...
int248,
int256,


u8,
u16,
u32,
u64,
u128,
*/
