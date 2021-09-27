// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Reverse Integer.
// Memory Usage: 2.1 MB, less than 20.24% of Rust online submissions for Reverse Integer.

struct Solution{
    x: i32
}
impl Solution {
    pub fn reverse(x: i32) -> i32 {
    let mut num:i64 =x.into();
    let mut reversed_integer:i64 = 0;
    let mut rev_num:i64 = 0;
    let mut neg_sign = false;
    
        // check if input number is in range of int32 then proceed for reverse
        if x < 2147483647 && x > -2147483648{
            // check if input number is negative if yes then set neg_sign flag true
            if x < 0 {
                num = num.abs();
                neg_sign = true;
            
                while num > 0{
                    rev_num = rev_num *10 + num%10;
                    num = num/10;
                }
                // convert rev_num to i32 from i64
                reversed_integer = rev_num;
                if reversed_integer < 2147483647 && reversed_integer > -2147483648 {
                    
                    if neg_sign == true{
                        // println!("rev{}",reversed_integer);
                        
                        return -reversed_integer as i32
                    }
                    else{
                        return reversed_integer as i32
                    }
                }
                else{
                    reversed_integer= 0;
                    return reversed_integer as i32
                }
            }
            else{
                // num = num.abs();
                neg_sign = false;
            
                while num > 0{
                    rev_num = rev_num *10 + num%10;
                    num = num/10;
                }
                // convert rev_num to i32 from i64
                reversed_integer = rev_num ;
                if reversed_integer < 2147483647 && reversed_integer > -2147483648 {

                    if neg_sign == true{
                        
                        return -reversed_integer as i32
                    }
                    else{
                        return reversed_integer as i32
                    }
                }
                else{
                    reversed_integer= 0;
                    return reversed_integer as i32
                }
            }
        }
        
        else{
            return 0
        }    
    }
}


pub fn run_reverse_int(){
   let sol =  Solution::reverse(
    -1563847412);
   println!("{:.2}",sol);
}