use super::assert::*;
use num::integer::lcm;

#[derive(Clone, Copy, Debug)]
struct Triple {
    x: i32,
    y: i32,
    z: i32,
}

impl Triple {
    fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

type Position = Triple;
type Velocity = Triple;

#[derive(Clone, Copy, Debug)]
struct Moon {
    position: Position,
    velocity: Velocity,
}

impl Moon {
    fn init(position: Position) -> Self {
        Self::new(position, Velocity::zero())
    }

    fn new(position: Position, velocity: Velocity) -> Self {
        Self { position, velocity }
    }

    fn apply_velocity(&self) -> Self {
        let p = self.position;
        let v = self.velocity;
        let position = Position::new(p.x + v.x, p.y + v.y, p.z + v.z);

        Self::new(position, self.velocity)
    }

    fn kinetic_energy(&self) -> i32 {
        let p = self.position;
        p.x.abs() + p.y.abs() + p.z.abs()
    }

    fn potential_energy(&self) -> i32 {
        let v = self.velocity;
        v.x.abs() + v.y.abs() + v.z.abs()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn apply_gravity(moons: &mut [Moon; 4]) {
    let index_pairs = vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

    for (i, j) in index_pairs {
        if moons[i].position.x > moons[j].position.x {
            moons[i].velocity.x -= 1;
            moons[j].velocity.x += 1;
        } else if moons[i].position.x < moons[j].position.x {
            moons[i].velocity.x += 1;
            moons[j].velocity.x -= 1;
        }

        if moons[i].position.y > moons[j].position.y {
            moons[i].velocity.y -= 1;
            moons[j].velocity.y += 1;
        } else if moons[i].position.y < moons[j].position.y {
            moons[i].velocity.y += 1;
            moons[j].velocity.y -= 1;
        }

        if moons[i].position.z > moons[j].position.z {
            moons[i].velocity.z -= 1;
            moons[j].velocity.z += 1;
        } else if moons[i].position.z < moons[j].position.z {
            moons[i].velocity.z += 1;
            moons[j].velocity.z -= 1;
        }
    }
}

fn apply_velocity(moons: &mut [Moon; 4]) {
    moons[0] = moons[0].apply_velocity();
    moons[1] = moons[1].apply_velocity();
    moons[2] = moons[2].apply_velocity();
    moons[3] = moons[3].apply_velocity();
}

pub fn solve() {
    let initial: [Moon; 4] = [
        Moon::init(Position::new(-16, -1, -12)),
        Moon::init(Position::new(0, -4, -17)),
        Moon::init(Position::new(-11, 11, 0)),
        Moon::init(Position::new(2, 2, -6)),
    ];
    let mut moons = initial.clone();
    let mut x_cycle: Option<i64> = None;
    let mut y_cycle: Option<i64> = None;
    let mut z_cycle: Option<i64> = None;
    let mut n = 1;

    loop {
        apply_gravity(&mut moons);
        apply_velocity(&mut moons);

        if n == 1000 {
            let energy: i32 = moons.iter().map(Moon::total_energy).sum();

            assert_eq(Day::new(12, Part::A), 5517, energy);
        }

        n += 1;

        if x_cycle == None
            && initial[0].position.x == moons[0].position.x
            && initial[1].position.x == moons[1].position.x
            && initial[2].position.x == moons[2].position.x
            && initial[3].position.x == moons[3].position.x
        {
            x_cycle = Some(n)
        }

        if y_cycle == None
            && initial[0].position.y == moons[0].position.y
            && initial[1].position.y == moons[1].position.y
            && initial[2].position.y == moons[2].position.y
            && initial[3].position.y == moons[3].position.y
        {
            y_cycle = Some(n)
        }

        if z_cycle == None
            && initial[0].position.z == moons[0].position.z
            && initial[1].position.z == moons[1].position.z
            && initial[2].position.z == moons[2].position.z
            && initial[3].position.z == moons[3].position.z
        {
            z_cycle = Some(n)
        }

        if let (Some(x), Some(y), Some(z)) = (x_cycle, y_cycle, z_cycle) {
            let frequency = lcm(x, lcm(y, z));

            assert_eq(Day::new(12, Part::B), 303_070_460_651_184, frequency);
            break;
        }
    }
}
