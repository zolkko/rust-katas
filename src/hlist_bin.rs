use rust_katas::hlist::*;
use rust_katas::hlist;


fn accept<T>(_value: &T) where T: IsHList {
    println!("ACCEPT is called");
}


fn main() {
    let ex = hlist![12, 1.2, "12", ];
    println!("{ex:?}");
    accept(&ex);

    let lst: HList<_, _> = (1, 2, 3).into();
    accept(&lst);
}

