fn main() {
    println!("The Twelve Days of Christmas!\n");

    // twelve days of christmas
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    // list of gifts for each day
    let gifts: [&str; 12] = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    // loop through each day from 0 to 11 (representing the 1st to 12th day of christmas)
    for (day, _) in days.iter().enumerate() {
        println!(
            "On the {} day of Christmas,\nMy true love sent to me",
            days[day]
        );

        // inner loop to list the gifts in reverse order for the current day
        for gift in (0..=day).rev() {
            // for any day after the first, the last line is "and a partridge in a pear tree"
            if day > 0 && gift == 0 {
                println!("And {}.", gifts[gift]);
            }
            // for the first day, the last line is "a partridge in a pear tree"
            else if gift == 0 {
                println!("{}.", gifts[gift]);
            }
            // remaining lines before the last line end with a comma
            else {
                println!("{},", gifts[gift]);
            }
        }

        // next line after each day but the last day of christmas
        if day < 11 {
            println!("\n");
        }
    }
}
