#[derive(Eq, PartialEq)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

const SOGGY_WAFFLES: [Direction; 4] = [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST];

pub fn solve_floor() {
    let data = parse_data();

    let mut cur_direction = &Direction::EAST;
    let mut x = 0;
    let mut y = 0;

    for (direction, mut distance) in data {
        let direction = match direction {
            'N' => &Direction::NORTH,
            'E' => &Direction::EAST,
            'S' => &Direction::SOUTH,
            'W' => &Direction::WEST,
            'L' => {
                cur_direction = turn_left(cur_direction, distance);
                distance = 0;
                cur_direction
            }
            'R' => {
                cur_direction = turn_right(cur_direction, distance);
                distance = 0;
                cur_direction
            }
            'F' => cur_direction,
            _ => panic!("Unrecognized direction char!")
        };

        match direction {
            Direction::NORTH => { y += distance }
            Direction::SOUTH => { y -= distance }
            Direction::EAST => { x += distance }
            Direction::WEST => { x -= distance }
        }
    }

    println!("{}", (x.abs() + y.abs()));
}

pub fn solve_basement() {
    let data = parse_data();

    let mut dx = 10;
    let mut dy = 1;
    let mut x = 0;
    let mut y = 0;

    for (direction, distance) in data {
        match direction {
            'N' => dy += distance,
            'E' => dx += distance,
            'S' => dy -= distance,
            'W' => dx -= distance,
            'L' => {
                for _ in 0..distance / 90 {
                    let temp = dy;
                    dy = dx;
                    dx = -temp;
                }
            }
            'R' => {
                for _ in 0..distance / 90 {
                    let temp = dx;
                    dx = dy;
                    dy = -temp;
                }
            }
            'F' => {
                x += dx * distance;
                y += dy * distance
            }
            _ => panic!("Unrecognized direction char!")
        };
    }

    println!("{}", (x.abs() + y.abs()));
}

fn turn_left(cur_direction: &Direction, degrees: i32) -> &Direction {
    let index = SOGGY_WAFFLES.iter().position(|dir| dir == cur_direction).unwrap() as i32;
    return SOGGY_WAFFLES.get((((index - degrees / 90) + 4) % 4) as usize).unwrap();
}

fn turn_right(cur_direction: &Direction, degrees: i32) -> &Direction {
    let index = SOGGY_WAFFLES.iter().position(|dir| dir == cur_direction).unwrap() as i32;
    return SOGGY_WAFFLES.get(((index + degrees / 90) % 4) as usize).unwrap();
}

fn parse_data() -> Vec<(char, i32)> {
    let data: Vec<(char, i32)> = include_str!("../resources/12-1.txt")
        .lines()
        .map(|line| {
            (line.chars().next().unwrap(), line.get(1..).unwrap().parse().unwrap())
        })
        .collect();
    data
}
