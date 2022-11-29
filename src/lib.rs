pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn ret_unit_type() {
    let x = 1;
    let _z = if x % 2 == 1 {"odd"} else {"even"};
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
    use num::Complex;

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

    #[test]
    #[should_panic]
    fn nan_test() {
        let x = (-42.0_f32).sqrt();
        assert_eq!(x,x);
    }

    #[test]
    fn range_test() {
        for i in 1..=5 {
            println!("{}", i);
        }
    }

    #[test]
    fn complex_test() {
        let a = Complex {re:2.1, im:-1.2};
        let b = Complex::new(11.1,22.2);
        let result = a+b;
        println!("{} + {}i", result.re, result.im)
    }

    #[test]
    fn bool_test() {
        // let t = true;
        let f: bool = false; // 使用类型标注，显式指定f的类型

        if f {
            println!("这是段无意义的代码");
        }
    }

    #[test]
    fn ret_unit_type_test(){
        assert_eq!(ret_unit_type(),())
    }

    #[test]
    fn ref_and_brrow_test() {
        let x= 5;
        let y=&x;
        assert_eq!(5,x);
        assert_eq!(5,*y);
    }


}
