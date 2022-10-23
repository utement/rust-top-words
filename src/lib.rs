pub fn get<T>(language: T) -> Vec<String>
where T: Into<String>, {
    match language.into().as_ref() {
        "english" => get_language(include_bytes!("resources/english")),
        &_ => todo!(),
    }
}

fn get_language(bytes: &[u8]) -> Vec<String> {
    let file = String::from_utf8_lossy(bytes);
    let lines = file.split('\n');
    let mut output = vec![];
    for line in lines {
        output.push(String::from(line));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let result = get("english");
        assert_eq!(result.len(), 100);
    }
}
