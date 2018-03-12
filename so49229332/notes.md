

It really depends on what the type is for. If it is only intended to hold values which implement the trait, then yes, it should have the trait bound e.g.

trait Child {
    fn name(&self);
}

struct School<T: Child> {
    pupil: T,
}

impl<T: Child> School<T> {
    fn role_call(&self) -> bool {
        // check everyone is here
    }
}

In this example, only children are allowed in the school so we have the bound on the struct.

If the struct is intended to hold any value but you want to offer extra behaviour when the trait is implemented, then no, the bound shouldn't be on the struct e.g.

trait GoldCustomer {
    fn get_store_points(&self) -> i32;
}

struct Store<T> {
    customer: T,
}

impl<T: GoldCustomer> Store {
    fn choose_reward(customer: T) {
        // Do something with the store points
    }
}

In this example, not all customers are gold customers and it doesn't make sense to have the bound on the struct.
