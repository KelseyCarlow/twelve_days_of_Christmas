fn main() {

    let mut counter: usize = 0;

    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eigth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming"
    ];

    while counter < days.len() {
        println!(
            "On the {} day of Christmas, my true love sent to me:",
            days[counter]
        );

        counter += 1;

        for gift in gifts[0..counter].into_iter().rev() {
            match gift.contains(&"Partridge") {
                true => {
                    if counter != 1 {
                        println!("and {}\n", gifts[0])
                    } else {
                        println!("{}\n", gifts[0])
                    }
                },
                _ => println!("{},", gift)
            }
        }
    }
}

//
// On the first day of Christmas
// my true love sent to me:
// A Partridge in a Pear Tree
//
// On the second day of Christmas
// my true love sent to me:
// Two Turtle Doves
// and a Partridge in a Pear Tree
//
// On the third day of Christmas
// my true love sent to me:
// Three French Hens
// Two Turtle Doves
// and a Partridge in a Pear Tree
//
// On the fourth day of Christmas
// my true love sent to me:
// Four Calling Birds
// Three French Hens
// Two Turtle Doves
// and a Partridge in a Pear Tree
//
// On the fifth day of Christmas
// my true love sent to me:
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves
// and a Partridge in a Pear Tree
//
// On the sixth day of Christmas
// my true love sent to me:
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves
// and a Partridge in a Pear Tree
//
// On the seventh day of Christmas
// my true love sent to me:
// Seven Swans a Swimming
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves
// and a Partridge in a Pear Tree
//
// On the eighth day of Christmas
// my true love sent to me:
// Eight Maids a Milking
// Seven Swans a Swimming
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves
// and a Partridge in a Pear Tree
//
// On the ninth day of Christmas
// my true love sent to me:
// Nine Ladies Dancing
// Eight Maids a Milking
// Seven Swans a Swimming
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves
// and a Partridge in a Pear Tree
//
// On the tenth day of Christmas
// my true love sent to me:
// Ten Lords a Leaping
// Nine Ladies Dancing
// Eight Maids a Milking
// Seven Swans a Swimming
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves
// and a Partridge in a Pear Tree
//
// On the eleventh day of Christmas
// my true love sent to me:
// Eleven Pipers Piping
// Ten Lords a Leaping
// Nine Ladies Dancing
// Eight Maids a Milking
// Seven Swans a Swimming
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves
// and a Partridge in a Pear Tree
//
// On the twelfth day of Christmas
// my true love sent to me:
// 12 Drummers Drumming
// Eleven Pipers Piping
// Ten Lords a Leaping
// Nine Ladies Dancing
// Eight Maids a Milking
// Seven Swans a Swimming
// Six Geese a Laying
// Five Golden Rings
// Four Calling Birds
// Three French Hens
// Two Turtle Doves
// and a Partridge in a Pear Tree
