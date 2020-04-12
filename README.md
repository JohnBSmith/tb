
# A small toolbox

## Examples 

### Random number generator

```rust
use tb::rng::{Rng,Rand};

fn main() {
    let seed = 0;
    let mut rng = Rng::new(seed);

    // Shuffle an array.
    let mut a: Vec<u32> = (0..10).collect();
    rng.shuffle(&mut a);
    println!("{:?}",a);

    // Throw a dice ten times.
    let a: Vec<u32> = (0..10).map(|_| rng.rand_range_u32(1,6)).collect();
    println!("{:?}",a);
}
```

### Additional literals

```rust
use tb::{array_from,vec_from,list,list_from};
use std::collections::LinkedList;

fn main() {
    let a: [String; 4] = array_from!["a","b","c","d"];
    println!("{:?}",a);

    let a: Vec<String> = vec_from!["a","b","c","d"];
    println!("{:?}",a);

    let a: LinkedList<String> = list_from!["a","b","c","d"];
    println!("{:?}",a);
    
    let a: LinkedList<i32> = list![1,2,3,4];
    println!("{:?}",a);
}
```
