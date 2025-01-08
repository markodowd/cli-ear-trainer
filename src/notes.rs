use rand::Rng;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Note {
    A3,
    ASharp3,
    B3,
    C4,
    CSharp4,
    D4,
    DSharp4,
    E4,
    F4,
    FSharp4,
    G4,
    GSharp4,
    A4,
    ASharp4,
    B4,
    C5,
    CSharp5,
    D5,
    DSharp5,
    E5,
    F5,
    FSharp5,
    G5,
    GSharp5,
    A5,
}

#[derive(Debug)]
pub struct NoteInfo {
    pub frequency: f32,
    pub answer: String,
    pub alt_answer: Option<String>,
}

pub fn create_note_map() -> HashMap<Note, NoteInfo> {
    let notes = vec![
        (Note::A3, 220.00, "A", None),
        (Note::ASharp3, 233.08, "A", Some("Bb".to_string())),
        (Note::B3, 246.94, "B", None),
        (Note::C4, 261.63, "C", None),
        (Note::CSharp4, 277.18, "C#", Some("Db".to_string())),
        (Note::D4, 293.66, "D", None),
        (Note::DSharp4, 311.13, "D#", Some("Eb".to_string())),
        (Note::E4, 329.63, "E", None),
        (Note::F4, 349.23, "F", None),
        (Note::FSharp4, 369.99, "F#", Some("Gb".to_string())),
        (Note::G4, 392.00, "G", None),
        (Note::GSharp4, 415.3, "G#", Some("Ab".to_string())),
        (Note::A4, 440.00, "A", None),
        (Note::ASharp4, 466.16, "A#", Some("Bb".to_string())),
        (Note::B4, 493.88, "B", None),
        (Note::C5, 523.25, "C", None),
        (Note::CSharp5, 554.37, "C#", Some("Db".to_string())),
        (Note::D5, 587.33, "D", None),
        (Note::DSharp5, 622.25, "D#", Some("Eb".to_string())),
        (Note::E5, 659.25, "E", None),
        (Note::F5, 698.46, "F", None),
        (Note::FSharp5, 739.99, "F#", Some("Gb".to_string())),
        (Note::G5, 783.99, "G", None),
        (Note::GSharp5, 830.61, "G#", Some("Ab".to_string())),
        (Note::A5, 880.00, "A", None),
    ];

    let mut note_map = HashMap::new();

    for (note, frequency, answer, alt_answer) in notes {
        note_map.insert(
            note,
            NoteInfo {
                frequency,
                answer: answer.to_string(),
                alt_answer,
            },
        );
    }

    note_map
}

pub fn get_random_note<'a>(note_map: &'a HashMap<Note, NoteInfo>) -> &'a Note {
    let mut rng = rand::thread_rng();
    let keys: Vec<_> = note_map.keys().collect();
    let random_key = rng.gen_range(0..keys.len());
    keys[random_key]
}
