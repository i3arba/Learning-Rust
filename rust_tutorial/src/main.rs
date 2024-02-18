#![allow(unused)]

use core::num;
use std::io; //para baixar todos os arquivos publicos da biblioteca usamos: `use std::io::*;`
use rand::Rng; //para verificar a versão da biblioteca vamos à https://crates.io/ e pesquisamos por `rand`
use std::io::{Write, BufReader, BufRead, ErrorKind}; //Podemos importar multiplos arquivos, se precisarmos.
use std::fs::File;
use std::cmp::Ordering;

//Continuar: https://youtu.be/ygL_xcavzQ4?si=7g6-NK_xzFhTuKL0&t=2292

fn main() {
    let arr_1 = [1, 2, 3, 4];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());

    let mut loop_idx = 0;

    while loop_idx < arr_1.len() {
        println!("Arr : {}", loop_idx);
        loop_idx += 1;
    }
}

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