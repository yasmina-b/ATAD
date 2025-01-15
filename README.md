# ATAD
# EASY PROBLEMS

# 1. Simple Array Sum

## Problem Overview

The task requires calculating the sum of an array of integers. Given an array `ar` of `n` integers, we need to return the sum of all the elements in the array.

### Solution Steps:

1. **`ar.iter()`**: 
   - This method creates an iterator over the array `ar`.
2. **`sum()`**:
   - The `sum()` method is called on the iterator created by `iter()`. It consumes the iterator and adds up all of its elements, returning the total sum. 

# 2. Plus Minus

## Problem Overview

The task requires calculating the ratio of positive, negative, and zero values in an array of integers.

### Solution Steps:

1. **Counting Positive, Negative, and Zero Values:**
   We loop through the array arr and count how many values are positive, negative, and zero; the ratio is the number divided by the total number of elements in the aray.
2. **Printing the Results:**
   The results are printed with a precision of six decimal places.
       ```rust
      println!("{:.6}", positive_ratio);
      println!("{:.6}", negative_ratio);
      println!("{:.6}", zero_ratio);
    ```
   When dividing integers, Rust would perform integer division, truncating the result. To prevent this and retain decimal precision, we cast the integers to `f64` before performing the division using the `as f64` syntax

# 3. Mini-Max Sum

## Problem Overview

Given five positive integers, the goal is to compute the minimum sum and the maximum sum that can be calculated by summing exactly four of the five integers. The solution should print the minimum and maximum values as two space-separated long integers.

### Solution Steps

1. **Calculate the Total Sum**
   Compute the total sum of all five integers in the array.
2. **Find Minimum and Maximum Values**
   Identify the smallest and largest values in the array.
3. **Compute Mini-Max Sums**
   Subtract the largest value from the total sum to get the minimum sum.
   Subtract the smallest value from the total sum to get the maximum sum.

# 4. Compare The Triplets

## Problem Overview

The Compare the Triplets problem involves comparing two sets of three scores, one from Alice and one from Bob, in three categories. Points are awarded based on who scores higher in each category:
   If Alice’s score is higher in a category, she earns 1 point.
   If Bob’s score is higher, he earns 1 point.
   If their scores are equal, no points are awarded.
The task is to determine the final scores for both Alice and Bob.

### Solution Steps

1. **Initialize Scores**
   Start with alice_score = 0 and bob_score = 0.
2. **Compare Scores**
   Loop through the scores in both lists:
   If Alice’s score is greater in a category, increment alice_score.
   If Bob’s score is greater, increment bob_score.
   Do nothing if the scores are equal.
3. **Return Results**
   Return the scores as a list [alice_score, bob_score].

# 5. Diagonal Difference

## Problem Overview

The Diagonal Difference problem involves calculating the absolute difference between the sums of the two diagonals of a square matrix. The matrix is of size n x n.

### Solution Steps

1. **Initialize Sums**
   Set two variables, primary_sum and secondary_sum, to 0 to hold the sums of the primary and secondary diagonals.
2. **Iterate Through the Matrix**
   Loop through the matrix rows:
   Add the primary diagonal element (matrix[i][i]) to primary_sum.
   Add the secondary diagonal element (matrix[i][n-1-i]) to secondary_sum.
3. **Calculate the Absolute Difference**
   Compute the absolute difference between primary_sum and secondary_sum.

# 6. Divisible Sum Pairs

## Problem Overview

The Divisible Sum Pairs problem involves finding how many pairs of elements in an array, ar, sum up to a number divisible by a given integer, k. The goal is to count how many such pairs exist in the array.

### Solution Steps

1. **Initialize a Counter for Pairs**
   Create a variable count to track the number of valid pairs.
2. **Iterate Through the Array**
   Use two nested loops to check each pair of elements (i, j) in the array, where i < j.
3. **Check Divisibility**
   For each pair (ar[i], ar[j]), check if the sum ar[i] + ar[j] is divisible by k (i.e., (ar[i] + ar[j]) % k == 0).
4. **Update the Counter**
   If the sum is divisible by k, increment the counter count.

# 7. Find Digits

## Problem Overview

The Find Digits problem requires determining how many digits of a given number evenly divide the number itself. Each digit in the number is checked, and digits that evenly divide the number are counted.

### Solution Steps

1. **Extract Digits**
   Iterate through each digit.
2. **Check Divisibility**
   For each digit:
   Ignore 0 (skip division by zero).
   Check if the number is divisible by the digit (n % digit == 0).
3. **Count Divisible Digits**
   Increment a counter for every digit that divides the number.
   Return the Count

# 8. Find The Median

## Problem Overview

The Find the Median problem requires finding the median value of an unsorted array. The median is the middle value when the array is sorted in ascending order.

### Solution Steps

1. **Sort the Array**
   Sort the array in ascending order.
2. **Find the Middle Element**
   Since the array length is guaranteed to be odd, the median is the element at the index n / 2.
3. **Return the Median**
   Return the value at the middle index as the result.

# 9. Staircase

## Problem Overview

The Staircase problem requires constructing a right-aligned staircase of height n using the # symbol. Each step of the staircase increases in width from 1 to n.

### Solution Steps

1. **Loop through Rows**
   Loop from 1 to n to represent each row of the staircase.
2. **Generate Each Row**
   For the i-th row:
   Calculate the number of spaces as n - i.
   Calculate the number of # symbols as i.
3. **Print the Staircase**
   Construct each row using the calculated spaces and # symbols, then print it.

# 10. Time Conversion

## Problem Overview

The Time Conversion problem involves converting a given time in a 12-hour AM/PM format to a 24-hour military time format. The input is a string representing time in the format hh:mm:ssAM or hh:mm:ssPM, and the task is to convert it to the 24-hour format HH:mm:ss.

### Solution Steps

1. **Extract Hour, Minute, and Second**
   Parse the input string to extract the hour, minute, second, and period (AM/PM).
2. **Handle AM/PM Conversion**
   If the period is AM:
      If the hour is 12, change it to 00 (for midnight).
      Otherwise, keep the hour unchanged.
   If the period is PM:
      If the hour is 12, leave it unchanged (for noon).
      For hours 01 to 11, add 12 to convert it to 24-hour format.
3. **Construct the 24-hour Format Time**
   Combine the modified hour, minute, and second into a new string in the 24-hour format.
   Return the converted time as a string in the HH:mm:ss format.

# MEDIUM PROBLEMS

# 1.  Extra Long Factorials

## Problem Overview

The problem asks us to calculate the factorial of a number `n` (denoted as `n!`). The task is to compute the factorial of a number `n` and print it, but in a way that it can handle extremely large numbers.

### Breakdown:

1. **Factorial Calculation**:
    - The factorial of a number `n` is calculated by multiplying all numbers from `1` to `n`

2. **Handling Large Numbers**:
    - Since the factorial grows exponentially, we need to handle very large numbers. In Rust, we can use a `Vec<i32>` to store each digit of the result separately, since standard integer types like `i32` or `u64` cannot handle the huge values that come with large factorials.

3. **Multiplication of Large Numbers**:
    - We multiply each number from `1` to `n` and store the result in the `Vec<i32>` where each element represents a single digit of the result, starting from the least significant digit (the units place).
    - To multiply the number by each digit, we handle any carry-over (i.e., when the product exceeds `9`, we carry over the extra value to the next higher place).

4. **Printing the Result**:
    - Once we have the result stored in a `Vec<i32>`, we print the digits starting from the most significant digit (the left-most digit) by reversing the vector.

### Solution Steps:

1. **Initialization**:
   We start by initializing a vector `result` with one element, `1`, because the factorial of `0` or `1` is `1`.

2. **Factorial Calculation**:
   - We loop through numbers from `2` to `n` and for each number, we multiply it with the current `result` using the `multiply` function.

3. **Multiplication Function**:
   - The `multiply` function takes the current result stored in the vector `result` and multiplies it by the current number (`num`). We keep track of any carry during multiplication and store the result back in the `result` vector.

4. **Final Output**:
   - After the factorial is computed, we print the result by iterating through the vector and printing each digit from the most significant to the least significant.


# 2.  Time in Words

## Problem Overview

The task is to convert a given time in hours and minutes into a human-readable string. The input consists of two integers:
- `h` (hours)
- `m` (minutes)

We need to output the time in a string format, where `h` and `m` are expressed in words.

### Breakdown:

1. **Mapping Numbers to Words**:
   - We start by defining a `time_words` array that contains the English words for numbers from 0 to 29. This helps us convert both hours and minutes to their word equivalents.

2. **Special Cases**:
   - **Exact Hour (`m == 0`)**: The time is represented as `h o' clock`, e.g., "five o' clock".
   - **Quarter Past (`m == 15`)**: If the minute is 15, it is represented as `quarter past h`, e.g., "quarter past five".
   - **Half Past (`m == 30`)**: If the minute is 30, it is represented as `half past h`, e.g., "half past five".
   - **Quarter To (`m == 45`)**: If the minute is 45, it is represented as `quarter to (h + 1)`, e.g., "quarter to six".
   
3. **General Case**:
   - **Minutes Less than 30**:
     - If `m < 30`, the format is `m minutes past h`. We handle singular and plural minute cases (e.g., "one minute" or "two minutes").
   - **Minutes Greater than 30**:
     - If `m > 30`, the format is `60 - m minutes to (h + 1)`, again handling singular and plural minute cases.

4. **Edge Case for Hours**:
   - We use modulo arithmetic (`h % 12`) to ensure that the hour value wraps around after 12 (e.g., if `h == 12`, the next hour will be `1`).

### Solution Steps:

1. **`time_words`**:
   - This array contains the words for numbers from 0 to 29 (for minutes), making it easy to map integer values to their string representations.

2. **Time Calculation**:
   - The function checks different conditions for the minutes value (`m`), and based on the value of `m`, it formats the time in words appropriately.
   - For minutes greater than 30, the function calculates how many minutes are left until the next hour and adjusts the hour accordingly (`(h + 1) % 12`).

3. **Return the Formatted String**:
   - The formatted string is returned in the form of "hour in words" and "minute in words", with the correct context ("past" or "to").


# 3. Flipping The Matrix

## Problem Overview

The problem involves maximizing the sum of the elements in the top-left n×n quadrant of a 2n×2n matrix. You are allowed to perform the following operations any number of times on the matrix:
   Row Flip: Reverse the order of elements in any row.
   Column Flip: Reverse the order of elements in any column.
The task is to maximize the sum of the elements in the top-left n×n quadrant after performing the optimal sequence of flips.

### Solution Steps

1. **Identify Quadrant Contribution**
   The top-left n x n quadrant of the matrix is the portion where the elements need to be maximized.
   For each position (i, j) in the top-left quadrant, there are up to four possible elements that can contribute to its final value:
      matrix[i][j] (original position),
      matrix[i][2n-1-j] (flipping the column),
      matrix[2n-1-i][j] (flipping the row),
      matrix[2n-1-i][2n-1-j] (flipping both row and column).

2. **Determine the Maximum Contribution**
   For each position (i, j) in the top-left n x n quadrant, calculate the maximum value among the four possible elements. This represents the optimal choice for that position after all possible flips.

3. **Calculate the Total Maximum Sum**
   Sum the maximum contributions for all positions in the top-left n x n quadrant.


# 4. Max Min

## Problem Overview

The Max Min problem involves minimizing the difference between the maximum and minimum values of a subset from a given array. You are given an array arr and a number k which represents the number of elements in a subset. You need to choose a subset of k elements from the array such that the difference between the maximum and minimum values in the subset is minimized.

### Solution Steps

1. **Sort the Array**
   Sort the array so the elements are in order. This helps us find a subset with the smallest difference between the largest and smallest values.

2. **Sliding Window**
   Use a sliding window of size k to look at all possible groups of k consecutive elements in the sorted array.
   The difference between the largest and smallest values in any group is the difference between the first and last elements of that group.

3. **Find the Smallest Difference**
   As you move the window across the sorted array, calculate the difference between the first and last elements of each group.
   Keep track of the smallest difference.
