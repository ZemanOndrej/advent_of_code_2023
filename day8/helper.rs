use std::collections::HashMap;

pub fn parse_input(
    input: String,
) -> (Vec<String>, HashMap<String, (String, String)>) {
    let mut lines = input.lines();
    let instructions: Vec<String> = lines
        .next()
        .unwrap()
        .split("")
        .filter(|&x| x != "")
        .map(|val| val.to_string())
        .collect();

    lines.next().unwrap();

    let network = lines.fold(HashMap::new(), |mut acc, line| {
        let split: Vec<_> = line.split(" = ").collect();
        let key = split.get(0).unwrap().to_string();
        let paths = split.get(1).unwrap().replace("(", "").replace(")", "");
        let value: Vec<_> = paths.split(", ").map(|s| s.to_string()).collect();
        acc.insert(
            key,
            (value.get(0).unwrap().clone(), value.get(1).unwrap().clone()),
        );

        acc
    });
    (instructions, network)
}
