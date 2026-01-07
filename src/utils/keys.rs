use std::str::FromStr;
use rdev::Key;

impl FromStr for Key {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

    }
}
