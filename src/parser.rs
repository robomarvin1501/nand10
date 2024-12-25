use crate::tokeniser::tokenise;

pub fn parse(input_data: String) {
    let tokens = tokenise(input_data);
    dbg!(tokens);
}
