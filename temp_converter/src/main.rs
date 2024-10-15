fn main() {
    let room_temp = Temperature {
        value: Temp::Celcius(27.0),
    };
    println!("Room temperature in Celcius is {}*C", room_temp.celcius());
    println!(
        "Room temperature in Farenheit is {}*F",
        room_temp.farenheit()
    );
}

enum Temp {
    Celcius(f64),
    Farenheit(f64),
}

struct Temperature {
    value: Temp,
}

impl Temperature {
    fn celcius(&self) -> f64 {
        match self.value {
            Temp::Celcius(temp) => temp,
            Temp::Farenheit(temp) => (temp - 32.0) / 1.8,
        }
    }

    fn farenheit(&self) -> f64 {
        match self.value {
            Temp::Celcius(temp) => (temp * 1.8) + 32.0,
            Temp::Farenheit(temp) => temp,
        }
    }
}
