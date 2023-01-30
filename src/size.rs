use std::default::Default;

#[derive(Clone, Debug)]
pub enum Size {
    Small,
    Large,
}
impl std::str::FromStr for Size {
    type Err = String;
    fn from_str(input: &str) -> Result<Size, Self::Err> {
        match input {
            "S" => Ok(Size::Small),
            "L" => Ok(Size::Large),
            _ => Err(format!("Invalid size: {input}")),
        }
    }
}
impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Small => write!(f, "S"),
            Size::Large => write!(f, "L"),
        }
    }
}

#[derive(Debug)]
pub struct SizeMap<T> {
    pub small: T,
    pub large: T,
}
impl<T> SizeMap<T> {
    pub fn new(small: T, large: T) -> Self {
        Self { small, large }
    }
    pub fn get_mut(&mut self, size: &Size) -> &mut T {
        match size {
            Size::Small => &mut self.small,
            Size::Large => &mut self.large,
        }
    }
}
impl<T: Default> Default for SizeMap<T> {
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}
