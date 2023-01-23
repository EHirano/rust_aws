struct PizzaList {
    pizzas: Vec<Pizza>,
}

struct Pizza {
    name: String,
}

impl PizzaList {
    fn new() -> PizzaList {
        PizzaList { 
            pizzas: vec![] 
        }
    }
}

fn get_pizza_from_name(pizza_name: &'a str, pizza_list: &'a PizzaList) -> Option<&'a Pizza> {
    let mut iter = pizza_list.pizzas.iter();
    iter.find(|pizza| pizza.name == pizza_name)

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_pizza_list_test() {
        let all_pizza = PizzaList::new();
        assert_eq!(0, all_pizza.pizzas.len());
        let veggie = get_pizza_from_name("veggie", &all_pizza);
    }
}