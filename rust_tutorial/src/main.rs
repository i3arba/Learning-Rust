//First Steps
// cargo new ~Project_name~
// dir
// cd ~Project_name~

#![allow(unused)]

use core::num;
use std::io;
use std::time::Duration; //para baixar todos os arquivos publicos da biblioteca usamos: `use std::io::*;`
use rand::Rng; //para verificar a versão da biblioteca vamos à https://crates.io/ e pesquisamos por `rand`
use std::io::{Write, BufReader, BufRead, ErrorKind}; //Podemos importar multiplos arquivos, se precisarmos.
use std::fs::File;
use std::cmp::Ordering;

//36
//Concurrency & Thread
//Real world example
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
fn main() {
    pub struct Bank {
        balance: f32
    }

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32){
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00{
            println!("Current Balance : {} Withdrawal a smaller amount", bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Customer withdrew {} Current Balance {}", amt, bank_ref.balance);
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>){
        withdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank {balance: 20.00}));

    let handles = (0..10).map(|_|{
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref)
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total {}", bank.lock().unwrap().balance);


    //This won't work because we need a smart pointer to access the data.
    // fn withdraw(the_bank: &mut Bank, amt: f32){
    //     the_bank.balance -= amt;
    // }

    // let mut bank = Bank{balance: 100.00};
    // withdraw(&mut bank, 5.00);
    // println!("Balance : {}", bank.balance);

    // fn customer(the_bank: &mut Bank){
    //     withdraw(the_bank, 5.00);
    // }

    // thread::spawn(|| {
    //     customer(&mut bank)
    // }).join().unwrap(); 

}



//35
// use std::thread;
// use std::time; //In the video he imports like `use std::time:Duration;`, however, something change recently and now I just need to do `use std::time;`
// fn main(){
    // // Commom problems with parallel programming involve
    //     //1. Thread are accessing data in the wrong order.
    //     //2. Threads are blocked from executing because of confusion
    //     // over requirements to proceed with execution
    // let thread1 = thread::spawn(|| {
    //     for i in 1..25 {//This prints only 21 times instead of 24 as declared. 
    //         println!("Spawn thread : {}", i);// In this form there is no garantie that the thread will be completed, as mentioned above.
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..20{
    //     println!("Main thread : {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // thread1.join().unwrap(); //This will garantie the fully execution
// }


//35
//Smart pointers & Box
// fn main(){    
    //& reference operator | Box<>>
    //Box is normlally used when you have a large amount of data storage in the heap and then to pass pointers to it in the stack

    // =============================================
    
    // struct TreeNode<T> {
    //     pub left: Option<Box<TreeNode<T>>>,
    //     pub right: Option<Box<TreeNode<T>>>,
    //     pub key: T,
    // }
    
    // impl<T> TreeNode<T> {
    //     pub fn new(key: T) -> Self{
    //         TreeNode {
    //             left: None, right: None, key,
    //         }
    //     }
    //     pub fn left(mut self, node: TreeNode<T>) -> Self{
    //         self.left = Some(Box::new(node));
    //         self
    //     }
    //     pub fn right(mut self, node: TreeNode<T>) -> Self{
    //         self.right = Some(Box::new(node));
    //         self
    //     }
    // }

    // let node1 = TreeNode::new(1).left(TreeNode::new(2)).right(TreeNode::new(3));

    // =============================================

    // let b_int1 = Box::new(10);
    // println!("b_int1 = {}", b_int1);
    
// }



//34
//Closures
// fn main(){
//     //let var_name = |parameters| -> return_type {Body}
//     //Closures can access variables outside of it's scope.

//     fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
//         func(a,b)
//     }

//     let sum = |a, b| a+b;
//     let prod = |a, b| a*b;
//     println!("5 + 4 = {}", use_func(5,4,sum));
//     println!("5 * 4 = {}", use_func(5, 4, prod));

// //=============================================

//     // let mut samp1 = 5;
//     // let print_var = || println!("Samp1 = {}", samp1);

//     // print_var();

//     // samp1 = 10;
//     // let mut change_var = || samp1 += 1;
//     // change_var();
//     // println!("Samp1 = {}", samp1);

//     // samp1 = 10;
//     // println!("Samp1 = {}", samp1);

// //=============================================

//     // let can_vote = |age: i32| {
//     //     age >= 18
//     // };

//     // println!("Can vote : {}", can_vote(8));
// }


//33
//Iterators
// fn main(){
//     //This will borrow values. You cannot make any changes to this values.
//     let mut arr_it = [1,2,3,4];
//     for val in arr_it.iter(){
//         println!("{}", val);
//     }

//     let mut iter1 = arr_it.iter();
//     println!("1st : {:?}", iter1.next());
// }


//32
// fn main() {
//     //Result has 2 variants Ok and Err
//         //enum Result<T, E> {
//         //Ok(T),
//         //Err(E),}
//         //Where T represents the data typeoff the value returns and E the type of error
//     let path = "lines.txt";
//     let output = File::create(path);
//     let mut output = match output {
//         Ok(file) => file,
//         Err(error) =>{
//             panic!("Problem creating file : {:?}", error);
//         }
//     };

//     write!(output, "Just some\nRandom words").expect("Failed to write to file");

//     let input = File::open(path).unwrap();
//     let buffered = BufReader::new(input);

//     for line in buffered.lines(){
//         println!("{}", line.unwrap());
//     }

//     let output2 = File::create("rand.txt");
//     let output2 = match output2 {
//         Ok(file) => file,
//         Err(error) => match error.kind(){
//             ErrorKind::NotFound => match File::create("rand.txt"){
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Can't create file: {:?}", error),
//             },
//             _other_error => panic!("Problem opening file: {:?}", error),
//         }
//     };

//     //Forcing an error
//     // let lil_arr = [1,2];
//     // println!("{}", lil_arr[10]);
// }


//31
//Using restaurant/mod to 'pack' all of its functionalities
// mod restaurant;
// use crate::restaurant::order_food;
// fn main() {
//     //Crates: Modules that produce a library or executable
//     //Modules: Organize and handle privacy
//     //Packages: Build, test and share crates
//     //Paths: A way of naming an item such as a struct, function

//     order_food();
// }

//30
// fn main(){
//     const PI: f32 = 3.141592;

//     trait Shape{
//         fn new(length: f32, width: f32) -> Self;
//         fn area(&self) -> f32;
//     }

//     struct Rectangle{length: f32, width: f32};
//     struct Circle{length: f32, width: f32};

//     impl Shape for Rectangle{
//         fn new(length: f32, width: f32) -> Rectangle{
//             return Rectangle{length, width};
//         }
//         fn area(&self) -> f32{
//             return self.length * self.width;
//         }
//     }

//     impl Shape for Circle{
//         fn new(length: f32, width: f32) -> Circle{
//             return Circle{length, width};
//         }
//         fn area(&self) -> f32{
//             return (self.length / 2.0).powf(2.0) * PI;
//         }
//     }

//     let rec: Rectangle = Shape::new(10.0, 10.0);
//     let circ: Circle = Shape::new(10.0, 10.0);

//     println!("Rectangle Area : {}", rec.area());
//     println!("Circle Area : {}", circ.area());

//     // struct Rectangle<T, U> {
//     //     length: T,
//     //     height: U,
//     // }

//     // let rec = Rectangle {
//     //     length: 4,
//     //     height: 10.5
//     // };

//     // struct Customer {
//     //     name: String,
//     //     address: String,
//     //     balance: f32,
//     // }

//     // let mut bob = Customer{
//     //     name: String::from("Bob Singer"),
//     //     address: String::from("555 Main St"),
//     //     balance: 234.50
//     // };

//     // bob.address = String::from("505 Main St");
// }


//29
//HashMaps it's kind of mapping in solidity. But you can iterate over.
// use std::collections::HashMap;
// fn main(){
//     let mut heroes: HashMap<&str, &str> = HashMap::new();
//     heroes.insert("Superman", "Clark Kent");
//     heroes.insert("Batman", "Bruce Wayne");
//     heroes.insert("Flash", "Barry Allen");

//     for(k, v) in heroes.iter(){
//         println!("{} = {}", k, v);
//     }

//     println!("Length : {}", heroes.len());

//     if heroes.contains_key(&"Batman"){
//         let the_batman = heroes.get(&"Batman");
//         match the_batman {
//             Some(x) => println!("Batman is a hero"),
//             None => println!("Batman is not a hero"),
//         }
//     }
// }


//28
// fn print_str(x: String){
//     println!("A string {}", x);
// }

// fn print_return_str(x: String) -> String {
//     println!("A string {}", x);
//     x
// }

// fn change_string(name: &mut String){
//     name.push_str(" is happy");
//     println!("Message : {}", name);
// }

// fn main() {
//     // let str1 = String::from("World");
//     // let str2 = str1.clone();
//     // // print_str(str1);
//     // let str3 = print_return_str(str1);
//     // println!("str3 = {}", str3);

//     let mut str1 = String::from("Barba");
//     change_string(&mut str1);
// }

//27
// Stack: Stores values in a LIFO - Last In First Out || Data on the stack must have a defined fixed size
// Heap: When putting data on the heap you request a certaing amount of space.
// The OS finds space available and returns an address for that space called a pointer

// RULES
//1. Each value has a variable that's called its owner
//2. there is only one owner at a time
//3. When the owner goes out of scope the value disappears

// fn main() {
    // let str1 = String::from("World");
    // let str2 = str1;
    // println!("Hello {}", str1); This will give an error because str1 doesn't exist anymore. str1 was absorved by str2.

    // let str1 = String::from("World");
    // let str2 = str1.clone(); //This only applies for Strings, Tupples, Arrays and Vectors
    // println!("Hello {}", str1); //Now it's working because we copied str1 into str2. We don't "absorved" it.
// }


//26
//Perform operation with generic type
// use std::ops::Add;

// fn get_sum_gen<T:Add<Output = T>> (x: T, y: T) -> T {
//     return x + y;
// }

// fn main() {
//     println!("5 + 4 = {}", get_sum_gen(5,4));
//     println!("5.2 + 4.6 = {}", get_sum_gen(5.2,4.6));
// }

//25
// fn sum_list(list: &[i32]) -> i32 {
//     let mut sum = 0;
//     for &val in list.iter(){
//         sum += &val;
//     }
//     sum
// }

// fn main() {
//     let num_list = vec![1, 2, 3, 4, 5];
//     println!("Sum of list = {}", sum_list(&num_list));
// }

//24
// fn get_2(x: i32) -> (i32, i32) {
//     return (x+1, x+2);
// }

// fn main() {
//     let (vale_1, val_2) = get_2(3);
//     println!("Nums: {} {}", vale_1, val_2);
// }

//23
// fn get_sum_3(x: i32, y: i32) -> i32 {
//     return x + y;
// }

// fn main() {
//     println!("{}", get_sum_3(5,4));
// }

//22
// fn get_sum_2(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn main() {
//     println!("{}", get_sum_2(5,4));
// }


//21
// fn get_sum(x: i32, y: i32){
//     println!("{} + {} = {}", x , y, x + y);
// }

// fn main() {
//     get_sum(5,4);
// }

//20
// fn say_hello(){
//     println!("Hello");
// }

// fn main() {
//     say_hello();
// }

//19
// fn main() {
//     let vec1: Vec<i32> = Vec::new();
//     let mut vec2 = vec![1,2,3,4];
//     vec2.push(5);

//     println!("1st :{}", vec2[0]);

//     let second: &i32 = &vec2[1];
    
//     match vec2.get(1){
//         Some(second) => println!("2nd : {}", second),
//         None => println!("Mo 2nd value"),
//     }

//     for i in &mut vec2{
//         *i *= 2;
//     }

//     for i in &vec2 {
//         println!("{}", i);
//     }

//     println!("Vec Length {}", vec2.len());
//     println!("Pop : {:?}", vec2.pop());
// }

//18
// fn main() {
//     enum Day {
//         Monday,
//         Tuesday,
//         Wednesday,
//         Thursday,
//         Friday,
//         Saturday,
//         Sunday
//     }

//     impl Day {
//         fn is_weekend(&self) -> bool {
//             match self {
//                 Day::Saturday | Day::Sunday => true,
//                 _ => false
//             }
//         }
//     }
//     let today:Day = Day::Monday;
//     match today {
//         Day:: Monday => println!("Everyone hates Mondays"),
//         Day:: Tuesday => println!("Donut Day"),
//         Day:: Wednesday => println!("Barbecue day"),
//         Day:: Thursday => println!("Burger day"),
//         Day:: Friday => println!("Pizza day"),
//         Day:: Saturday => println!("Coxinha day"),
//         Day:: Sunday => println!("Pasta day"),
//     }

//     println!("Is today the weekend {}", today.is_weekend());
// }

//17
// fn main() {
//     let int_u8 = 5;
//     let int2_u8 = 4;
//     let int3_u32 = (int_u8 as u32) + (int2_u8 as u32);
// }

//16
// fn main() {
//     let st3 = String::from("x r t b h k k a m c");
//     let mut v1: Vec<char> = st3.chars().collect();
//     v1.sort();
//     v1.dedup(); 
//     for char in v1{
//         println!("{}", char);
//     }
    
//     let st4: &str = "Random string";
//     let mut st5: String = st4.to_string();
//     println!("{}", st5);

//     let byte_arr1 = st5.as_bytes();
//     let st6 = &st5[0..6];
//     println!("String length :{}", st6.len());

//     st5.clear();

//     let st6 = String::from("Just some");
//     let st7 = String::from("words");
//     let st8 = st6 + &st7;

//     for char in st8.bytes() {
//         println!("{}", char);
//     }
// }


//15
// fn main() {
//     let mut st1 = String::new();

//     st1.push('A');
//     st1.push_str(" word");

//     for word in st1.split_whitespace(){
//         println!("{}", word);
//     }

//     let st2 = st1.replace("A", "Another");
//     println!("{}", st2);
// }

//14
// fn main() {
//     let my_tuple: (u8, String, f64) = (28, "Barba".to_string(), 50_000.00);

//     println!("Name: {}", my_tuple.1);
//     let(v1, v2, v3) = my_tuple;
//     println!("Age : {}", v1);
// }

//13
// fn main() {
//     let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
//     let mut loop_idx = 0;

//     for val in arr_1.iter() {
//         println!("Val: {}", val);
//     }
// }

//12
// fn main() {
//     let arr_1 = [1, 2, 3, 4];
//     println!("1st : {}", arr_1[0]);
//     println!("Length : {}", arr_1.len());

//     let mut loop_idx = 0;

//     while loop_idx < arr_1.len() {
//         println!("Arr : {}", arr_1[loop_idx]);
//         loop_idx += 1;
//     }
// }


//11
// fn main() {
//     let arr_1 = [1, 2, 3, 4];
//     println!("1st : {}", arr_1[0]);
//     println!("Length : {}", arr_1.len());

//     let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
//     let mut loop_idx = 0;
//     loop {
//         if arr_2[loop_idx] % 2 == 0 {
//             loop_idx += 1;
//             continue;
//         }
//         if arr_2[loop_idx] == 9 {
//             break;
//         }
//         println!("Val : {}", arr_2[loop_idx]);
//         loop_idx += 1;
//     }
// }

//10
// fn main() {
//     let my_age = 18;
//     let voting_age = 18;
//     match my_age.cmp(&voting_age){
//         Ordering::Less => println!("Can't Vote"),
//         Ordering::Greater => println!("Can Vote"),
//         Ordering::Equal => println!("You gained the right to vote"),
//     }
// }

//9
// fn main() {
//     let age2 = 8;
//     match age2 {
//         1..=18 => println!("important Birthday"),
//         21 | 50 => println!("important Birthday"),
//         65..=i32::MAX => println!("important Birthday"),
//         _ => println!("Not important Birthday"),
//     }
// }

//8
// fn main() {
//     let mut my_age = 47;
//     let can_vote = if my_age >= 18 {
//         true
//     } else {
//         false
//     };
//     println!("Can Vote : {}", can_vote);
// }

//7
// fn main() {
//     let age : i32 = 8;
//     if( age >= 1) && (age <= 18){
//         println!("important Birthday");
//     } else if (age == 21) || (age == 50) {
//         println!("important Birthday");
//     } else if age >= 65 {
//         println!("important Birthday");
//     } else {
//         println!("Not important Birthday");
//     }
// }

//6
// fn main() {
//     let random_num = rand::thread_rng().gen_range(1..=100);
//     println!("Random : {}", random_num);
// }

//5
// fn main() {
//     let num_1: f32 = 1.111111111111111;
//     println!("{}", num_1 + 0.111111111111111);
//     let num_2: f64 = 1.111111111111111;
//     println!("{}", num_2 + 0.111111111111111);
//     let num_3: u32 = 5;
//     let num_4: u32 = 4;
//     println!("5+4 = {}", num_3 + num_4);
//     println!("5-4 = {}", num_3 - num_4);
//     println!("5*4 = {}", num_3 * num_4);
//     println!("5/4 = {}", num_3 / num_4);
//     println!("5%4 = {}", num_3 % num_4);
//     let mut num_5: u32 = 3;
//     num_5 += 1;
//     println!("3+1 = {}", num_5);
// }

//4
// fn main() {
//     let is_true: bool = true;
//     let my_grade = 'A';
// }

//3
// fn main() {
//     println!("Max u32 : {}", u32::MAX);
//     println!("Max u64 : {}", u64::MAX);
//     println!("Max usize : {}", usize::MAX);
//     println!("Max u128 : {}", u128::MAX);
//     println!("Max f32 : {}", f32::MAX);
//     println!("Max f64 : {}", f64::MAX);
// }

//2
// fn main() {
//     const ONE_MIL: u32 = 1_000_000;
//     const PI: f64 = 3.14159;
//     let age: &str = "28";
//     let mut age: u32 = age.trim().parse()
//         .expect("Age wasn't assigned a number");
//     age = age + 1;

//     println!("I'm {} and I want ${}", age, ONE_MIL);
// }

//1
// fn main() {
//     println!("What is you name?");
//     let mut name: String = String::new();
//     let greeting: &str = "Nice to meet you";
//     io::stdin().read_line(&mut name)
//         .expect("Didn't Receive Input");
//     println!("Hello {}! {}", name.trim_end(), greeting);
// }