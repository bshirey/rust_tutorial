use std::collections::HashMap;

fn main()
{
    //let f: f64 = 100.0;
    //println!("{:.2} Fahrenheit = {:.2} Celcius", f, fahrenheit_to_celcius(f));

    //let n: u32 = 35;
    //println!("The {}{} Fibonacci number is {}", n, correct_number_suffix(n), fibonacci(n));

    //println!("");
    //twelve_days_of_christmas();

    //let list_of_ints = vec![52, 97, 65, 86, 46, 4, 34, 17, 81, 35, 48, 76, 83, 78, 24, 48, 3, 50, 41, 14, 92, 100, 80, 90, 72, 28, 30, 56, 84, 74, 13, 78, 47, 66, 69, 29, 25, 18, 45, 14, 31, 73, 83, 2, 64, 17, 94, 35, 27, 13, 52, 74, 4, 22, 39, 56, 79, 91, 58, 44, 81, 45, 72, 27, 9, 17, 57, 61, 97, 62, 27, 26, 84, 50, 43, 92, 12, 28, 49, 37, 88, 88, 9, 87, 50, 68, 10, 100, 27, 67, 88, 19, 22, 90, 42, 22, 21, 41, 51, 86];
    //println!("Median of the list of {} is {}", list_of_ints.len(), get_median_of_ints(&list_of_ints));

    //let most_common = get_most_common_int_in_list(&list_of_ints);
    //println!("Most frequent occurrence is {} ({} times)", most_common.0, most_common.1);

    let english_string = String::from("This is a dumb sentence to convert to pig latin");
    println!("The pig latin for \"{}\" is \"{}\"", english_string, pig_latin(&english_string));
}

#[allow(dead_code)]
fn fahrenheit_to_celcius(f: f64) -> f64
{
    (f - 32.0) * 5.0 / 9.0
}

#[allow(dead_code)]
fn fibonacci(n: u32) -> u32
{
    if n < 2
    {
        return n
    }
    
    fibonacci(n - 1) + fibonacci(n - 2)
}

#[allow(dead_code)]
fn correct_number_suffix(n: u32) -> &'static str
{
    match n {
        11 => "th",
        12 => "th",
        13 => "th",
        _ => match n % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    }
}

#[allow(dead_code)]
fn twelve_days_of_christmas()
{
    for day in 1..=12
    {
        println!("On the {} day of Christmas,", match day {
            1 => "first",
            2 => "second",
            3 => "third",
            4 => "fourth",
            5 => "fifth",
            6 => "sixth",
            7 => "seventh",
            8 => "eighth",
            9 => "ninth",
            10 => "tenth",
            11 => "eleventh",
            12 => "twelfth",
            _ => "",
        });
        println!("My true love gave to me");

        for lyric_day in (1..=day).rev()
        {
            let mut lyric = day_of_christmas_lyrics(lyric_day).to_string();
            lyric = lyric + if lyric_day > 1 { "," } else { "." };

            if lyric_day == 1 && day > 1
            {
                lyric = "And a".to_string() + lyric.split_at(1).1;
            }
            println!("{}", lyric);
        }

        println!("");
    }
}

#[allow(dead_code)]
fn day_of_christmas_lyrics(day: u32) -> &'static str
{
    if day < 1 || day > 12
    {
        return "";
    }

    return match day {
        1 => "A partridge in a pear tree",
        2 => "Two turtle doves",
        3 => "Three French hens",
        4 => "Four calling birds",
        5 => "Five golden rings",
        6 => "Six geese a-laying",
        7 => "Seven swans a-swimming",
        8 => "Eight maids a-milking",
        9 => "Nine ladies dancing",
        10 => "Ten lords a-leaping",
        11 => "Eleven pipers piping",
        12 => "Twelve drummers drumming",
        _ => "",
    };
}

#[allow(dead_code)]
fn get_median_of_ints(ints: &Vec<u32>) -> u32
{
    let mut sorted_ints: Vec<u32> = ints.clone();
    sorted_ints.sort();
    //dbg!("List of ints = {}", &sorted_ints);

    let middle_index = sorted_ints.len() / 2;

    sorted_ints[middle_index]
}

#[allow(dead_code)]
fn get_most_common_int_in_list(ints: &Vec<u32>) -> (u32, u32)
{
    let mut count_map= HashMap::new();
    for int in ints
    {
        let count = count_map.entry(int).or_insert(0);
        *count += 1;
    }

    let mut winner = (0, 0);
    for (key, value) in count_map
    {
        if value > winner.1
        {
            winner = (*key, value);
        }
    }

    winner
}

#[allow(dead_code)]
fn pig_latin(s: &String) -> &str
{
    for word in s.split_whitespace()
    {

    }

    ""
}