use std::collections::HashMap;

#[derive(Debug)]
pub struct Attributes<'a>(HashMap<&'static str, &'a str>);

impl<'a, T> From<T> for Attributes<'a>
where
    T: AsRef<[(&'static str, &'a str)]>,
{
    fn from(item: T) -> Self {
        Attributes(item.as_ref().into_iter().map(|&(k, v)| (k, v)).collect())
    }
}

fn main() {
    let fruit = "banana".to_string();

    // This works. As it is coerced into &str because of the first one.
    let attr: Attributes = [("fruit", "apple"), ("new_fruit", &fruit)].into(); 

    // Does not work as type is &String. Help! Make it work.
    // let another: Attributes = [("fruit", &fruit)].into(); 

    // Works
    let one_more: Attributes = [("fruit", fruit.as_ref())].into();

    println!("{:?}", attr);
    // println!("{:?}", another);
    println!("{:?}", one_more);
}

fn test() {
    let fruit = "banana".to_string();

    // Failing case
    // let fails: Attributes = [("fruit", &fruit)].into();

    // Ascribe the type works
    let ascribe_test: [(&str, &str); 1] = [("fruit", &fruit)];
    let ascribe_attr: Attributes = ascribe_test.into();
}

// My attempt but errors out with unconstrained type parameter.
//
impl<'a, T> From<T> for Attributes<'a> where
    T: AsRef<[(&'static str, &'a U)]>, // Want only &String and &str as U
    {
    fn from(item: T) -> Self {
        Attributes(item.as_ref().into_iter().map(|&(k, v)| (k, v.as_ref())).collect())
    }
}
