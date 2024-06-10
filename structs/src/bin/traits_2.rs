fn main(){

//    let mut amount= Bank{act:1, bal:3222.2};

let mut xyz =Bank::new(101, 204.33);
make_payment(&mut xyz,1000.32 );
make_payment1(&mut xyz, 10000.32);
}

fn make_payment(o:&mut impl Paymennt, amount:f32){
    o.make_payment(amount);
    let bal = o.check_balance();
    println!("{}", bal);
    
}

fn make_payment1<T:Paymennt>(o:&mut T, amount:f32){
    o.make_payment(amount);
    let bal = o.check_balance();
    println!("{}", bal);
    
}
// fn make_paymet(amount:f32, b:&mut Bank){
// b.payment(amount);
// let bal = b.checkbalance();
// println!("{}", bal); 
// }

// fn make_paymet_bank(amount:f32, b:&mut UpiBank){
//     b.upipayment(amount);
//     let bal = b.checkupbalance();
//     println!("{}", bal);
// } 
trait Paymennt{
    fn check_balance(&self) -> f32;
    fn make_payment(&mut self, amount:f32) -> f32;
}

struct Bank{
    act:i32,
    bal:f32,
}

impl Paymennt for Bank{
    fn check_balance(&self) -> f32{
        return self.bal;

    }
    fn make_payment(&mut self, amount:f32) -> f32{
        let b = self.checkbalance();
        if b > amount{
            self.bal = self.bal - amount;
        }
        self.bal
    }
}
impl Bank{
    fn checkbalance(&self) -> f32{
        return self.bal
    }
    fn payment(&mut self, amount:f32) -> f32{
        let b = self.checkbalance();
        if b > amount{
            self.bal = self.bal - amount;
        }
        self.bal
    }
    fn new(act:i32, bal:f32) -> Self{
        Bank{act:act, bal:bal}

    }
}

struct UpiBank{
    uid:i32,
    bal:f32,
}

impl UpiBank{
    fn checkupbalance(&self) -> f32{
        return self.bal
    }
    fn upipayment(&mut self, amount:f32) -> f32{
        let b = self.checkupbalance();
        if b > amount{
            self.bal = self.bal - amount;
        }
        self.bal
    }
}