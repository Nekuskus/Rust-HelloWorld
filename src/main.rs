use rand::distributions::{Distribution, Uniform};
use std::collections::HashSet;

#[macro_export]
macro_rules! scramble {
    ( $( $str:tt ),* ) => {
        {
            
            let mut shuffled_vec = vec![(' ', 0 as usize)].iter().cycle().take(12).map(|item| item.to_owned()).collect::<Vec<(char, usize)>>();
            $(
                let chars = $str.chars().collect::<Vec<char>>();
                let mut used = HashSet::new();
                let between = Uniform::from(0..chars.len());
                let mut rng = rand::thread_rng();
                for i in 0..chars.len() {
                    let mut num = between.sample(&mut rng);
                    if !used.contains(&num) {
                        used.insert(num);
                        shuffled_vec[num] = (chars[i], i)
                    } else {
                        while used.contains(&num) {
                            num = between.sample(&mut rng);
                        }
                        used.insert(num);
                        shuffled_vec[num] = (chars[i], i)
                    }
                }
            )*
            shuffled_vec
        }
    };
}
fn main() {
    
    // Let's get our hello world...
    let hello_world:Vec<(char, usize)> = scramble!("Hello World!");
    
    // Oh no, someone's scrambled it!
    println!("{}", hello_world.iter().map(|item| item.0).collect::<String>());

    // Need to distract the user while it's still not ready...
    println!("\nFixing, please wait...\n");

    // Let's reconstruct the string...
    let mut new_world = vec![' '].iter().cycle().take(12).map(|item| item.to_owned()).collect::<Vec<char>>();
    for i in 0..hello_world.len() {
        new_world[hello_world[i].1] = hello_world[i].0;
    }

    // Now, time to print.
    println!("{}", new_world.iter().collect::<String>());
}
