//209. 长度最小的子数组
// 中等
// 1.5K
// 相关企业
// 给定一个含有 n 个正整数的数组和一个正整数 target 。
//
// 找出该数组中满足其和 ≥ target 的长度最小的 连续子数组 [numsl, numsl+1, ..., numsr-1, numsr] ，并返回其长度。如果不存在符合条件的子数组，返回 0 。
//
//
//
// 示例 1：
//
// 输入：target = 7, nums = [2,3,1,2,4,3]
// 输出：2
// 解释：子数组 [4,3] 是该条件下的长度最小的子数组。
// 示例 2：
//
// 输入：target = 4, nums = [1,4,4]
// 输出：1
// 示例 3：
//
// 输入：target = 11, nums = [1,1,1,1,1,1,1,1]
// 输出：0
//
//
// 提示：
//
// 1 <= target <= 109
// 1 <= nums.length <= 105
// 1 <= nums[i] <= 105
//
//
// 进阶：
//
// 如果你已经实现 O(n) 时间复杂度的解法, 请尝试设计一个 O(n log(n)) 时间复杂度的解法。

struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut nc, mut tmp_target, mut s) = (0, 0, 0);

        for (index, value) in nums.iter().enumerate() {
            tmp_target += value;

            while tmp_target >= target {
                let el = index - s + 1;
                if nc==0||el < nc{
                    nc = el;
                }

                tmp_target -= nums[s];
                s += 1;
            }
        }

        nc as i32
    }
}

#[test]
fn test_min_sub_array_len() {
    let nums = vec![2, 3, 1, 2, 4, 3];
    let target = 7;

    assert_eq!(Solution::min_sub_array_len(target, nums), 2)
}