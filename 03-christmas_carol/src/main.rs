fn main() {
    // Prints the lytics of the song to the console using recursion
    println!("🎄 The Twelve Days of Christmas 🎄");

    // get lyrics for each day
    for day in 1..13 {
        // print "On the Nth day of Christmas..."
        start_day(day);

        // on day 1, print the lyrics without the "and" before the last line
        if day == 1 {
            println!("  a Partridge in a Pear Tree.");
            continue;
        }

        // loop over the lyrics for the previous days
        for day_of_christmas in (1..(day + 1)).rev() {
            gift(day_of_christmas);
        }
    }

    println!("\n🎄 Merry Christmas 🎄");
}

// Get the lyrics for the first line of each day
fn start_day(day: u32) {
    // turn ordinal number into string
    let day_string = match day {
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
    };

    println!(
        "\nOn the {} day of Christmas\nmy true love gave to me:",
        day_string
    );
}

// Get the lyrics for each gift
fn gift(day_of_christmas: u32) {
    let days_song_lyrics = match day_of_christmas {
        1 => "and a Partridge in a Pear Tree.",
        2 => "Two Turtle Doves,",
        3 => "Three French Hens,",
        4 => "Four Calling Birds,",
        5 => "Five Golden Rings,",
        6 => "Six Geese a Laying,",
        7 => "Seven Swans a Swimming,",
        8 => "Eight Maids a Milking,",
        9 => "Nine Ladies Dancing,",
        10 => "Ten Lords a Leaping,",
        11 => "Eleven Pipers Piping,",
        12 => "Twelve Drummers Drumming,",
        _ => "",
    };

    println!("    {}", days_song_lyrics);
}
