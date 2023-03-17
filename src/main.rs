fn main() {
    let tabs = "4 -4 5 6 6 -6 6 5 4 -4 5 5 -4 4 -4
4 -4 5 6 6 -6 6 5 4 -4 5 5 -4 -4 4 

-5 -5 -6 -6 -6 6 5 4 -4 5 
4 -4 5 6 6 -6 6 5 4 -4 5 5 -4 -4 4";

    let (res, _errs) = transpose_tabs(tabs, -12, "richter");
    println!("{}", res);
    let semitones = get_playable_keys(tabs, "richter");
    dbg!(semitones);
}

pub fn transpose_tabs(
    tab: &str,
    semitones: i32,
    input_tuning: &str,
) -> (String, Vec<String>) {
    let (input_notes, duplicated_notes) = harptabber::tuning_to_notes_in_order(input_tuning);

    let _kalimba_notes = "C X D X E F X G X A X B C' X D' X E' F' X G' X A' X B' C'' X D'' X E''";
    let kalimba_numbers = "1 X 2 X 3 4 X 5 X 6 X 7 1' X 2' X 3' 4' X 5' X 6' X 7' 1'' X 2'' X 3''";
    let output_notes: Vec<String> = kalimba_numbers
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect();

    let mut result = String::from("");
    let mut errors: Vec<String> = Vec::new();

    for line in tab.lines() {
        for note in line.split_whitespace() {
            let note = harptabber::fix_enharmonics(note, &duplicated_notes);
            let new_note = harptabber::transpose(&input_notes, &output_notes, note, semitones);

            match new_note {
                Ok(new_note) => {
                    result.push_str(new_note);
                    result.push(' ');
                },
                Err(_) => {
                    errors.push(note.to_string());
                }
            }
        }
        result.push('\n');
    }
    (result, errors)
}

pub fn get_playable_keys(
    tab: &str,
    input_tuning: &str,
) -> Vec<(&'static str, i32)> {
    let mut results = Vec::new();
    if tab.is_empty() {
        return results;
    }

    let chromatic_notes = ["C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"];
    for semitones in -24..=24 {
        let (notes, _errors) = transpose_tabs(
            tab,
            semitones,
            input_tuning,
        );

        let index = semitones.rem_euclid(12) as usize;
        let key = chromatic_notes[index];

        if !notes.contains("X") {
            results.push((key, semitones));
        }
    }
    results
}
