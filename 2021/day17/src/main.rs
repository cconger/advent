fn main() {
    // Hard-coded input parsing seems hard
    // target area: x=96..125, y=-144..-98
    let target = Target{
        x: (96,125),
        y: (-98,-144),
    };

    let sample_target = Target{
        x: (20, 30),
        y: (-5, -10),
    };

    let (v, highest) = search_highest(&target);
    println!("Part1 highest: {} ({}, {})", highest, v.0, v.1);

    let all = search_all(&target);
    println!("Part2 all_solutions: {}", all.len());

    //let (_, hit) = does_hit(init_p.clone(), test_v.clone(), &sample_target);
    //println!("Velo ({}, {}) hits? {}", test_v.0, test_v.1, hit)
}

fn x_to_hit(x: i32) -> i32 {
    let term = ((1 + 8 * x) as f64).sqrt();
    (term.ceil() as i32 - 1) / 2
}

fn search_all(target: &Target) -> Vec<Velocity> {
    let mut valid: Vec<Velocity> = Vec::new();

    for x_t in x_to_hit(target.x.0)..=target.x.1 {
        for y_t in target.y.1..target.y.1.abs() {
            let (_, hit) = does_hit(Point(0,0), Velocity(x_t, y_t), target, false);
            if hit {
                valid.push(Velocity(x_t, y_t));
            }
        }
    }

    return valid;
}

fn search_highest(target: &Target) -> (Velocity, i32) {
    let mut highest = 0;
    let mut highest_velo = Velocity(0,0);
    for x_t in x_to_hit(target.x.0)..=x_to_hit(target.x.1) {
        for y_t in 0..target.y.1.abs() {
            let (high, _hit) = does_hit(Point(0,0), Velocity(x_t, y_t), target, false);
            if high > highest {
                highest = high;
                highest_velo = Velocity(x_t, y_t);
            }
        }
    }
    return (highest_velo, highest);
}

#[derive(Clone)]
struct Point (i32, i32);

#[derive(Clone, Debug)]
struct Velocity (i32, i32);

struct Target {
    x: (i32, i32),
    y: (i32, i32),
}

fn does_hit(init_p: Point, v: Velocity, target: &Target, pp: bool) -> (i32, bool) {
    let mut v_ = v;
    let mut p_ = init_p;

    let mut highest = 0;

    // We missed if our y pos is below the lowest y with no vertical velocity or our x_pos >
    // highest x_pos
    while !((p_.1 < target.y.1 && v_.1 <= 0) || (p_.0 > target.x.1 )) {
        if p_.1 > highest { highest = p_.1; }
        if (p_.0 >= target.x.0) && (p_.0 <= target.x.1) && (p_.1 <= target.y.0) && (p_.1 >= target.y.1) {
            return (highest, true);
        }
        //Advance
        p_.0 += v_.0;
        p_.1 += v_.1;
        if v_.0 > 0 { v_.0 -= 1; }
        if v_.0 < 0 { v_.0 += 1; }
        v_.1 -= 1;
        if pp { println!("X: {}, Y: {}", p_.0, p_.1); }
    }
    return (-1, false);
}
