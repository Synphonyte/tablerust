use std::string::ToString;
use strum_macros::Display;


#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum Size {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum Shape {
    Rounded,
    RoundedCircle,
    Rounded0,
    Rounded1,
    Rounded2,
    Rounded3,
    // TODO : are there more?
}