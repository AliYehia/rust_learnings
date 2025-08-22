use std::fs::File;

struct CreditCard {
    card_number: i32,
    cvv: i32,
}

struct PayPal {
    email: String,
}

struct BankTransfer {
    iban: i32,
    bank_name: String,
}

trait ProcessPayment {
    fn process(&self);
}

impl ProcessPayment for CreditCard {
    fn process(&self) {
        println!("Processing credit card number {}, with CVV: {}", self.card_number, self.cvv)
    }
}

impl ProcessPayment for PayPal {
    fn process(&self) {
        println!("Processing for PayPal with email: {}.", self.email)
    }
}


impl ProcessPayment for BankTransfer {
    fn process(&self) {
        println!("Processing a bank transfer with IBAN: {}, for Bank: {}", self.iban, self.bank_name)
    }
}



fn main() {
    let cc1 = CreditCard {
        card_number: 1234,
        cvv: 876,
    };

    let pp1 = PayPal {
        email: String::from("aliy@amazon.com"),
    };

    let bt1 = BankTransfer {
        iban: 564321,
        bank_name: String::from("Bank of Afghanistan"),
    };

    let payments: Vec<Box<dyn ProcessPayment>> = vec![
        Box::new(cc1),
        Box::new(pp1),
        Box::new(bt1),
        ];

    for payment in payments {
        payment.process();
    }
}
