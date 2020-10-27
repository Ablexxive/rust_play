use std::env;
use std::process;

mod nolib;
use nolib::Config;


fn main() {
   let args: Vec<String> = env::args().collect();

   let config = Config::new(&args).unwrap_or_else(|err| {
       println!("Problem parsing arguments: {}", err);
       process::exit(1);
   });

   if let Err(e) = nolib::run(config)  {
        println!("Application error: {}", e);
        process::exit(1);
   }
}
//Learning tests before continuing, look at adder
//file:///Users/ablexxive/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch12-04-testing-the-librarys-functionality.html

// Just for other testing fun
fn tmp() {
    let str1 = String::from("abcd");
    let result;
    {
        let str2 = "xyz";
        //let str2 = String::from("xyz");

        result = longest(str1.as_str(), str2);
        //result = longest(str1.as_str(), str2.as_str());
    }
    println!("The longest string is: {}", result);

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
