pub struct Day {
    day: usize,
    part: Part,
}

impl Day {
    pub fn new(day: usize, part: Part) -> Self {
        Self { day, part }
    }
}

pub enum Part {
    A,
    B,
}

// impl ToString for Part {
//     fn to_string(&self) -> String {
//         match self {
//             A => String::from("A"),
//             B => String::from("B"),
//         }
//     }
// }

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::A => write!(f, "A"),
            Part::B => write!(f, "B"),
        }
    }
}

pub fn assert_eq<T: Eq>(day: Day, expected: T, result: T)
where
    T: std::fmt::Display,
{
    if expected == result {
        println!("Day {}:{} = {}", day.day, day.part, result);
    } else {
        panic!("Day {}:{} => {} != {}", day.day, day.part, expected, result);
    }
}
