fn main() {
    /*
    //expressions
    let num = 3;

    if num == 2 {
        println!("the number is defintely three!!");
    }else {
        println!("the number could be anything but three!!");
    }

    //in a let statement

    let val = 7;
    let is_even: bool = is_even_func(val);

    println!("{}", is_even);

    let arr = [1, 2, 3, 4, 50];

    let summation = sum_func_loop(arr);

    println!("{}", summation);

    let summation = sum_func_while(arr);

    println!("{}", summation);

    let summation = sum_func_for(arr);

    println!("{}", summation);

    iterable_func_for(arr);
    */

    let celsius_res = farenheit_to_celsius(182.0);
    println!("{}", celsius_res);

    let farenheit_res = celsius_to_farenheit(0.0);
    println!("{}", farenheit_res);

    //let nth_fib_num = fib_num(5);

    let arr = [-1; 9];

    let nth_fib_num = fib_num_arr(arr, 7 as usize);

    println!("{}", nth_fib_num);

    twelve_days_of_christmas_lyrics_print();
}


fn farenheit_to_celsius(frnh_temp: f32)->f32{
    (frnh_temp-32.0)*(5.0/9.0)
}

fn celsius_to_farenheit(cls_temp: f32)->f32{
    (cls_temp*(9.0/5.0))+32.0
}


/*fn fib_num(n: i32)->i32{
    if n == 1 || n == 2 {
        1
    }
    else if n>=3 {
        fib_num(n-1)+fib_num(n-2)
    }
    else{
        0
    }
}*/

fn fib_num_arr(mut dp: [i32; 9], n: usize)->i32{
    if dp[n]!=-1 {
        dp[n]
    }else if n == 1 || n == 2{
        dp[n] = 1;
        dp[n]
    }else if n>=3 {
        dp[n] = fib_num_arr(dp, n-1)+fib_num_arr(dp, n-2);
        dp[n]
    }else{
        0
    }
}

fn twelve_days_of_christmas_lyrics_print() {
    let mut arr = [ "12 drummers drumming", "Eleven pipers piping", "Ten lords a leaping",
                "Nine ladies dancing", "Eight maids a milking", "Seven swans a swimming",
                "Six geese a laying", "Five gold rings, badam-pam-pam", "Four calling birds",
                "Three French hens", "Two turtle doves", "A partridge in a pear tree"
              ];
    
    arr.reverse();

    let day = [ " first ", " second ", " third ",
                " fourth ", " fifth ", " sixth ",
                " seventh ", " eighth ", " ninth ",
                " tenth ", " eleventh ", " twelfth ",
              ];

    let first_verse_prefix = "On the";
    let first_verse_suffix = "day of Christmas";
    let second_verse = "My true love gave to me";
    for idx in 0..12 {
        println!("{}{}{}", first_verse_prefix, day[idx], first_verse_suffix);
        println!("{}", second_verse);
        for inner_idx in (0..idx+1).rev() {
            if inner_idx == 0 && idx != 0 {
                println!("And {}", arr[inner_idx]);
            }else{
                println!("{}", arr[inner_idx]);
            }
        }

        println!();
    }
}


/*
fn is_even_func(val: i32)->bool{
    if val%2==0 {
        true
    } else {
        false
    }
}


// write a function that loops 
// through numbers and returns
// sum

//using loop
fn sum_func_loop(arr: [i32; 5])->i32{
    let mut idx = 0;
    let mut sum = 0;
    loop{
        sum += arr[idx];
        idx += 1;

        if idx==5 {
            break;
        }
    }
    sum
}

//using while
fn sum_func_while(arr: [i32; 5])->i32{
    let mut idx = 0;
    let mut sum = 0;
    while idx!=5 {
        sum += arr[idx];
        idx +=  1;
    }
    sum
}

//using for
fn sum_func_for(arr: [i32; 5])->i32{
    let mut sum = 0;

    //for object loop
    for element in arr.iter() {
        sum += element;
    }
    sum
}

fn iterable_func_for(arr: [i32; 5]) {
    for idx in 0..5 {
        println!("idx: {}", idx);
        println!("{}", arr[idx]);
    }
}
*/