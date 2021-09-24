struct Solution{
    nums: Vec<i32>,
    target: i32
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut position_vec:Vec<i32> = Vec::new();
        let mut temp = 0;
        for (i, element) in nums.iter().enumerate(){
            for (i_1, element_1) in nums.iter().enumerate(){
                
                temp = element + element_1;
                if i == i_1 {
                    break
                }

                if temp == target {
                   position_vec.push(i as i32);
                    position_vec.push(i_1 as i32);
                } 
            }
        }
        return position_vec;
    }

}

pub fn run_two_sum() {

    let sol = Solution::two_sum([2,7,11,15].to_vec(),9);
    println!("{:?}",sol);
}
