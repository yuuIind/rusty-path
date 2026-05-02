const DAYS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
];
const VERSES: [&str; 12] = [
    "a Partridge in a Pear Tree",
    "two Turtle Doves",
    "three French Hens",
    "four Calling Birds",
    "five Gold Rings",
    "six Geese a-Laying",
    "seven Swans a-Swimming",
    "eight Maids a-Milking",
    "nine Ladies Dancing",
    "ten Lords a-Leaping",
    "eleven Pipers Piping",
    "twelve Drummers Drumming",
];

fn main() {
    println!("The Twelve Days of Christmas!");
    print_separator();

    for (day, day_name) in DAYS.iter().enumerate()  {
        println!("On the {} day of Christmas, my true love sent to me:", day_name);
        for gift in (0..=day).rev() {
            if gift == 0 && day > 0 {
                println!("and {}", VERSES[0]);
            } else if gift == 0 {
                println!("{}", VERSES[0]);
            } else {
                println!("{},", VERSES[gift]);
            }
        }
        print_separator();

    }
}


fn print_separator() {
    println!("\n===================================================\n");
}