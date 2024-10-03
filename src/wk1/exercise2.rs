/*  ### Exercise 2 - Code analysis
    - Without testing it, what is wrong with this code snippet?
    - How can it be fixed?
*/

// - Snippet 1
fn main() {
    //let a = vec![1, 2, 3, 4]; make mutable
    let mut a = vec![1, 2, 3, 4];
    a.push(27);
}

// - Snippet 2
//fn my_operation(a: u64, b: u64) -> u64 {
fn my_operation(mut a: u64, b: u64) -> u64 {
    a += b;
    a
}

fn main() {
    let num1 = 1234;
    let num2 = 1122;
    println!("My result is {}!", my_operation(num1, num2));
}

// Snippet 3
//     - Without testing it, what is wrong with this code snippet?
fn main() {
    let x = 1;

    {
        let mut x = 2;

        x = x ^ 2;

        {
            x = 3;
            let x = 12;
        }
        println!("x is: {}", x);
    }
}

/*
- Snippet 4

    The following Solidity and Rust snippets shows the (Key ⇒ Value) functionality. Solidity provides this through a mapping while Rust provides it through an Hashmap.

    - Snippets

        ```solidity
        // SPDX-License-Identifier: GPL-3.0

        pragma solidity ^0.8.0;

        contract TestMapping {

            mapping(string => uint256) values;

            function add(string calldata input, uint256 value) external {
                values[input] = value;
            }

            function read(string calldata input) external view returns (uint256) {
                return values[input];
            }
        }
        ```
    - Hint
        - https://docs.soliditylang.org/en/latest/control-structures.html#scoping-and-declarations
        - https://doc.rust-lang.org/book/ch08-03-hash-maps.html#accessing-values-in-a-hash-map

*/

use std::collections::HashMap;

fn main() {
    let mut values: HashMap<String, u64> = HashMap::new();

    values.insert(String::from("test"), 12345);
    // None
    println!(
        "\"test\" associated value is {}",
        values.get(&String::from("test")).unwrap()
    );
}
/*

        - https://doc.rust-lang.org/book/ch08-03-hash-maps.html#accessing-values-in-a-hash-map
Here, score will have the value that’s associated with the Blue team, and the result will be 10. The get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None.

        - https://docs.soliditylang.org/en/latest/control-structures.html#scoping-and-declarations
The “default values” of variables are the typical “zero-state” of whatever the type is.
*/
