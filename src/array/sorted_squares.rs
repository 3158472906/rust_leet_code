//977. 有序数组的平方
// 简单
// 682
// 相关企业
// 给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。
//
//
//
// 示例 1：
//
// 输入：nums = [-4,-1,0,3,10]
// 输出：[0,1,9,16,100]
// 解释：平方后，数组变为 [16,1,0,9,100]
// 排序后，数组变为 [0,1,9,16,100]
// 示例 2：
//
// 输入：nums = [-7,-3,2,3,11]
// 输出：[4,9,9,49,121]
//
//
// 提示：
//
// 1 <= nums.length <= 104
// -104 <= nums[i] <= 104
// nums 已按 非递减顺序 排序
//
//
// 进阶：
//
// 请你设计时间复杂度为 O(n) 的算法解决本问题
// 通过次数
// 446.2K
// 提交次数
// 649.9K
// 通过率
struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let nl = nums.len();

        let (mut s, mut e) = (0, nl - 1);
        let (mut res, mut index) = (vec![0; nl], e);

        while s <= e {
            let ss = nums[s] * nums[s];
            let ee = nums[e] * nums[e];

            if ss < ee {
                res[index] = ee;
                e -= 1
            } else {
                res[index] = ss;
                s += 1;
            }

            if index > 0 {
                index -= 1;
            }
        }

        res
    }
}


#[test]
fn test_solution() {
    let nums = vec![-4, -1, 0, 3, 10];
    let res = vec![0, 1, 9, 16, 100];

    assert_eq!(Solution::sorted_squares(nums), res)
}