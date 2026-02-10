class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        int i = 0;
        int size = nums.size();
        while(i < size){
            int numberChecking = target- nums[i];
            int j = size-1;
            while(j>0){
                if(j==i){
                    j--;
                    continue;
                }
                if(nums[j]==numberChecking){
                    vector<int> result{i,j};
                    return result;
                }
                j--;
            }
            i++;
        }
        vector<int> result{};
        return result;
    }
};