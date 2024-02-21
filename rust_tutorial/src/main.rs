#![allow(unused)]

use core::num;
use std::io; //para baixar todos os arquivos publicos da biblioteca usamos: `use std::io::*;`
use rand::Rng; //para verificar a versão da biblioteca vamos à https://crates.io/ e pesquisamos por `rand`
use std::io::{Write, BufReader, BufRead, ErrorKind}; //Podemos importar multiplos arquivos, se precisarmos.
use std::fs::File;
use std::cmp::Ordering;


// Stack: Stores values in a LIFO - Last In First Out ||
// Heap:

fn main() {
    
}


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