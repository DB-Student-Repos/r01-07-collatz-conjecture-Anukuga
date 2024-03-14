pub fn collatz(n: u64) -> Option<u64> {
    let mut steps = 0;
     let mut number = n;

     while number != 1 {
         if number % 2 == 0 {
             number /= 2;
         } else {
             number = 3 * number + 1;
         }
         steps += 1;
     }

     Some(steps)
}
