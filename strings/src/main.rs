#![allow(unused)]

fn main() {
    {
        //CREATE NEW STRING

        let mut s = String::new();

        let data = "initial contents";

        let s = data.to_string();

        // the method also works on a literal directly:
        let s = "initial contents".to_string();

        let s = String::from("initial contents");

        println!("{}", s);
    }
    {
        // UPDATE STRINGS

        let mut s = String::from("foo");
        s.push_str("bar");

        println!("{}", s);

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {s2}");

        let mut s = String::from("lo");
        s.push('l');

        println!("{}", s);
    }
    {
        // Concatenation with the + Operator or the format! Macro

        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");

        // fn add(self, s: &str) -> String {
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

        println!("{}", s3);
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("{}", s);

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");
        println!("{}", s);
    }
    {
        // Methods for Iterating Over Strings

        for c in "Зд".chars() {
            println!("{c}");
        }

        for b in "Зд".bytes() {
            println!("{b}");
        }
    }
}
