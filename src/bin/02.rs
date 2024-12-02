advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut totalSafe = 0;

    for line in input.lines()
    {
        let reactor : Vec<_> = line.split(' ').map(|c| c.parse::<i32>().unwrap()).collect();
        let reportSize = reactor.len();
        //println!("Size of report {}\n", reportSize);
        //Lets compare the first 2 numbers to establish if increasing or decreasing.
        let mut firstVal;
        let mut currVal;

        if (reactor[1] - reactor[0]) < 0
        {
            firstVal = 0;
        }
        else {
            firstVal = 1;
        }

        if (reactor[1] - reactor[0]) == 0
        {
            //println!("Line break in line {}\n",line);
            continue;
        }
        let firstCheck = (reactor[1] - reactor[0]).abs();
        if(firstCheck <= 0 || firstCheck > 3)
        {
            continue;
        }

        for i in 2..reportSize
        {
            //println!("line {}\n", i);
            let check = reactor[i] - reactor[i-1];
            if check < 0
            {
                currVal = 0;
            }
            else
            {
                currVal = 1;
            }

            if currVal != firstVal
            {

                //println!("Line break in line {}\n",line);
                break;
            }
            else if check.abs() <= 0 || check.abs() > 3
            {
                //println!("Line break in line {}\n",line);
                break;
            }
            else if i == reportSize -1
            {
                //println!("Safe line: {}\n", line);
                totalSafe += 1;
            }
        }

    }
    //println!("Total Reports Safe: {}\n", totalSafe);
    return(Some(totalSafe.try_into().unwrap()));
    //If the difference between numbers is not between 1 and 3 (inclusive)
    //Must be only decreasing or increasing, not both
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut totalSafe = 0;

    for line in input.lines()
    {
        let reactor : Vec<_> = line.split(' ').map(|c| c.parse::<i32>().unwrap()).collect();
        let reportSize = reactor.len();
        //println!("Size of report {}\n", reportSize);
        //Lets compare the first 2 numbers to establish if increasing or decreasing.
        let mut firstVal;
        let mut currVal;
        let mut freebie = 1;

        if (reactor[1] - reactor[0]) < 0
        {
            firstVal = 0;
        }
        else {
            firstVal = 1;
        }

        if (reactor[1] - reactor[0]) == 0
        {
            //println!("Line break in line {}\n",line);
            freebie -= 1;
        }
        let firstCheck = (reactor[1] - reactor[0]).abs();
        if(firstCheck <= 0 || firstCheck > 3)
        {
            if freebie > 0
            {
                freebie -= 1;
            }
            else {
                continue;
            }
        }


        //println!("Freebie Count {}\n", freebie);
        for i in 2..reportSize
        {
            //println!("line {}\n", i);
            let check = reactor[i] - reactor[i-1];
            if check < 0
            {
                currVal = 0;
            }
            else
            {
                currVal = 1;
            }

            if currVal != firstVal
            {
                if freebie > 0
                {
                    freebie -= 1;
                }
                //println!("Line break in line {}\n",line);
                else{
                    break;
                }
            }
            else if check.abs() <= 0 || check.abs() > 3
            {
                if freebie > 0
                {
                    freebie -= 1;
                }
                //println!("Line break in line {}\n",line);
                else{
                    break;
                }
            }
            else if i == reportSize -1 && freebie == 1
            {
                //println!("Safe line: {}\n", line);
                totalSafe += 1;
            }
        }
        //println!("Freebie Count {}\n", freebie);


    }
    //println!("Total Reports Safe: {}\n", totalSafe);
    return(Some(totalSafe.try_into().unwrap()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
