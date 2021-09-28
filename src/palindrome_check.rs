// Runtime: 8 ms, faster than 65.87% of Rust online submissions for Palindrome Number.
// Memory Usage: 2.1 MB, less than 29.16% of Rust online submissions for Palindrome Number.

// Input: x = 121
// Output: true

// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

struct Solution{
    x: i32
}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
    let initial_x:i64 =x.into();
    let mut num:i64 =x.into();
    let mut reversed_integer:i64 = 0;
    let mut rev_num:i64 = 0;
    
        // check if input number is in range of int32 then proceed for reverse
        if x < 2147483647 && x > -2147483648{
            // check if input number is negative if yes then return false
            if x < 0 {
                return false
                }
            else{
            
                while num > 0{
                    rev_num = rev_num *10 + num%10;
                    num = num/10;
                }
                // convert rev_num to i32 from i64
                reversed_integer = rev_num.into();
                if reversed_integer < 2147483647 && reversed_integer > -2147483648 {
                    // compare the reverse of a number with orginal to check if it's a palindrome.
                    if reversed_integer == initial_x{
                        return true
                    }
                    else{
                        return false;
                    }
                }
                else{
                    return false;
                }

            }
        }
        
        else{
            return false
        }    
    }
}


pub fn run_is_palindrome(){
   let sol =  Solution::is_palindrome(
    151);
   println!("{:?}",sol);
}