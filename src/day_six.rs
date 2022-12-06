use std::collections::{HashMap, VecDeque};

pub fn day_six(riddle_input: &String) {
    search_section(riddle_input, 4);
    search_section(riddle_input, 14);
}

fn search_section(text: &String, section_size: usize) {
    let mut potential_packet_start: VecDeque<char> = VecDeque::new();
    let mut char_map: HashMap<char, i32> = HashMap::new();
    let mut char_count = 0;
    for signal_elem in text.chars() {
        char_count += 1;
        if char_map.contains_key(&signal_elem) {
            char_map.insert(signal_elem, &char_map[&signal_elem] + 1);
        } else {
            char_map.insert(signal_elem, 1);
        }
        potential_packet_start.push_front(signal_elem);
        if potential_packet_start.len() == section_size {
            let mut packet_started = true;
            for (_, num) in &char_map {
                if *num > 1 {
                    packet_started = false;
                    break;
                }
            }
            if packet_started {
                println!("Char count: {}", char_count);
                break;
            }
            let last_char = potential_packet_start.pop_back().expect("queue empty");
            char_map.insert(last_char, &char_map[&last_char] - 1);
        }
    }
}
