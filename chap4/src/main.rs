fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    {
        let s1 = String::from("hello");
        println!("s1 : {s1}");

        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {s1} , s2 = {s2}");
    }

    let mut t = String::from("hello");
    {
        let t1 = &mut t;
        println!("t1 : {t1}");
    }
    let t2 = &mut t;
    println!("t2 : {t2}");
}
