// An enumeration of soft drinks.
// It's a type with variants, and it may be one of them.
enum SoftDrink {
  Coffee,
  Tea
}

enum Currency {
  Euro,
  Bitcoin
}

// A structure, just like in C.
// A struct or an enum can be reused as field of another struct.
struct Cup {
  drink: SoftDrink,
  price: f32,
  currency: Currency
}

// The implementation of Cup.
impl Cup {
  // self represents an instantiated Cup
  fn get_type(&self) -> String {
    // We use match to compare the drink against a series of patterns.
    // That's what we call "Pattern matching".
    match self.drink {
      SoftDrink::Coffee => String::from("coffee"),
      SoftDrink::Tea => String::from("tea")
    }
  }

  fn get_price(&self) -> f32 {
    self.price
  }

  fn get_currency(&self) -> String {
    match self.currency {
      Currency::Euro => String::from("€"),
      Currency::Bitcoin => String::from("BTC")
    }
  }
}

fn main() {
  // Instantiate a Cup of coffee...
  let coffee_cup = Cup {
    drink: SoftDrink::Coffee,
    price: 5.99,
    currency: Currency::Euro
  };

  // .. and a Cup of tea
  let tea_cup = Cup {
    drink: SoftDrink::Tea,
    price: 0.001872,
    currency: Currency::Bitcoin
  };

  println!("My cup of {} costs {}{}.", coffee_cup.get_type(), coffee_cup.get_price(), coffee_cup.get_currency());
  println!("My cup of {} costs {} {}.", tea_cup.get_type(), tea_cup.get_price(), tea_cup.get_currency());
}
