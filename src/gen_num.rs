mod num_gen{
    use rand::Rng;
    use round::round_up;
    pub struct Num{
       pub max:i64,
    }
    trait Number {
        fn generate_num( m:i64);
    }
    impl Num {
        pub fn generate_num( m:i64) {
            let num = rand::thread_rng().gen_range(1.0..16.0);
            let max = 15;
        
            for m in 0..max {
                println!("{}", round_up(num, 2))
            }
        }
    }
}