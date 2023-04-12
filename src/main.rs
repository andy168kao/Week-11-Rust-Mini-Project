struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut i, mut j, mut k) = (0, 0, nums.len() as i32-1);
        while j<=k {
            match nums[j as usize] {
                0 => {nums.swap(i as usize,j as usize);i+=1;j+=1;},
                2 => {nums.swap(j as usize,k as usize);k-=1;},
                _ => {j+=1;}
            }
        }
    }
}

fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    println!("Before sorting: {:?}", nums);
    
    Solution::sort_colors(&mut nums);
    
    println!("After sorting: {:?}", nums);
}
