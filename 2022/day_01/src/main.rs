use calorie_counting::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod calorie_counting {
    pub fn solution_01(input: Vec<Option<u32>>) -> u32 {
        elf_inventories_from_input(input)
            .iter()
            .map(|ei| ei.total())
            .max()
            .unwrap_or(0)
    }

    pub fn solution_02(input: Vec<Option<u32>>) -> u32 {
        let mut total_calories_on_inventories: Vec<u32> = elf_inventories_from_input(input)
            .iter()
            .map(|ei| ei.total())
            .collect();
        total_calories_on_inventories.sort_by(|a, b| b.cmp(a));
        total_calories_on_inventories.iter().take(3).sum()
    }

    #[derive(Default, Debug)]
    pub struct ElfInventory {
        inventory: Vec<u32>,
        total: u32,
    }

    impl ElfInventory {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn push_meal_calorie(&mut self, meal_calorie: u32) {
            self.inventory.push(meal_calorie);
            self.total += meal_calorie;
        }

        pub fn inventory(&self) -> &Vec<u32> {
            &self.inventory
        }

        pub fn total(&self) -> u32 {
            self.total
        }
    }

    pub fn elf_inventories_from_input(meal_calories: Vec<Option<u32>>) -> Vec<ElfInventory> {
        let mut elf_inventory: Option<ElfInventory> = None;
        let mut elf_inventories = Vec::new();
        for meal_calorie in meal_calories {
            if let Some(meal_calorie) = meal_calorie {
                if elf_inventory.is_none() {
                    elf_inventory = Some(ElfInventory::new());
                }
                elf_inventory
                    .as_mut()
                    .unwrap()
                    .push_meal_calorie(meal_calorie);
            } else if let Some(elf_inventory) = elf_inventory.take() {
                elf_inventories.push(elf_inventory);
            }
        }
        if let Some(elf_inventory) = elf_inventory {
            elf_inventories.push(elf_inventory);
        }
        elf_inventories
    }

    pub fn transform_input(input: &str) -> Vec<Option<u32>> {
        input.lines().map(|line| line.parse().ok()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::calorie_counting::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 24000);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 45000);
    }
}
