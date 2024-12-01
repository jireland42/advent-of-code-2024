/*
Problem

    You haven't even left yet and the group of Elvish Senior Historians has already hit a problem: their list of locations to check is currently empty. Eventually, someone decides that the best place to check first would be the Chief Historian's office.

    Upon pouring into the office, everyone confirms that the Chief Historian is indeed nowhere to be found. Instead, the Elves discover an assortment of notes and lists of historically significant locations! This seems to be the planning the Chief Historian was doing before he left. Perhaps these notes can be used to determine which locations to search?

    Throughout the Chief's office, the historically significant locations are listed not by name but by a unique number called the location ID. To make sure they don't miss anything, The Historians split into two groups, each searching the office and trying to create their own complete list of location IDs.

    There's just one problem: by holding the two lists up side by side (your puzzle input), it quickly becomes clear that the lists aren't very similar. Maybe you can help The Historians reconcile their lists?

    For example:

    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    Maybe the lists are only off by a small amount! To find out, pair up the numbers and measure how far apart they are. Pair up the smallest number in the left list with the smallest number in the right list, then the second-smallest left number with the second-smallest right number, and so on.

    Within each pair, figure out how far apart the two numbers are; you'll need to add up all of those distances. For example, if you pair up a 3 from the left list with a 7 from the right list, the distance apart is 4; if you pair up a 9 with a 3, the distance apart is 6.

    In the example list above, the pairs and distances would be as follows:

    The smallest number in the left list is 1, and the smallest number in the right list is 3. The distance between them is 2.
    The second-smallest number in the left list is 2, and the second-smallest number in the right list is another 3. The distance between them is 1.
    The third-smallest number in both lists is 3, so the distance between them is 0.
    The next numbers to pair up are 3 and 4, a distance of 1.
    The fifth-smallest numbers in each list are 3 and 5, a distance of 2.
    Finally, the largest number in the left list is 4, while the largest number in the right list is 9; these are a distance 5 apart.
    To find the total distance between the left list and the right list, add up the distances between all of the pairs you found. In the example above, this is 2 + 1 + 0 + 1 + 2 + 5, a total distance of 11!

    Your actual left and right lists contain many location IDs. What is the total distance between your lists?

Notes
    WRONG
    By ordering the lists from smallest to largest and labelling the elements as {x1, x2, .., xn} and {y1, y2, .., yn} the solution is (x1 - y1) + (x2 - y2) + .. + (xn - yn)
    This can be expanded and simplified as x1 + x2 + .. + xn - y1 - y2 - .. - yn

    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    -----
TOT: 16  27
    ------
DIF 11

    By ordering the lists from smallest to largest and labelling the elements as {x1, x2, .., xn} and {y1, y2, .., yn} the solution is |x1 - y1| + |x2 - y2| + .. + |xn - yn|
*/
use std::fs;

fn main()
{
    let file_path = "/Users/jesseireland/Documents/advent-of-code/advent-of-code-2024/01.in".to_string();

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut left_values = Vec::new();
    let mut right_values = Vec::new();

    for line in contents.lines()
    {
        let mut parts = line.split_whitespace().map(|s| s.parse::<i64>());
        match (parts.next(), parts.next())
        {
            (Some(Ok(left)), Some(Ok(right))) =>
            {
                left_values.push(left);
                right_values.push(right);
            }
            _ => {}  // ignore invalid input
        }
    }

    left_values.sort();
    right_values.sort();

    let mut total_distance = 0;

    for i in 0..left_values.len()
    {
        let mut distance = left_values[i] - right_values[i];
        if distance < 0 { distance *= -1 }
        total_distance += distance;
    }
    println!("{}", total_distance);

    // Guess 1: 568443 (too low - wrong approach)
    // Guess 2: 1829659 (too low - off-by-one error)
    // Guess 3: 1830467 (correct)
}
