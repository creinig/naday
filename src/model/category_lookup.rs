use super::Category;
use std::collections::HashMap;
use std::rc::Rc;

/// Collection holding all known Category s, plus a method to find a category by its name or alias
#[derive(Debug)]
pub struct CategoryLookup {
    categories: HashMap<String, Rc<Category>>,
    by_name_or_alias: HashMap<String, Rc<Category>>,
}

impl CategoryLookup {
    pub fn new() -> CategoryLookup {
        CategoryLookup {
            categories: HashMap::new(),
            by_name_or_alias: HashMap::new(),
        }
    }

    /// Add the given category to the lookup.
    /// Re-adding a category will be silently ignored. Adding a category with a name or alias that
    /// is already in use will create an Err
    pub fn add(&mut self, category: Category) -> Result<(), String> {
        if self.categories.contains_key(&category.name) {
            return Ok(()); // No-op. Maybe emit a warning?
        }

        for name in category.all_names() {
            if self.by_name_or_alias.contains_key(name) {
                let msg = format!(
                    "Duplicate category key: '{}' is used by '{}' and '{}'",
                    name,
                    category.name,
                    self.find(name).unwrap().name
                );
                return Err(msg);
            }
        }

        let cat_rc = Rc::new(category);

        self.categories
            .insert(String::from(&cat_rc.name), cat_rc.clone());

        for name in cat_rc.all_names() {
            self.by_name_or_alias
                .insert(name.to_string(), cat_rc.clone());
        }

        Ok(())
    }

    /// Find a category by its name of alias
    pub fn find<S: AsRef<str>>(&self, alias_or_name: S) -> Option<Rc<Category>> {
        match self.by_name_or_alias.get(alias_or_name.as_ref()) {
            Some(cat) => Some(cat.clone()),
            None => None,
        }
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.categories.len()
    }

    pub fn iter(&self) -> std::collections::hash_map::Values<'_, String, Rc<Category>> {
        self.categories.values()
    }
}

//
// Tests ---------------------------
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_add_and_find() {
        let mut lookup = CategoryLookup::new();

        lookup
            .add(Category::new("Pushups", 1.0, vec!["pu", "push"]))
            .unwrap();
        lookup
            .add(Category::new("Situps", 1.0, vec!["su", "si"]))
            .unwrap();
        lookup
            .add(Category::new("Burpees", 1.5, vec!["bu", "oof"]))
            .unwrap();

        assert_eq!("Burpees", &(lookup.find("Burpees").unwrap().name));
        assert_eq!("Burpees", &(lookup.find("oof").unwrap().name));
        assert_eq!("Situps", &(lookup.find("su").unwrap().name));
        assert_eq!("Pushups", &(lookup.find("push").unwrap().name));
    }

    #[test]
    fn duplicates() {
        let mut lookup = CategoryLookup::new();

        assert_eq!(0, lookup.len());

        lookup
            .add(Category::new("Pushups", 1.0, vec!["pu", "push"]))
            .unwrap();
        lookup
            .add(Category::new("Pushups", 1.0, vec!["pu", "push"]))
            .unwrap();
        lookup
            .add(Category::new("Pushups", 1.0, vec!["pu", "push"]))
            .unwrap();

        assert_eq!(1, lookup.len());
    }

    #[test]
    fn duplicate_alias() {
        let mut lookup = CategoryLookup::new();

        assert_eq!(0, lookup.len());

        lookup
            .add(Category::new("Pushups", 1.0, vec!["pu", "push"]))
            .unwrap();
        lookup
            .add(Category::new("Pushdowns", 1.0, vec!["pd", "push"]))
            .expect_err("Should return an error for duplicate key");
    }
}
