// mod Mathy;

mod gen_num;

fn main() {
    // let x = gen_num::Num{max: 10}
    
}

// fn problem() {
//     println!("x:");
//     println!("w:");
// }

// struct Units {
//     product: i64,
//     people: i64,
//     percent:f64,
//     time: f64,
// }

// trait Gen {
//     fn generate_fv();
//     fn generate_pv();
//     fn generate_iy();
//     fn generate_pmt();
//     fn generate_years();
// }

// impl Gen for Units{
    
//     fn generate_iy() {
//         // Generate random number with "%"
//         let percent = rand::thread_rng().gen_range(1.0..16.0);
        
//         // Rounds float to two decimal places
//         println!("I/Y = {}%", round_up(percent, 2));
//     }
    
//     fn generate_pv() {
//         // Generate random number with "$"
//         let pv = rand::thread_rng().gen_range(1.0..10000.0);
        
//         // Rounds float to two decimal places
//         println!("PV = ${}", round_up(pv, 2));
//     }
    
//     fn generate_fv() {
//         // Generate random number with "$"
//         let fv = rand::thread_rng().gen_range(1.0..10000.0);
        
//         // Rounds float to two decimal places
//         println!("FV = ${}", round_up(fv, 2));
//     }
    
//     fn generate_pmt() {
//         // Generate random number with "$"
//         let pmt = rand::thread_rng().gen_range(1.0..1000.0);
        
//         // Rounds float to two decimal places
//         println!("PMT = ${}", round_up(pmt, 2));
//     }
    
//     fn generate_years() {
//         // Generate random number with "_years"
//         let years = rand::thread_rng().gen_range(1..36);
        
//         // Rounds float to two decimal places
//         if years > 1 {
//             println!("Time = {} years", years);
//         } else {
//             println!("Time = {} year", years);
//         }
    
//     }
// }