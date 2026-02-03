



fn is_even(n: i32) -> bool{
    n % 2 == 0
}

fn fizzbuzz(n:i32) {
    if n % 3 == 0 && n % 5 == 0 {
        print!("FizzBuzz!");
    } 
    else if n % 3 == 0 {
        print!("Fizz!");
    }
    else if n % 5 == 0 {
        print!("Buzz!");
    }
}

fn main() {
   
    let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for i in 0..10{
        print!("is {} even? {}. ", arr[i], is_even(arr[i]));
        fizzbuzz(arr[i]);
        println!("");
    }

    println!("");
    
    let mut total = 0;
    let mut w = 0;

    while w < arr.len() {
        total += arr[w];
        w += 1;
    }

    println!("Sum of all numbers: {}", total);

    let mut biggest = 0;

    for i in 0..10{
        if arr[i] > biggest {
             biggest = arr[i];
        }
    }
    println!("Largest number in the array: {}", biggest)
}
