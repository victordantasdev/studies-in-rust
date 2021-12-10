fn main() {
    //==> imutable variables <==
    // let name = "Victor";
    // let age = 21;

    // println!("Heloo, my name is {}, and my age is {}!", name, age);

    //==> mutable variables <==
    // let mut a = 10;
    // println!("The number is {}!", a);

    // a = 15;
    // println!("Now the number is {}!", a);

    //==> shadow variables <==
    let shadow_num = 5;
    println!("{}", shadow_num);
    let shadow_num = shadow_num + 5;
    println!("{}", shadow_num);
    let shadow_num = shadow_num * 2;

    println!("The number is {}!", shadow_num);
}
