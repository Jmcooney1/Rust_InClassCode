#[derive(Debug,PartialEq, Copy,Clone)]

enum IceCreamFlavor{
    Strawberry,
    Vanilla,
    Chocolate,
}
struct Order{
    size: i32,
    iceCreamFlavorList:IceCreamFlavor,
}

fn Price(scoop:&Order.size,) -> i32{
   let mut num_price = 0;
   if scoop == 1{
    num_price = 3.75;
   }
   else if scoop == 2{
    num_price = 4.75;
   }
   else{
    num_price = 5.75;
   }
}
struct Inventory{
    gallons: Vec<IceCreamFlavor>,
}

impl Inventory{
 fn orderMade(&self,user_preference:Option<Order>,) -> Order {
    user_preference.unwrap_or_else(|| self.most_stocked())
 }
 fn most_stocked(&self) -> Order{
    let mut num_strawberry = 0;
    let mut num_vanilla = 0;
    let mut num_choclate = 0;

    for flavor in &self.gallons{
        match flavor{
            IceCreamFlavor::Strawberry => num_strawberry += 1,
            IceCreamFlavor::Chocolate => num_choclate += 1,
            IceCreamFlavor::Vanilla => num_vanilla += 1,
        }
    }
    if num_strawberry > num_vanilla{
        if num_strawberry > num_choclate{
            IceCreamFlavor::Strawberry
        }
        else{
            IceCreamFlavor::Chocolate
        }
    }
    else if num_choclate > num_vanilla{
        if num_choclate > num_strawberry{
            IceCreamFlavor:: Chocolate
        }
        else{
            IceCreamFlavor::Strawberry
        }
    }
    else{
        IceCreamFlavor::Vanilla
    }

 }
}

struct customOrder{
    cusOrder: Vec<(Order,price(Order.size))>,
}
fn main() {
    let icecreamStore = Inventory{
        gallons:vec![
            IceCreamFlavor::Chocolate,
            IceCreamFlavor::Strawberry,
            IceCreamFlavor:: Vanilla,
            IceCreamFlavor::Chocolate,
            IceCreamFlavor::Vanilla,
        ],
    };

  
        let customer1 = Some(size::2,icecreamFlavorList::Chocolate);
        let order1 = store.orderMade(customer1);
        println!("")
        
    



} 
