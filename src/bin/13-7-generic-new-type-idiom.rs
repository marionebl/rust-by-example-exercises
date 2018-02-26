fn main() {
    let age = Years(5);
    let days = age.to_days();
    println!("old enough {}", old_enough(&age));
    println!("old enough {}", old_enough(&days.to_years()));

    // error[E0308]: mismatched types: expected struct `Years`, found struct `Days`
    // println!("old enough {}", old_enough(&days));
}

struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    if age.0 >= 18 {
        return true;
    }
    false
}