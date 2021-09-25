fn main() {
    let consonants = vec![
        'b', 'c', 'd', 'f', 'g', 'j', 'k', 'l',
        'm', 'n', 'p', 'q', 's', 't', 'v', 'x',
        'z', 'h', 'r', 'w', 'y',
    ];
    let vowels = vec![
        'a', 'e', 'i', 'o', 'u', 'y'
    ];

    let input = String::from("зpinitial");
    let input: Vec<char> =
        input.chars().collect();

    let mut output = String::from("");
    if consonants.contains(&input[0]) {
        for (i, c) in input.iter().enumerate() {
            if i > 0 {
                output = output + &c.to_string();
            }
        }
        output += "-";
        output += &(input[0].to_string() + "ay");
    } else if vowels.contains(&input[0]) {
        for (_i, c) in input.iter().enumerate() {
            output += &c.to_string();
        }
        output += "-hay";
    } else {
        panic!("First character should be latin");
    }

    println!("Output: {}", output);
}
