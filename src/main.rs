fn main()
{
    let f: f64 = 100.0;
    println!("{:.2} Fahrenheit = {:.2} Celcius", f, fahrenheit_to_celcius(f));

    let n: u32 = 35;
    println!("The {}{} Fibonacci number is {}", n, correct_number_suffix(n), fibonacci(n));

    println!("");
    twelve_days_of_christmas();
}

fn fahrenheit_to_celcius(f: f64) -> f64
{
    (f - 32.0) * 5.0 / 9.0
}

fn fibonacci(n: u32) -> u32
{
    if n < 2
    {
        return n
    }
    
    fibonacci(n - 1) + fibonacci(n - 2)
}

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