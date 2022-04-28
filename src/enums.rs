
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
    #[strum(serialize = "rounded-0")]
    Rounded0,
    #[strum(serialize = "rounded-1")]
    Rounded1,
    #[strum(serialize = "rounded-2")]
    Rounded2,
    #[strum(serialize = "rounded-3")]
    Rounded3,
    // TODO : are there more?
}