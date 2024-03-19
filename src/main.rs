use std::collections::HashMap;

enum OrgChartAction
{
    AddAction(String, String),
    ListAction(String),
    EmptyAction,
}

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

    //let english_string = String::from("This is a dumb sentence to convert to pig latin");
    //println!("The pig latin for \"{}\" is \"{}\"", english_string, pig_latin(&english_string));

    simulate_dept_employee_commands();
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
fn pig_latin(s: &String) -> String
{
    let mut result = String::new();
    let mut is_start = true;
    for word in s.split_whitespace()
    {
        if is_start
        {
            is_start = false;
        }
        else
        {
            result.push(' ');
        }

        let first = &word[0..1];
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let starts_with_vowel = vowels.iter().any(|v| first.starts_with(*v));

        if word.len() > 0
        {
            let end = &word[1..];
            if starts_with_vowel { result += format!("{first}{end}hay").as_str(); }
            else { result += format!("{end}{first}ay").as_str(); }
        }
        else
        {
            if starts_with_vowel { result += format!("{first}hay").as_str(); }
            else { result += format!("{first}ay").as_str(); }
        }
    }

    result.clone()
}

#[allow(dead_code)]
fn simulate_dept_employee_commands()
{
    let commands = vec!
    [
        "Add Sally to Engineering",
        "Add Amir to Sales",
        "Add Brian to Engineering",
        "List Engineering",
        "Add Mark to Engineering",
        "Add Kathryn to HR",
        "Add Rich to C-Suite",
        "List Sales",
        "List Engineering",
    ];

    let mut org_chart: HashMap<String, Vec<String>> = HashMap::new();

    for command in commands
    {
        execute_command(command, &mut org_chart);
    }
}

#[allow(dead_code)]
fn execute_command(command: &str, org_chart: &mut HashMap<String, Vec<String>>)
{
    let words: Vec<&str> = command.split_whitespace().collect();
    let action_verb = words[0];
    let mut action: OrgChartAction = OrgChartAction::EmptyAction;
    match action_verb
    {
        "Add" => action = OrgChartAction::AddAction(words[1].to_string(), words[3].to_string()),
        "List" => action = OrgChartAction::ListAction(words[1].to_string()),
        _ => (),
    }

    execute_action(&action, org_chart);
}

#[allow(dead_code)]
fn execute_action(action: &OrgChartAction, org_chart: &mut HashMap<String, Vec<String>>)
{
    match action
    {
        OrgChartAction::AddAction(name, dept) =>
        {
            println!("ACTION: Adding {name} to {dept}");
            if org_chart.contains_key(&dept.to_string())
            {
                org_chart.get_mut(&dept.to_string()).unwrap().push(name.to_string());
            }
            else
            {
                let names: Vec<String> = vec![name.to_string()];
                org_chart.insert(dept.clone(), names.clone());
            }
        }
        OrgChartAction::ListAction(dept) =>
        {
            println!("ACTION: Listing employees in dept {dept}");
            let names: Vec<String> = org_chart.get_mut(&dept.to_string()).unwrap().to_vec();
            for name in names
            {
                println!("-- {name}");
            }
        }
        _ =>
        {
            println!("Invalid action");
        }
    }
}