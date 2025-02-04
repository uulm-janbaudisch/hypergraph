pub fn to_csv(data: (usize, Vec<usize>)) -> String {
    let (original, split) = data;

    let mut output = original.to_string();
    output.push(',');

    output.push_str(
        split
            .iter()
            .map(|part| part.to_string())
            .collect::<Vec<String>>()
            .join(";")
            .as_str(),
    );

    output
}
