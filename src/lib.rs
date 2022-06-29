// create a fair (non-weighted) die
use rand::Rng;

#[derive(Debug)]
struct Die {
    sides: u32,
}

impl Die {
    pub fn new(n_sides: u32) -> Die{
        if n_sides < 1{
            panic!("Die cannot have less than 1 side")
        }
        Die {
            sides: n_sides
        }
    }
    pub fn roll(&self) -> u32{
        let random_value = rand::thread_rng().gen_range(1..self.sides+1);
        random_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic()]
    fn sides_less_than_one(){
        let sides = 0;
        let die = Die::new(sides);
    }

    #[test]
    fn roll_value_less_than_or_equal_to_sides(){
        let sides = 6;
        let die= Die::new(sides);
        assert!(die.roll() <= sides)  
    }


    #[test]
    fn roll_value_greater_than_zero(){
        let sides = 6;
        let die= Die::new(sides);
        assert!(die.roll() > 0)
    }
}