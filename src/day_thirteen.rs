use std::cmp::Ordering;
use crate::day_thirteen::EqualState::{HIGHER, LOWER, UNKNOWN};
use crate::day_thirteen::ListType::{LIST, NUMBER};

pub fn day_thirteen(riddle_input: &String) {
    let mut packet_number = 1;
    let mut packet_number_sum = 0;
    let mut packet_list: Vec<ListElement> = vec![];
    for packet_pair_str in riddle_input.trim().split("\n\n") {
        let packet_pair = packet_pair_str.split_once("\n").expect("Invalid Format");
        let packet_one = parse_packet(packet_pair.0);
        let packet_two = parse_packet(packet_pair.1);
        if packet_one <= packet_two {
            packet_number_sum += packet_number;
        }
        packet_list.push(packet_one);
        packet_list.push(packet_two);
        packet_number += 1;
    }
    let distress_one = ListElement {
        element_type: LIST,
        list: vec![ListElement {
            element_type: NUMBER,
            list: vec![],
            element: 2
        }],
        element: 0
    };
    let distress_two = ListElement {
        element_type: LIST,
        list: vec![ListElement {
            element_type: NUMBER,
            list: vec![],
            element: 6
        }],
        element: 0
    };
    packet_list.push(distress_one.clone());
    packet_list.push(distress_two.clone());
    packet_list.sort_by(|first, second| first.partial_cmp(second).unwrap());
    let mut decoder_key = 0;
    for i in 0..packet_list.len() {
        if packet_list[i] == distress_one {
            decoder_key = i + 1;
        } else if packet_list[i] == distress_two {
            decoder_key *= i + 1;
            break;
        }
    }
    println!("The Sum of correct packet indices is {packet_number_sum}");
    println!("The decoder key is {decoder_key}");
}

fn parse_packet(packet_str: &str) -> ListElement {
    let mut element_to_build = ListElement {
        element_type: LIST,
        list: vec![],
        element: 0,
    };
    let packet_str_inner;
    if packet_str.starts_with("[") {
        packet_str_inner = packet_str.chars().skip(1).take(packet_str.len() - 2).collect::<String>()
    } else {
        packet_str_inner = packet_str.to_string();
    }
    let mut element: String = "".to_string();
    let mut inner_packet = false;
    let mut bracket_count = 0;
    for packet_char in packet_str_inner.chars() {
        if packet_char == '[' {
            inner_packet = true;
            bracket_count += 1;
        } else if packet_char == ']' {
            bracket_count -= 1;
        }
        if bracket_count == 0 && packet_char == ',' {
            if inner_packet {
                element_to_build.list.push(parse_packet(element.as_str()));
            } else {
                element_to_build.list.push(ListElement {
                    element_type: NUMBER,
                    list: vec![],
                    element: element.parse().expect("Invalid Number"),
                });
            }
            element = "".to_string();
        } else {
            element.push_str(packet_char.to_string().as_str());
        }
    }
    if element.contains("[") {
        element_to_build.list.push(parse_packet(element.as_str()));
    } else if element.len() > 0 {
        element_to_build.list.push(ListElement {
            element_type: NUMBER,
            list: vec![],
            element: element.parse().expect("Invalid Number"),
        });
    }
    return element_to_build;
}

struct ListElement {
    element_type: ListType,
    list: Vec<ListElement>,
    element: i32,
}

impl PartialEq for ListElement {
    fn eq(&self, other: &Self) -> bool {
        let comp = compare(self, other);
        comp == UNKNOWN
    }
}

impl PartialOrd<Self> for ListElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let comp = compare(self, other);
        match comp {
            LOWER => Some(Ordering::Less),
            UNKNOWN => Some(Ordering::Equal),
            HIGHER => Some(Ordering::Greater)
        }
    }
}

fn compare(first: &ListElement, other: &ListElement) -> EqualState {
    return if first.element_type == other.element_type {
        if first.element_type == NUMBER {
            // Compare Number
            if first.element < other.element {
                LOWER
            } else if first.element > other.element {
                HIGHER
            } else {
                UNKNOWN
            }
        } else {
            // Compare Lists
            let mut first_iter = first.list.iter();
            let mut second_iter = other.list.iter();
            let mut first_elem = first_iter.next();
            let mut second_elem = second_iter.next();
            while first_elem.is_some() && second_elem.is_some() {
                let comp = compare(first_elem.unwrap(), second_elem.unwrap());
                if comp == LOWER  {
                    return LOWER;
                } else if comp == HIGHER {
                    return HIGHER
                }
                first_elem = first_iter.next();
                second_elem = second_iter.next();
            }
            if first_elem.is_some() && second_elem.is_none() {
                HIGHER
            } else if first_elem.is_none() && second_elem.is_some() {
                LOWER
            } else {
                UNKNOWN
            }
        }
    } else {
        // Box Number into List
        if first.element_type == NUMBER {
            if other.list.len() == 0 {
                HIGHER
            } else {
                compare(&ListElement {
                    element_type: LIST,
                    list: vec![first.clone()],
                    element: 0,
                }, other)
            }
        } else {
            if first.list.len() == 0 {
                LOWER
            } else {
                compare(first, &ListElement {
                    element_type: LIST,
                    list: vec![other.clone()],
                    element: 0,
                })
            }
        }
    }
}

#[derive(PartialEq)]
enum EqualState {
    LOWER,
    UNKNOWN,
    HIGHER
}

impl Clone for ListElement {
    fn clone(&self) -> Self {
        ListElement {
            element_type: self.element_type,
            list: self.list.clone(),
            element: self.element,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum ListType {
    LIST,
    NUMBER
}
