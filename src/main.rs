mod lexing;

fn main() {
    let code = "asadsa.b.c.d()!\"fsfd  \\r \\n  dsdssd\" \n false node \ntrue '\\b'";

    for token in lexing::tokenize(code) {
        println!("{:#?}", token);
        //println!("{:#?}",token.to_string());
    }
}
