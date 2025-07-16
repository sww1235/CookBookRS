use num_rational::Rational64;
use uom::si::{
    mass::{
        centigram, decagram, decigram, gigagram, gram, hectogram, kilogram, megagram, microgram, milligram, nanogram, ounce,
        picogram, pound, teragram,
    },
    rational64::{Mass, TemperatureInterval, Time, Volume},
    temperature_interval::{
        centikelvin, decakelvin, decikelvin, degree_celsius, degree_fahrenheit, degree_rankine, gigakelvin, hectokelvin, kelvin,
        kilokelvin, megakelvin, microkelvin, millikelvin, nanokelvin, picokelvin, terakelvin,
    },
    time::{
        centisecond, day, decasecond, decisecond, gigasecond, hectosecond, hour, kilosecond, megasecond, microsecond,
        millisecond, minute, nanosecond, picosecond, second, terasecond, year,
    },
    volume::{
        acre_foot, barrel, bushel, centiliter, cord, cubic_centimeter, cubic_decameter, cubic_decimeter, cubic_foot,
        cubic_gigameter, cubic_hectometer, cubic_inch, cubic_kilometer, cubic_megameter, cubic_meter, cubic_micrometer,
        cubic_mile, cubic_millimeter, cubic_nanometer, cubic_picometer, cubic_terameter, cubic_yard, cup, decaliter, deciliter,
        fluid_ounce, fluid_ounce_imperial, gallon, gallon_imperial, gigaliter, gill, gill_imperial, hectoliter, kiloliter, liter,
        megaliter, microliter, milliliter, nanoliter, peck, picoliter, pint_dry, pint_liquid, quart_dry, quart_liquid,
        tablespoon, teaspoon, teraliter,
    },
};

/// takes in a value and unit string and returns a `[uom::si::Time]` value.
pub fn time_unit_parser(value: Rational64, unit_string: &str) -> Time {
    match unit_string {
        "Ts" => Time::new::<terasecond>(value),
        "Gs" => Time::new::<gigasecond>(value),
        "Ms" => Time::new::<megasecond>(value),
        "ks" => Time::new::<kilosecond>(value),
        "hs" => Time::new::<hectosecond>(value),
        "das" => Time::new::<decasecond>(value),
        "s" => Time::new::<second>(value),
        "ds" => Time::new::<decisecond>(value),
        "cs" => Time::new::<centisecond>(value),
        "ms" => Time::new::<millisecond>(value),
        "µs" => Time::new::<microsecond>(value),
        "ns" => Time::new::<nanosecond>(value),
        "ps" => Time::new::<picosecond>(value),
        "d" => Time::new::<day>(value),
        "h" => Time::new::<hour>(value),
        "min" => Time::new::<minute>(value),
        "a" => Time::new::<year>(value),
        "placeholder" => panic!("Unit not specified for time_needed"),
        x => panic!("{x} not recognized as a supported time unit abbreviation"),
    }
}

/// takes a value and unit string and returns a `[uom::si::TemperatureInterval]` value.
pub fn temp_interval_unit_parser(value: Rational64, unit_string: &str) -> TemperatureInterval {
    match unit_string {
        "TK" => TemperatureInterval::new::<terakelvin>(value),
        "GK" => TemperatureInterval::new::<gigakelvin>(value),
        "MK" => TemperatureInterval::new::<megakelvin>(value),
        "kK" => TemperatureInterval::new::<kilokelvin>(value),
        "hK" => TemperatureInterval::new::<hectokelvin>(value),
        "daK" => TemperatureInterval::new::<decakelvin>(value),
        "K" => TemperatureInterval::new::<kelvin>(value),
        "dK" => TemperatureInterval::new::<decikelvin>(value),
        "cK" => TemperatureInterval::new::<centikelvin>(value),
        "mK" => TemperatureInterval::new::<millikelvin>(value),
        "µK" => TemperatureInterval::new::<microkelvin>(value),
        "nK" => TemperatureInterval::new::<nanokelvin>(value),
        "pK" => TemperatureInterval::new::<picokelvin>(value),
        "°C" => TemperatureInterval::new::<degree_celsius>(value),
        "°F" => TemperatureInterval::new::<degree_fahrenheit>(value),
        "°R" => TemperatureInterval::new::<degree_rankine>(value),
        "placeholder" => panic!("Unit not specified for temperature"),
        x => panic!("{x} not recognized as a supported temperature interval abbreviation"),
    }
}

/// takes a value and unit string and returns a `[uom::si::Mass]` value.
pub fn mass_unit_parser(value: Rational64, unit_string: &str) -> Mass {
    match unit_string {
        "Tg" => Mass::new::<teragram>(value),
        "Gg" => Mass::new::<gigagram>(value),
        "Mg" => Mass::new::<megagram>(value),
        "kg" => Mass::new::<kilogram>(value),
        "hg" => Mass::new::<hectogram>(value),
        "dag" => Mass::new::<decagram>(value),
        "g" => Mass::new::<gram>(value),
        "dg" => Mass::new::<decigram>(value),
        "cg" => Mass::new::<centigram>(value),
        "mg" => Mass::new::<milligram>(value),
        "µg" => Mass::new::<microgram>(value),
        "ng" => Mass::new::<nanogram>(value),
        "pg" => Mass::new::<picogram>(value),
        "oz" => Mass::new::<ounce>(value),
        "lb" => Mass::new::<pound>(value),
        "placeholder" => panic!("Unit not specified for ingredient mass"),
        x => panic!("{x} not recognized as a supported mass unit abbreviation"),
    }
}

/// takes a value and unit string and returns a `[uom::si::Volume]` value.
pub fn volume_unit_parser(value: Rational64, unit_string: &str) -> Volume {
    match unit_string {
        "Tm³" => Volume::new::<cubic_terameter>(value),
        "Gm³" => Volume::new::<cubic_gigameter>(value),
        "Mm³" => Volume::new::<cubic_megameter>(value),
        "km³" => Volume::new::<cubic_kilometer>(value),
        "hm³" => Volume::new::<cubic_hectometer>(value),
        "dam³" => Volume::new::<cubic_decameter>(value),
        "m³" => Volume::new::<cubic_meter>(value),
        "dm³" => Volume::new::<cubic_decimeter>(value),
        "cm³" => Volume::new::<cubic_centimeter>(value),
        "mm³" => Volume::new::<cubic_millimeter>(value),
        "µm³" => Volume::new::<cubic_micrometer>(value),
        "nm³" => Volume::new::<cubic_nanometer>(value),
        "pm³" => Volume::new::<cubic_picometer>(value),
        "ac · ft" => Volume::new::<acre_foot>(value),
        "bbl" => Volume::new::<barrel>(value),
        "bu" => Volume::new::<bushel>(value),
        "cords" => Volume::new::<cord>(value),
        "ft³" => Volume::new::<cubic_foot>(value),
        "in³" => Volume::new::<cubic_inch>(value),
        "mi³" => Volume::new::<cubic_mile>(value),
        "yd³" => Volume::new::<cubic_yard>(value),
        "cup" => Volume::new::<cup>(value),
        "fl oz" => Volume::new::<fluid_ounce>(value),
        "fl oz (UK)" => Volume::new::<fluid_ounce_imperial>(value),
        "gal (UK)" => Volume::new::<gallon_imperial>(value),
        "gal" => Volume::new::<gallon>(value),
        "gi (UK)" => Volume::new::<gill_imperial>(value),
        "gi" => Volume::new::<gill>(value),
        "TL" => Volume::new::<teraliter>(value),
        "GL" => Volume::new::<gigaliter>(value),
        "ML" => Volume::new::<megaliter>(value),
        "kL" => Volume::new::<kiloliter>(value),
        "hL" => Volume::new::<hectoliter>(value),
        "daL" => Volume::new::<decaliter>(value),
        "L" => Volume::new::<liter>(value),
        "dL" => Volume::new::<deciliter>(value),
        "cL" => Volume::new::<centiliter>(value),
        "mL" => Volume::new::<milliliter>(value),
        "µL" => Volume::new::<microliter>(value),
        "nL" => Volume::new::<nanoliter>(value),
        "pL" => Volume::new::<picoliter>(value),
        "pk" => Volume::new::<peck>(value),
        "dry pt" => Volume::new::<pint_dry>(value),
        "liq pt" => Volume::new::<pint_liquid>(value),
        "dry qt" => Volume::new::<quart_dry>(value),
        "liq qt" => Volume::new::<quart_liquid>(value),
        "tbsp" => Volume::new::<tablespoon>(value),
        "tsp" => Volume::new::<teaspoon>(value),
        "placeholder" => panic!("Unit not specified for ingredient mass"),
        x => panic!("{x} not recognized as a supported mass unit abbreviation"),
    }
}
