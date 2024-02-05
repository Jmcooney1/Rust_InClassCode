#[derive(Debug,ParticalEq, Copy,Clone)]
enum IceCreamFlavor{
    Strawberry,
    Vanilla,
    Chocolate,
}
enum ScoopSize{
    one: Vec<IceCreamFlavor>,
    two: Vec <IceCreamFlavor,IceCreamFlavor>,
    three: Vec<IceCreamFlavor,IceCreamFlavor,IceCreamFlavor>,
}

//Need math to due gallon
//1 gallon = 

struct Inventory{
    Order: Vec<ScoopSize>,
}

fn main() {
    println!("Hello, world!");
}
