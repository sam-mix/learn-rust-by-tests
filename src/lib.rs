pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value:i32,
}

impl Guess{
    pub fn new(value:i32)->Guess {
        if value < 1 {
            panic!("desc1 {}", value);
        }
        if value > 100 {
            panic!("desc 2 {}", value);
        }
        Guess { value }
    }

    pub fn value(self) -> i32 {
        self.value
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn greeting_test() {
        let result = greeting("world");
        assert_eq!("Hello world!", result)
    }

    #[test]
    fn greeting_by_test() {
        let result = greeting("x");
        let target = "x";
        let test = "hhhhhhh";
        assert!(
            result.contains(target),
            "target: {}, result: {}, test: {}",
            target,
            result,
            test
        );
    }

    #[test]
    #[should_panic]
    fn guess_should_panic() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected="desc 2 200")]
    fn guess_should_panic_desc() {
        Guess::new(200);
    }

    #[test]
    fn is_work_result_t_e() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            Err(String::from("2加2不等于4"))
        }
    }


    #[test]
    #[should_panic]
    fn float_eq_panic() {
        // assert!(0.1+0.2 == 0.3);
        assert_eq!(0.1+0.2, 0.3);
    }

    #[test]
    #[should_panic]
    fn float_test_1() {
        let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2);
    }

}
