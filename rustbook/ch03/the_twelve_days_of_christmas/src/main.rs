fn main() {
    // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
    let gifts = [
        "a Partridge in a pear tree.",
        "Two turtle doves",
        "Three French Hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a laying,",
        "Seven swans a swimming,",
        "Eight maids a milking,",
        "Nine ladies dancing,",
        "Ten lords a leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    println!("Lyrics to the Christmas carol: “The Twelve Days of Christmas” ");
    println!();

    // Outer loop: one iteration per verse.
    // `days.len()` returns 12, so `day` runs from 0 to 11.
    for day in 0..days.len() {
        print_verse(day, &days, &gifts);
    }
}

fn print_verse(day: usize, days: &[&str], gifts: &[&str]) {
    println!("On the {} day of Christmas,", days[day]);
    println!("My true love sent to me:");

    // Inner loop: counts down from the current day to 0.
    // `..=` makes the range inclusive (includes `day` itself).
    // `.rev()` iterates backwards, printing newest gift first.
    for gift in (0..=day).rev() {
        if gift == 0 && day > 0 {
            println!("And {}", gifts[gift]);
        } else {
            println!("{}", gifts[gift]);
        }
    }

    println!();
}
