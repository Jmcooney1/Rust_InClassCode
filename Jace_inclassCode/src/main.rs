#[derive(Debug,PartialEq, Copy,Clone)]

enum IceCreamFlavor{
    Strawberry,
    Vanilla,
    Chocolate,
}
struct Order{
    size: i32,
    ice_Cream_Flavor_List:IceCreamFlavor,
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
 fn order_Made(&self,user_preference:Option<Order>,) -> Order {
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
    cusOrder: Vec<(Order,Price(Order.size))>,
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

  
        let customer1 = Some(Order<size::2,ice_Cream_Flavor_List::Chocolate>);
        let order1 = icecreamStore.order_Made(customer1);
        println!("")
        
    



} 
