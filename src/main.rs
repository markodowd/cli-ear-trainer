mod notes;

fn main() {
    let note_map = notes::create_note_map();

    loop {
        let random_note = notes::get_random_note(&note_map);
        if let Some(note_info) = note_map.get(random_note) {
            println!("Random note: {:?} => {:?}", random_note, note_info);
        }

        // Ask the user to guess the note that is in NoteInfo.answer or NoteInfo.alt_answer
        // If the user guesses correctly, print "Correct!"
        // If the user guesses incorrectly, print "Incorrect! The correct answer is: <answer>"
        // If the user types "exit", break the loop

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if let Some(note_info) = note_map.get(random_note) {
            // convert input to uppercase and compare it to the answer and alt_answer
            let input = input.to_uppercase();

            if input == note_info.answer.to_uppercase() {
                println!("Correct!");
            } else if let Some(alt_answer) = &note_info.alt_answer {
                if input == alt_answer.to_uppercase() {
                    println!("Correct!");
                } else {
                    println!("Incorrect! The correct answer is: {}", note_info.answer);
                }
            } else {
                println!("Incorrect! The correct answer is: {}", note_info.answer);
            }
        }

        println!();
    }
}
