use enum_iterator::IntoEnumIterator;

pub struct Allergies {
    score: u32,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoEnumIterator)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score as u8 & *allergen as u8 > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::into_enum_iter().filter(|allergen| Self::is_allergic_to(&self, allergen)).collect()
    }
}
