mod lib;

fn main() {
    let mut cache = lib::LruCache::<i32, String>::new(3);

    cache.put(1, "one".to_string());
    cache.put(2, "two".to_string());
    cache.put(3, "three".to_string());

    println!("Value for key 1: {:?}", cache.get(&1)); // Some("one")
    println!("Value for key 2: {:?}", cache.get(&2)); // Some("two")
    println!("Value for key 3: {:?}", cache.get(&3)); // Some("three")

    cache.put(4, "four".to_string()); // Evicts key 2

    println!("Value for key 1: {:?}", cache.get(&1)); // Some("one")
    println!("Value for key 2: {:?}", cache.get(&2)); // None
    println!("Value for key 3: {:?}", cache.get(&3)); // Some("three")
    println!("Value for key 4: {:?}", cache.get(&4)); // Some("four")
}
