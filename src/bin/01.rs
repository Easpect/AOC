advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut totalSum= 0;
    let mut leftList = Vec::new();
    let mut rightList = Vec::new();

    for line in input.lines()
    {
        let numVec : Vec<_>=line.split("   ").map(|c| c.parse::<i32>().unwrap()).collect();
        leftList.push(numVec[0]);
        rightList.push(numVec[1]);
    }
    leftList.sort();
    rightList.sort();
    let vecSize = leftList.len();
    for i in 0..vecSize
    {
        let vecSum = rightList[i] - leftList[i];
        let sumAdd = vecSum.abs();
        totalSum += sumAdd;
    }

    let returnSum: Option<u32> = Some(totalSum.try_into().unwrap());
    return(returnSum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut totalSum = 0;
    let mut leftList = Vec::new();
    let mut rightList = Vec::new();

    for line in input.lines()
    {
        let numVec : Vec<_>=line.split("   ").map(|c| c.parse::<i32>().unwrap()).collect();
        leftList.push(numVec[0]);
        rightList.push(numVec[1]);
    }

    let vecSize = leftList.len();
    for i in 0..vecSize
    {
        let mut repeat = 0;
        for j in 0..vecSize
        {
            if(leftList[i] == rightList[j])
            {
                repeat += 1;
            }
        }
        totalSum += leftList[i] * repeat;
    }
    let returnSum: Option<u32> = Some(totalSum.try_into().unwrap());
    return(returnSum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
