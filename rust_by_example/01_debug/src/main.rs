use std::fmt;

#[derive(Debug)] // required to print whole structs
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct MinMax(i64, i64);

// implement print 'Display' for the MinMax class (similar to C++)
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({}, {})", self.0, self.1);
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        return write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name, self.lat, lat_c, self.lon, lon_c
        );
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person {
        name: name,
        age: age,
    };
    println!("{:#?}", peter); // pretty print
    println!("{:?}\n", peter); // regular print

    let minmax = MinMax(69, 420);
    println!("Display: {}", minmax); // uses the fmt::Display implementation for MinMax
    println!("Debug: {:?}\n", minmax);

    let city_1 = City {
        name: "Los Angeles",
        lat: 53.43543,
        lon: -6.432432,
    };

    let city_2 = City {
        name: "San Francisco",
        lat: 69.43543,
        lon: -4.432432,
    };

    let mut cities: Vec<City> = vec![]; // same as Vec::new()
    cities.push(city_1);
    cities.push(city_2);

    for city in cities.iter() {
        println!("{}", city);
    }
}
