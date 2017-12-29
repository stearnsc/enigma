extern crate rand;

use rand::Rng;

static CHARS: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                            'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

fn main() {
    let plugboard_pairs = vec![('A', 'R'), ('H', 'J'), ('N', 'X')];
    let plugboard = Plugboard::new(&plugboard_pairs);

    let rotors = vec![
        Rotor::new(['Q', 'F', 'Z', 'E', 'K', 'D', 'S', 'C', 'X', 'T', 'W', 'V', 'Y', 'R', 'G', 'A', 'M', 'B', 'N', 'P', 'J', 'L', 'O', 'I', 'U', 'H'], 3),
        Rotor::new(['D', 'T', 'B', 'F', 'H', 'Q', 'Z', 'R', 'J', 'U', 'V', 'Y', 'L', 'K', 'S', 'W', 'N', 'A', 'P', 'E', 'M', 'X', 'I', 'G', 'O', 'C'], 17),
        Rotor::new(['S', 'A', 'H', 'F', 'P', 'Y', 'T', 'Q', 'M', 'O', 'I', 'R', 'U', 'K', 'B', 'Z', 'C', 'D', 'J', 'V', 'W', 'X', 'E', 'G', 'N', 'L'], 5),
    ];

    let reflector_pairs = [('A', 'Z'), ('B', 'Y'), ('C', 'X'), ('D', 'W'), ('E', 'V'),
        ('F', 'U'), ('G', 'T'), ('H', 'S'), ('I', 'R'), ('J', 'Q'), ('K', 'P'), ('L', 'O'), ('M', 'N')];
    let reflector = Reflector::from_pairs(&reflector_pairs);

    let mut enigma = Enigma::new(plugboard, rotors, reflector);

    let plaintext = "hello world";
    let cyphertext = enigma.encypher_str(plaintext);


    enigma.reset_positions(vec![3, 17, 5].as_slice());

    let decyphered = enigma.encypher_str(&cyphertext);
    println!("Plaintext: {}", plaintext);
    println!("Cyphertext: {}", cyphertext);
    println!("Decyphered: {}", decyphered);
}

#[derive(Debug)]
struct Enigma {
    plugboard: Plugboard,
    rotors: Vec<Rotor>,
    reflector: Reflector
}

impl Enigma {
    pub fn new(plugboard: Plugboard, rotors: Vec<Rotor>, reflector: Reflector) -> Enigma {
        Enigma { plugboard, rotors, reflector }
    }

    pub fn reset_positions(&mut self, positions: &[usize]) {
        self.rotors.iter_mut().zip(positions).for_each(|(rotor, position)| {
            rotor.set_rotation(*position)
        });
    }

    pub fn encypher_str(&mut self, s: &str) -> String {
        s.to_uppercase().chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| self.encypher(c))
            .collect()
    }

    pub fn encypher(&mut self, c: char) -> char {
        println!("Input: {}", c);
        println!("A B C D E F G H I J K L M N O P Q R S T U V W X Y Z");
        print_arrow(get_char_digit(c), get_char_digit(c));

        self.plugboard.print();
        let plugboard_output = self.plugboard.run(c);
        print_arrow(get_char_digit(c), get_char_digit(plugboard_output));

        let mut rotor_num = 1;
        let end_char =  self.rotors.iter().fold(plugboard_output, |wire_char, ref rotor| {
            rotor.print(rotor_num);
            rotor_num += 1;

            let output = rotor.forward(wire_char);
            print_arrow(get_char_digit(wire_char), get_char_digit(output));
            output
        });

        rotor_num -= 1;
        let reflected_char = self.reflector.run(end_char);
        self.reflector.print();
        print_arrow(get_char_digit(end_char), get_char_digit(reflected_char));

        let rotor_output = self.rotors.iter().rev().fold(reflected_char, |wire_char, ref rotor| {
            rotor.print_inverse(rotor_num);
            rotor_num -= 1;

            let output = rotor.reverse(wire_char);
            print_arrow(get_char_digit(wire_char), get_char_digit(output));
            output
        });

        self.rotate_rotors();

        let plugboard_output = self.plugboard.run(rotor_output);
        self.plugboard.print();
        print_arrow(get_char_digit(rotor_output), get_char_digit(plugboard_output));
        println!("A B C D E F G H I J K L M N O P Q R S T U V W X Y Z");
        println!("\nInput: {}, Output: {}\n", c.to_uppercase(), plugboard_output);
        plugboard_output
    }

    pub fn rotate_rotors(&mut self) {
        for rotor in &mut self.rotors {
            let carry = rotor.rotate();
            if !carry {
                break
            }
        }
    }

}

#[derive(Debug)]
pub struct Plugboard {
    mapping: [char; 26],
}

impl Plugboard {
    pub fn new(pairs: &[(char, char)]) -> Plugboard {
        let mut mapping = ['0'; 26];
        mapping.copy_from_slice(&CHARS);
        for &(a, b) in pairs {
            mapping[get_char_digit(a)] = b;
            mapping[get_char_digit(b)] = a;
        }

        Plugboard { mapping }
    }

    pub fn new_rand(num_pairs: usize) -> Plugboard {
        assert!(num_pairs < 13);
        let mut chars = ['0'; 26];
        chars.copy_from_slice(&CHARS);
        rand::thread_rng().shuffle(&mut chars);

        let mut swappings = Vec::new();
        for i in 0..num_pairs {
            swappings.push((chars[i], chars[13 + i]));
        }

        Plugboard::new(&swappings)
    }

    pub fn run(&self, c: char) -> char {
        self.mapping[get_char_digit(c)]
    }

    pub fn print(&self) {
        let mut s = String::new();
        for c in self.mapping.iter() {
            s.push(*c);
            s.push(' ');
        }
        s.pop();
        println!("A B C D E F G H I J K L M N O P Q R S T U V W X Y Z");
        println!("| | | | | | | | | | | | | | | | | | | | | | | | | |");
        println!("{} (plugboard)", s);
    }
}

#[derive(Debug)]
struct Rotor {
    rotation: usize,
    mapping: [char; 26],
    inverted: [char; 26]
}

impl Rotor {
    pub fn new_rand() -> Rotor {
        let mut mapping: [char; 26] = ['0'; 26];
        mapping.copy_from_slice(&CHARS);

        rand::thread_rng().shuffle(&mut mapping);

        Rotor::new(mapping, 0)
    }

    pub fn new(mapping: [char; 26], rotation: usize) -> Rotor {
        let mut inverted = ['0'; 26];
        for i in 0..26 {
            inverted[get_char_digit(mapping[i])] = CHARS[i];
        }
        Rotor { rotation, mapping, inverted }
    }

    pub fn rotate(&mut self) -> bool {
        let mut overflow = false;
        self.rotation += 1;

        if self.rotation == 26 {
            overflow = true;
            self.rotation = 0;
        }

        let tmp = self.mapping[0];
        for i in 0..25 {
            self.mapping[i] = self.mapping[i + 1];
            // [2,3,4,t] [1]
        }
        self.mapping[25] = tmp;

        for i in 0..26 {
            self.inverted[get_char_digit(self.mapping[i])] = CHARS[i];
        }

        overflow
    }

    pub fn set_rotation(&mut self, rotation: usize) {
        let mut mapping = ['0'; 26];
        let delta = (26 + self.rotation - rotation) % 26;
        for i in 0..26 {
            mapping[i] = self.mapping[(i + 26 - delta) % 26];
        }
        self.mapping.copy_from_slice(&mapping);

        for i in 0..26 {
            self.inverted[get_char_digit(self.mapping[i])] = CHARS[i];
        }

        self.rotation = rotation;
    }


    pub fn forward(&self, c: char) -> char {
        self.mapping[get_char_digit(c)]
    }

    pub fn reverse(&self, c: char) -> char {
        self.inverted[get_char_digit(c)]
    }

    pub fn print(&self, rotor: usize) {
        let mut s = String::new();
        for i in 0..26 {
            s.push(self.mapping[i]);
            s.push(' ');
        }
        println!("A B C D E F G H I J K L M N O P Q R S T U V W X Y Z");
        println!("| | | | | | | | | | | | | | | | | | | | | | | | | |");
        println!("{} (rotor {}, position: {})", s, rotor, self.rotation);
    }

    pub fn print_inverse(&self, rotor: usize) {
        let mut s = String::new();
        for i in 0..26 {
            s.push(self.inverted[i]);
            s.push(' ');
        }
        println!("A B C D E F G H I J K L M N O P Q R S T U V W X Y Z");
        println!("| | | | | | | | | | | | | | | | | | | | | | | | | |");
        println!("{} (rotor {}, position: {})", s, rotor, self.rotation);
    }
}

#[derive(Debug)]
struct Reflector {
    mapping: [char; 26]
}

impl Reflector {
    fn new_rand() -> Reflector {
        let mut chars: [char; 26] = ['0'; 26];
        chars.copy_from_slice(&CHARS);
        rand::thread_rng().shuffle(&mut chars);

        let (a, b) = chars.split_at(13);
        let pairs: Vec<(char, char)> = a.iter().cloned().zip(b.iter().cloned()).collect();
        Reflector::from_pairs(&pairs)
    }

    fn from_pairs(pairs: &[(char, char)]) -> Reflector {
        if pairs.len() != 13 {
            panic!("Illegal number of pairs used to create reflector");
        }
        let mut mapping: [char; 26] = ['0'; 26];
        for (a, b) in pairs.iter().cloned() {
            mapping[get_char_digit(a)] = b;
            mapping[get_char_digit(b)] = a;
        }
        if mapping.iter().cloned().any(|c| c == '0') {
            panic!("Incomplete pairs used to create reflector");
        }
        Reflector { mapping }
    }

    pub fn run(&self, c: char) -> char {
        self.mapping[get_char_digit(c)]
    }

    pub fn print(&self) {
        let mut s = String::new();
        for i in 0..26 {
            s.push(self.mapping[i]);
            s.push(' ');
        }
        println!("A B C D E F G H I J K L M N O P Q R S T U V W X Y Z");
        println!("| | | | | | | | | | | | | | | | | | | | | | | | | |");
        println!("{} (reflector)", s);
    }
}

fn print_arrow(start: usize, end: usize) {
    let mut s = String::new();
    if start == end {
        let spacer = " ".repeat(2 * start);
        s.push_str(&format!("{}|\n{}|", spacer, spacer))
    } else if start < end {
        s.push_str(&format!("{}|{}\n{}|",
            " ".repeat(2 * start),
            "_".repeat(2 * (end - start) - 1),
            " ".repeat(2 * end)
        ));
    } else {
        s.push_str(&format!("{}{}|\n{}|",
                           " ".repeat(2 * end + 1),
                           "_".repeat(2 * (start - end) - 1),
                           " ".repeat(2 * end)
        ));
    }
    println!("{}", s);
}

/*
a b c d
   ___|
  |
d c a b

*/

//panics if not A-Z
fn get_char_digit(c: char) -> usize {
    let d = c.to_digit(36).unwrap() as usize;
    if d < 10 || d > 36 {
        panic!("illegal character passed to get_char_digit")
    }
    d - 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo() {

    }
}

/*
Rotor
    B C D E F G H I J K L M N O P Q R S T U V W X Y Z A
Inverse
    Z A B C D E F G H I J K L M N O P Q R S T U V W X Y

Rotor pos 1 (< 1)
    C D E F G H I J K L M N O P Q R S T U V W X Y Z A B

Inverse pos 1 (> 1)
    Y Z A B C D E F G H I J K L M N O P Q R S T U V W X

(i + 26 - position) % 26
*/