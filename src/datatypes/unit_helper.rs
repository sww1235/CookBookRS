use num_rational::Rational64;
use uom::{
    fmt::DisplayStyle,
    si::{
        Unit,
        mass::{
            centigram, decagram, decigram, gigagram, gram, hectogram, kilogram, megagram, microgram, milligram, nanogram, ounce,
            picogram, pound, teragram,
        },
        rational64::{Mass, TemperatureInterval, Time, Volume},
        temperature_interval::{
            centikelvin, decakelvin, decikelvin, degree_celsius, degree_fahrenheit, degree_rankine, gigakelvin, hectokelvin,
            kelvin, kilokelvin, megakelvin, microkelvin, millikelvin, nanokelvin, picokelvin, terakelvin,
        },
        time::{
            centisecond, day, decasecond, decisecond, gigasecond, hectosecond, hour, kilosecond, megasecond, microsecond,
            millisecond, minute, nanosecond, picosecond, second, terasecond, year,
        },
        volume::{
            acre_foot, barrel, bushel, centiliter, cord, cubic_centimeter, cubic_decameter, cubic_decimeter, cubic_foot,
            cubic_gigameter, cubic_hectometer, cubic_inch, cubic_kilometer, cubic_megameter, cubic_meter, cubic_micrometer,
            cubic_mile, cubic_millimeter, cubic_nanometer, cubic_picometer, cubic_terameter, cubic_yard, cup, decaliter,
            deciliter, fluid_ounce, fluid_ounce_imperial, gallon, gallon_imperial, gigaliter, gill, gill_imperial, hectoliter,
            kiloliter, liter, megaliter, microliter, milliliter, nanoliter, peck, picoliter, pint_dry, pint_liquid, quart_dry,
            quart_liquid, tablespoon, teaspoon, teraliter,
        },
    },
};

/// takes in a value and unit string and returns a `[uom::si::Time]` value.
pub fn time_unit_input_parser(value: Rational64, unit_string: &str) -> Time {
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

/// takes in a `[uom::si::Time]` value and unit string and returns the raw value in the
/// specified unit for display or output to file.
pub fn time_unit_raw_output(value: Time, unit_string: &str) -> Rational64 {
    match unit_string {
        "Ts" => value.get::<terasecond>(),
        "Gs" => value.get::<gigasecond>(),
        "Ms" => value.get::<megasecond>(),
        "ks" => value.get::<kilosecond>(),
        "hs" => value.get::<hectosecond>(),
        "das" => value.get::<decasecond>(),
        "s" => value.get::<second>(),
        "ds" => value.get::<decisecond>(),
        "cs" => value.get::<centisecond>(),
        "ms" => value.get::<millisecond>(),
        "µs" => value.get::<microsecond>(),
        "ns" => value.get::<nanosecond>(),
        "ps" => value.get::<picosecond>(),
        "d" => value.get::<day>(),
        "h" => value.get::<hour>(),
        "min" => value.get::<minute>(),
        "a" => value.get::<year>(),
        "placeholder" => panic!("Unit not specified for time_needed"),
        x => panic!("{x} not recognized as a supported time unit abbreviation"),
    }
}

/// takes in a `[uom::si::Time]` value, unit string and `[uom::fmt::DisplayStyle]` and returns a formatted string in the
/// specified unit for display or output to file.
pub fn time_unit_format_output(value: Time, unit_string: &str, style: DisplayStyle) -> String {
    match unit_string {
        "Ts" => format!("{}", value.into_format_args(terasecond, style)),
        "Gs" => format!("{}", value.into_format_args(gigasecond, style)),
        "Ms" => format!("{}", value.into_format_args(megasecond, style)),
        "ks" => format!("{}", value.into_format_args(kilosecond, style)),
        "hs" => format!("{}", value.into_format_args(hectosecond, style)),
        "das" => format!("{}", value.into_format_args(decasecond, style)),
        "s" => format!("{}", value.into_format_args(second, style)),
        "ds" => format!("{}", value.into_format_args(decisecond, style)),
        "cs" => format!("{}", value.into_format_args(centisecond, style)),
        "ms" => format!("{}", value.into_format_args(millisecond, style)),
        "µs" => format!("{}", value.into_format_args(microsecond, style)),
        "ns" => format!("{}", value.into_format_args(nanosecond, style)),
        "ps" => format!("{}", value.into_format_args(picosecond, style)),
        "d" => format!("{}", value.into_format_args(day, style)),
        "h" => format!("{}", value.into_format_args(hour, style)),
        "min" => format!("{}", value.into_format_args(minute, style)),
        "a" => format!("{}", value.into_format_args(year, style)),
        "placeholder" => panic!("Unit not specified for time_needed"),
        x => panic!("{x} not recognized as a supported time unit abbreviation"),
    }
}

/// takes a value and unit string and returns a `[uom::si::TemperatureInterval]` value.
pub fn temp_interval_unit_input_parser(value: Rational64, unit_string: &str) -> TemperatureInterval {
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

/// takes a `[uom::si::TemperatureInterval]` and unit string and returns the raw value in the
/// specified unit for display or output to file.
pub fn temp_interval_unit_raw_output(value: TemperatureInterval, unit_string: &str) -> Rational64 {
    match unit_string {
        "TK" => value.get::<terakelvin>(),
        "GK" => value.get::<gigakelvin>(),
        "MK" => value.get::<megakelvin>(),
        "kK" => value.get::<kilokelvin>(),
        "hK" => value.get::<hectokelvin>(),
        "daK" => value.get::<decakelvin>(),
        "K" => value.get::<kelvin>(),
        "dK" => value.get::<decikelvin>(),
        "cK" => value.get::<centikelvin>(),
        "mK" => value.get::<millikelvin>(),
        "µK" => value.get::<microkelvin>(),
        "nK" => value.get::<nanokelvin>(),
        "pK" => value.get::<picokelvin>(),
        "°C" => value.get::<degree_celsius>(),
        "°F" => value.get::<degree_fahrenheit>(),
        "°R" => value.get::<degree_rankine>(),
        "placeholder" => panic!("Unit not specified for temperature"),
        x => panic!("{x} not recognized as a supported temperature interval abbreviation"),
    }
}

/// takes a `[uom::si::TemperatureInterval]` and unit string and returns a formatted string in the
/// specified unit for display or output to file.
pub fn temp_interval_unit_format_output(value: TemperatureInterval, unit_string: &str, style: DisplayStyle) -> String {
    match unit_string {
        "TK" => format!("{}", value.into_format_args(terakelvin, style)),
        "GK" => format!("{}", value.into_format_args(gigakelvin, style)),
        "MK" => format!("{}", value.into_format_args(megakelvin, style)),
        "kK" => format!("{}", value.into_format_args(kilokelvin, style)),
        "hK" => format!("{}", value.into_format_args(hectokelvin, style)),
        "daK" => format!("{}", value.into_format_args(decakelvin, style)),
        "K" => format!("{}", value.into_format_args(kelvin, style)),
        "dK" => format!("{}", value.into_format_args(decikelvin, style)),
        "cK" => format!("{}", value.into_format_args(centikelvin, style)),
        "mK" => format!("{}", value.into_format_args(millikelvin, style)),
        "µK" => format!("{}", value.into_format_args(microkelvin, style)),
        "nK" => format!("{}", value.into_format_args(nanokelvin, style)),
        "pK" => format!("{}", value.into_format_args(picokelvin, style)),
        "°C" => format!("{}", value.into_format_args(degree_celsius, style)),
        "°F" => format!("{}", value.into_format_args(degree_fahrenheit, style)),
        "°R" => format!("{}", value.into_format_args(degree_rankine, style)),
        "placeholder" => panic!("Unit not specified for temperature"),
        x => panic!("{x} not recognized as a supported temperature interval abbreviation"),
    }
}

/// takes a value and unit string and returns a `[uom::si::Mass]` value.
pub fn mass_unit_input_parser(value: Rational64, unit_string: &str) -> Mass {
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

/// takes a `[uom::si::Mass]` value and unit string and returns the raw value in the
/// specified unit for display or output to file.
pub fn mass_unit_raw_output(value: Mass, unit_string: &str) -> Rational64 {
    match unit_string {
        "Tg" => value.get::<teragram>(),
        "Gg" => value.get::<gigagram>(),
        "Mg" => value.get::<megagram>(),
        "kg" => value.get::<kilogram>(),
        "hg" => value.get::<hectogram>(),
        "dag" => value.get::<decagram>(),
        "g" => value.get::<gram>(),
        "dg" => value.get::<decigram>(),
        "cg" => value.get::<centigram>(),
        "mg" => value.get::<milligram>(),
        "µg" => value.get::<microgram>(),
        "ng" => value.get::<nanogram>(),
        "pg" => value.get::<picogram>(),
        "oz" => value.get::<ounce>(),
        "lb" => value.get::<pound>(),
        "placeholder" => panic!("Unit not specified for ingredient mass"),
        x => panic!("{x} not recognized as a supported mass unit abbreviation"),
    }
}

/// takes a `[uom::si::Mass]` value and unit string and returns a formatted string in the
/// specified unit for display or output to file.
pub fn mass_unit_format_output(value: Mass, unit_string: &str, style: DisplayStyle) -> String {
    match unit_string {
        "Tg" => format!("{}", value.into_format_args(teragram, style)),
        "Gg" => format!("{}", value.into_format_args(gigagram, style)),
        "Mg" => format!("{}", value.into_format_args(megagram, style)),
        "kg" => format!("{}", value.into_format_args(kilogram, style)),
        "hg" => format!("{}", value.into_format_args(hectogram, style)),
        "dag" => format!("{}", value.into_format_args(decagram, style)),
        "g" => format!("{}", value.into_format_args(gram, style)),
        "dg" => format!("{}", value.into_format_args(decigram, style)),
        "cg" => format!("{}", value.into_format_args(centigram, style)),
        "mg" => format!("{}", value.into_format_args(milligram, style)),
        "µg" => format!("{}", value.into_format_args(microgram, style)),
        "ng" => format!("{}", value.into_format_args(nanogram, style)),
        "pg" => format!("{}", value.into_format_args(picogram, style)),
        "oz" => format!("{}", value.into_format_args(ounce, style)),
        "lb" => format!("{}", value.into_format_args(pound, style)),
        "placeholder" => panic!("Unit not specified for ingredient mass"),
        x => panic!("{x} not recognized as a supported mass unit abbreviation"),
    }
}

/// takes a value and unit string and returns a `[uom::si::Volume]` value.
pub fn volume_unit_input_parser(value: Rational64, unit_string: &str) -> Volume {
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

/// takes a `[uom::si::Volume]` value and unit string and returns the raw value in the
/// specified unit for display or output to file.
pub fn volume_unit_raw_output(value: Volume, unit_string: &str) -> Rational64 {
    match unit_string {
        "Tm³" => value.get::<cubic_terameter>(),
        "Gm³" => value.get::<cubic_gigameter>(),
        "Mm³" => value.get::<cubic_megameter>(),
        "km³" => value.get::<cubic_kilometer>(),
        "hm³" => value.get::<cubic_hectometer>(),
        "dam³" => value.get::<cubic_decameter>(),
        "m³" => value.get::<cubic_meter>(),
        "dm³" => value.get::<cubic_decimeter>(),
        "cm³" => value.get::<cubic_centimeter>(),
        "mm³" => value.get::<cubic_millimeter>(),
        "µm³" => value.get::<cubic_micrometer>(),
        "nm³" => value.get::<cubic_nanometer>(),
        "pm³" => value.get::<cubic_picometer>(),
        "ac · ft" => value.get::<acre_foot>(),
        "bbl" => value.get::<barrel>(),
        "bu" => value.get::<bushel>(),
        "cords" => value.get::<cord>(),
        "ft³" => value.get::<cubic_foot>(),
        "in³" => value.get::<cubic_inch>(),
        "mi³" => value.get::<cubic_mile>(),
        "yd³" => value.get::<cubic_yard>(),
        "cup" => value.get::<cup>(),
        "fl oz" => value.get::<fluid_ounce>(),
        "fl oz (UK)" => value.get::<fluid_ounce_imperial>(),
        "gal (UK)" => value.get::<gallon_imperial>(),
        "gal" => value.get::<gallon>(),
        "gi (UK)" => value.get::<gill_imperial>(),
        "gi" => value.get::<gill>(),
        "TL" => value.get::<teraliter>(),
        "GL" => value.get::<gigaliter>(),
        "ML" => value.get::<megaliter>(),
        "kL" => value.get::<kiloliter>(),
        "hL" => value.get::<hectoliter>(),
        "daL" => value.get::<decaliter>(),
        "L" => value.get::<liter>(),
        "dL" => value.get::<deciliter>(),
        "cL" => value.get::<centiliter>(),
        "mL" => value.get::<milliliter>(),
        "µL" => value.get::<microliter>(),
        "nL" => value.get::<nanoliter>(),
        "pL" => value.get::<picoliter>(),
        "pk" => value.get::<peck>(),
        "dry pt" => value.get::<pint_dry>(),
        "liq pt" => value.get::<pint_liquid>(),
        "dry qt" => value.get::<quart_dry>(),
        "liq qt" => value.get::<quart_liquid>(),
        "tbsp" => value.get::<tablespoon>(),
        "tsp" => value.get::<teaspoon>(),
        "placeholder" => panic!("Unit not specified for ingredient mass"),
        x => panic!("{x} not recognized as a supported mass unit abbreviation"),
    }
}

/// takes a `[uom::si::Volume]` value and unit string and returns a formatted string in the
/// specified unit for display or output to file.
pub fn volume_unit_format_output(value: Volume, unit_string: &str, style: DisplayStyle) -> String {
    match unit_string {
        "Tm³" => format!("{}", value.into_format_args(cubic_terameter, style)),
        "Gm³" => format!("{}", value.into_format_args(cubic_gigameter, style)),
        "Mm³" => format!("{}", value.into_format_args(cubic_megameter, style)),
        "km³" => format!("{}", value.into_format_args(cubic_kilometer, style)),
        "hm³" => format!("{}", value.into_format_args(cubic_hectometer, style)),
        "dam³" => format!("{}", value.into_format_args(cubic_decameter, style)),
        "m³" => format!("{}", value.into_format_args(cubic_meter, style)),
        "dm³" => format!("{}", value.into_format_args(cubic_decimeter, style)),
        "cm³" => format!("{}", value.into_format_args(cubic_centimeter, style)),
        "mm³" => format!("{}", value.into_format_args(cubic_millimeter, style)),
        "µm³" => format!("{}", value.into_format_args(cubic_micrometer, style)),
        "nm³" => format!("{}", value.into_format_args(cubic_nanometer, style)),
        "pm³" => format!("{}", value.into_format_args(cubic_picometer, style)),
        "ac · ft" => format!("{}", value.into_format_args(acre_foot, style)),
        "bbl" => format!("{}", value.into_format_args(barrel, style)),
        "bu" => format!("{}", value.into_format_args(bushel, style)),
        "cords" => format!("{}", value.into_format_args(cord, style)),
        "ft³" => format!("{}", value.into_format_args(cubic_foot, style)),
        "in³" => format!("{}", value.into_format_args(cubic_inch, style)),
        "mi³" => format!("{}", value.into_format_args(cubic_mile, style)),
        "yd³" => format!("{}", value.into_format_args(cubic_yard, style)),
        "cup" => format!("{}", value.into_format_args(cup, style)),
        "fl oz" => format!("{}", value.into_format_args(fluid_ounce, style)),
        "fl oz (UK)" => format!("{}", value.into_format_args(fluid_ounce_imperial, style)),
        "gal (UK)" => format!("{}", value.into_format_args(gallon_imperial, style)),
        "gal" => format!("{}", value.into_format_args(gallon, style)),
        "gi (UK)" => format!("{}", value.into_format_args(gill_imperial, style)),
        "gi" => format!("{}", value.into_format_args(gill, style)),
        "TL" => format!("{}", value.into_format_args(teraliter, style)),
        "GL" => format!("{}", value.into_format_args(gigaliter, style)),
        "ML" => format!("{}", value.into_format_args(megaliter, style)),
        "kL" => format!("{}", value.into_format_args(kiloliter, style)),
        "hL" => format!("{}", value.into_format_args(hectoliter, style)),
        "daL" => format!("{}", value.into_format_args(decaliter, style)),
        "L" => format!("{}", value.into_format_args(liter, style)),
        "dL" => format!("{}", value.into_format_args(deciliter, style)),
        "cL" => format!("{}", value.into_format_args(centiliter, style)),
        "mL" => format!("{}", value.into_format_args(milliliter, style)),
        "µL" => format!("{}", value.into_format_args(microliter, style)),
        "nL" => format!("{}", value.into_format_args(nanoliter, style)),
        "pL" => format!("{}", value.into_format_args(picoliter, style)),
        "pk" => format!("{}", value.into_format_args(peck, style)),
        "dry pt" => format!("{}", value.into_format_args(pint_dry, style)),
        "liq pt" => format!("{}", value.into_format_args(pint_liquid, style)),
        "dry qt" => format!("{}", value.into_format_args(quart_dry, style)),
        "liq qt" => format!("{}", value.into_format_args(quart_liquid, style)),
        "tbsp" => format!("{}", value.into_format_args(tablespoon, style)),
        "tsp" => format!("{}", value.into_format_args(teaspoon, style)),
        "placeholder" => panic!("Unit not specified for ingredient mass"),
        x => panic!("{x} not recognized as a supported mass unit abbreviation"),
    }
}

/// `print_units` prints all unit names and abbreviations that are usable
/// in configuration and recipe files.
pub fn print_units() {
    // Time units
    println!("Only abbreviations are allowed in config files and recipe files for now");
    println!("Mass Units");

    println!("{}: {}", terasecond::singular(), terasecond::abbreviation());
    println!("{}: {}", gigasecond::singular(), gigasecond::abbreviation());
    println!("{}: {}", megasecond::singular(), megasecond::abbreviation());
    println!("{}: {}", kilosecond::singular(), kilosecond::abbreviation());
    println!("{}: {}", hectosecond::singular(), hectosecond::abbreviation());
    println!("{}: {}", decasecond::singular(), decasecond::abbreviation());
    println!("{}: {}", second::singular(), second::abbreviation());
    println!("{}: {}", decisecond::singular(), decisecond::abbreviation());
    println!("{}: {}", centisecond::singular(), centisecond::abbreviation());
    println!("{}: {}", millisecond::singular(), millisecond::abbreviation());
    println!("{}: {}", microsecond::singular(), microsecond::abbreviation());
    println!("{}: {}", nanosecond::singular(), nanosecond::abbreviation());
    println!("{}: {}", picosecond::singular(), picosecond::abbreviation());
    println!("{}: {}", day::singular(), day::abbreviation());
    println!("{}: {}", hour::singular(), hour::abbreviation());
    println!("{}: {}", minute::singular(), minute::abbreviation());
    println!("{}: {}", year::singular(), year::abbreviation());

    // Temp units
    println!("Only abbreviations are allowed in config files and recipe files for now");
    println!("Temperature Interval Units");

    println!("{}: {}", terakelvin::singular(), terakelvin::abbreviation());
    println!("{}: {}", gigakelvin::singular(), gigakelvin::abbreviation());
    println!("{}: {}", megakelvin::singular(), megakelvin::abbreviation());
    println!("{}: {}", kilokelvin::singular(), kilokelvin::abbreviation());
    println!("{}: {}", hectokelvin::singular(), hectokelvin::abbreviation());
    println!("{}: {}", decakelvin::singular(), decakelvin::abbreviation());
    println!("{}: {}", kelvin::singular(), kelvin::abbreviation());
    println!("{}: {}", decikelvin::singular(), decikelvin::abbreviation());
    println!("{}: {}", centikelvin::singular(), centikelvin::abbreviation());
    println!("{}: {}", millikelvin::singular(), millikelvin::abbreviation());
    println!("{}: {}", microkelvin::singular(), microkelvin::abbreviation());
    println!("{}: {}", nanokelvin::singular(), nanokelvin::abbreviation());
    println!("{}: {}", picokelvin::singular(), picokelvin::abbreviation());
    println!("{}: {}", degree_celsius::singular(), degree_celsius::abbreviation());
    println!("{}: {}", degree_fahrenheit::singular(), degree_fahrenheit::abbreviation());
    println!("{}: {}", degree_rankine::singular(), degree_rankine::abbreviation());

    // Mass units
    println!("Only abbreviations are allowed in config files and recipe files for now");
    println!("Mass Units");

    println!("{}: {}", teragram::singular(), teragram::abbreviation());
    println!("{}: {}", gigagram::singular(), gigagram::abbreviation());
    println!("{}: {}", megagram::singular(), megagram::abbreviation());
    println!("{}: {}", kilogram::singular(), kilogram::abbreviation());
    println!("{}: {}", hectogram::singular(), hectogram::abbreviation());
    println!("{}: {}", decagram::singular(), decagram::abbreviation());
    println!("{}: {}", gram::singular(), gram::abbreviation());
    println!("{}: {}", decigram::singular(), decigram::abbreviation());
    println!("{}: {}", centigram::singular(), centigram::abbreviation());
    println!("{}: {}", milligram::singular(), milligram::abbreviation());
    println!("{}: {}", microgram::singular(), microgram::abbreviation());
    println!("{}: {}", nanogram::singular(), nanogram::abbreviation());
    println!("{}: {}", picogram::singular(), picogram::abbreviation());
    println!("{}: {}", ounce::singular(), ounce::abbreviation());
    println!("{}: {}", pound::singular(), pound::abbreviation());

    // Volume Units
    println!("Only abbreviations are allowed in config files and recipe files for now");
    println!("Volume Units");

    println!("{}: {}", cubic_terameter::singular(), cubic_terameter::abbreviation());
    println!("{}: {}", cubic_gigameter::singular(), cubic_gigameter::abbreviation());
    println!("{}: {}", cubic_megameter::singular(), cubic_megameter::abbreviation());
    println!("{}: {}", cubic_kilometer::singular(), cubic_kilometer::abbreviation());
    println!("{}: {}", cubic_hectometer::singular(), cubic_hectometer::abbreviation());
    println!("{}: {}", cubic_decameter::singular(), cubic_decameter::abbreviation());
    println!("{}: {}", cubic_meter::singular(), cubic_meter::abbreviation());
    println!("{}: {}", cubic_decimeter::singular(), cubic_decimeter::abbreviation());
    println!("{}: {}", cubic_centimeter::singular(), cubic_centimeter::abbreviation());
    println!("{}: {}", cubic_millimeter::singular(), cubic_millimeter::abbreviation());
    println!("{}: {}", cubic_micrometer::singular(), cubic_micrometer::abbreviation());
    println!("{}: {}", cubic_nanometer::singular(), cubic_nanometer::abbreviation());
    println!("{}: {}", cubic_picometer::singular(), cubic_picometer::abbreviation());
    println!("{}: {}", acre_foot::singular(), acre_foot::abbreviation());
    println!("{}: {}", barrel::singular(), barrel::abbreviation());
    println!("{}: {}", bushel::singular(), bushel::abbreviation());
    println!("{}: {}", cord::singular(), cord::abbreviation());
    println!("{}: {}", cubic_foot::singular(), cubic_foot::abbreviation());
    println!("{}: {}", cubic_inch::singular(), cubic_inch::abbreviation());
    println!("{}: {}", cubic_mile::singular(), cubic_mile::abbreviation());
    println!("{}: {}", cubic_yard::singular(), cubic_yard::abbreviation());
    println!("{}: {}", cup::singular(), cup::abbreviation());
    println!("{}: {}", fluid_ounce::singular(), fluid_ounce::abbreviation());
    println!(
        "{}: {}",
        fluid_ounce_imperial::singular(),
        fluid_ounce_imperial::abbreviation()
    );
    println!("{}: {}", gallon_imperial::singular(), gallon_imperial::abbreviation());
    println!("{}: {}", gallon::singular(), gallon::abbreviation());
    println!("{}: {}", gill_imperial::singular(), gill_imperial::abbreviation());
    println!("{}: {}", gill::singular(), gill::abbreviation());
    println!("{}: {}", teraliter::singular(), teraliter::abbreviation());
    println!("{}: {}", gigaliter::singular(), gigaliter::abbreviation());
    println!("{}: {}", megaliter::singular(), megaliter::abbreviation());
    println!("{}: {}", kiloliter::singular(), kiloliter::abbreviation());
    println!("{}: {}", hectoliter::singular(), hectoliter::abbreviation());
    println!("{}: {}", decaliter::singular(), decaliter::abbreviation());
    println!("{}: {}", liter::singular(), liter::abbreviation());
    println!("{}: {}", deciliter::singular(), deciliter::abbreviation());
    println!("{}: {}", centiliter::singular(), centiliter::abbreviation());
    println!("{}: {}", milliliter::singular(), milliliter::abbreviation());
    println!("{}: {}", microliter::singular(), microliter::abbreviation());
    println!("{}: {}", nanoliter::singular(), nanoliter::abbreviation());
    println!("{}: {}", picoliter::singular(), picoliter::abbreviation());
    println!("{}: {}", peck::singular(), peck::abbreviation());
    println!("{}: {}", pint_dry::singular(), pint_dry::abbreviation());
    println!("{}: {}", pint_liquid::singular(), pint_liquid::abbreviation());
    println!("{}: {}", quart_dry::singular(), quart_dry::abbreviation());
    println!("{}: {}", quart_liquid::singular(), quart_liquid::abbreviation());
    println!("{}: {}", tablespoon::singular(), tablespoon::abbreviation());
    println!("{}: {}", teaspoon::singular(), teaspoon::abbreviation());

    println!("Only abbreviations are allowed in config files and recipe files for now");
}
