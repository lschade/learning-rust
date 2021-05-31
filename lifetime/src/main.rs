fn main() {
    let a = 10;
    let c;
    {
        let b = 20;
        //Error: lifetime of c must not be longer than lifetime of b
        c = max(&a, &b);
        println!("{}", c);
    }

    println!("{}", a);

    let s2 = String::from("Hello");
    let owner;

    {
        let s = String::from("Hello");
        // owner = Owner { x: &s }; //s does not live long enough
        owner = Owner { x: &s2 };
    }

    println!("{}", owner.x);
}


fn max<'a>(a: &'a u32, b: &'a u32) -> &'a u32 {
    if a > b {
        a
    } else {
        b
    }
}

// the reference must live at least as long as owner
struct Owner<'a> {
    x: &'a String,
}