// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

pub struct WeekTemperatures {
    temp: Vec<i32>,
}

pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekTemperatures {
    pub fn new() -> Self {
        WeekTemperatures {
            temp: vec![0; 7], // Initialize with 7 elements
        }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        match day {
            Weekday::Monday => self.temp.get(0).copied(),
            Weekday::Tuesday => self.temp.get(1).copied(),
            Weekday::Wednesday => self.temp.get(2).copied(),
            Weekday::Thursday => self.temp.get(3).copied(),
            Weekday::Friday => self.temp.get(4).copied(),
            Weekday::Saturday => self.temp.get(5).copied(),
            Weekday::Sunday => self.temp.get(6).copied(),
        }
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        match day {
            Weekday::Monday => self.temp[0] = temperature,
            Weekday::Tuesday => self.temp[1] = temperature,
            Weekday::Wednesday => self.temp[2] = temperature,
            Weekday::Thursday => self.temp[3] = temperature,
            Weekday::Friday => self.temp[4] = temperature,
            Weekday::Saturday => self.temp[5] = temperature,
            Weekday::Sunday => self.temp[6] = temperature,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(0));
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), Some(0));
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), Some(0));
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), Some(0));
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(0));
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), Some(0));
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(0));

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), Some(30));
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), Some(35));
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), Some(40));
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), Some(50));
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));

        week_temperatures.set_temperature(Weekday::Monday, 0);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(0));
    }
}
