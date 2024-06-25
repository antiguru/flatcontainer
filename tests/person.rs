//! Test a slightly more struct with nested regions, representing people.

use flatcontainer::{Containerized, FlatStack, IntoOwned, Push, ReadRegion, Region, ReserveItems};

struct Person {
    name: String,
    age: u16,
    hobbies: Vec<String>,
}

impl Containerized for Person {
    type Region = PersonRegion;
}

#[derive(Default)]
struct PersonRegion {
    name_container: <String as Containerized>::Region,
    age_container: <u16 as Containerized>::Region,
    hobbies: <Vec<String> as Containerized>::Region,
}

#[derive(Debug, Clone, Copy)]
struct PersonRef<'a> {
    name: <<String as Containerized>::Region as ReadRegion>::ReadItem<'a>,
    age: <<u16 as Containerized>::Region as ReadRegion>::ReadItem<'a>,
    hobbies: <<Vec<String> as Containerized>::Region as ReadRegion>::ReadItem<'a>,
}

impl<'a> IntoOwned<'a> for PersonRef<'a> {
    type Owned = Person;

    fn into_owned(self) -> Self::Owned {
        Person {
            name: self.name.into_owned(),
            age: self.age,
            hobbies: self.hobbies.into_owned(),
        }
    }

    fn clone_onto(self, other: &mut Self::Owned) {
        self.name.clone_onto(&mut other.name);
        other.age = self.age;
        self.hobbies.clone_onto(&mut other.hobbies);
    }

    fn borrow_as(owned: &'a Self::Owned) -> Self {
        Self {
            name: IntoOwned::borrow_as(&owned.name),
            age: owned.age,
            hobbies: IntoOwned::borrow_as(&owned.hobbies),
        }
    }
}

impl ReadRegion for PersonRegion {
    type Owned = Person;
    type ReadItem<'a> = PersonRef<'a>
    where
        Self: 'a;
    type Index = (
        <<String as Containerized>::Region as ReadRegion>::Index,
        <<u16 as Containerized>::Region as ReadRegion>::Index,
        <<Vec<String> as Containerized>::Region as ReadRegion>::Index,
    );

    fn index(&self, (name, age, hobbies): Self::Index) -> Self::ReadItem<'_> {
        PersonRef {
            name: self.name_container.index(name),
            age: self.age_container.index(age),
            hobbies: self.hobbies.index(hobbies),
        }
    }
}

impl Region for PersonRegion {
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            name_container: <String as Containerized>::Region::merge_regions(
                regions.clone().map(|r| &r.name_container),
            ),
            age_container: <u16 as Containerized>::Region::merge_regions(
                regions.clone().map(|r| &r.age_container),
            ),
            hobbies: <Vec<String> as Containerized>::Region::merge_regions(
                regions.map(|r| &r.hobbies),
            ),
        }
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.name_container
            .reserve_regions(regions.clone().map(|r| &r.name_container));
        self.age_container
            .reserve_regions(regions.clone().map(|r| &r.age_container));
        self.hobbies
            .reserve_regions(regions.clone().map(|r| &r.hobbies));
    }

    fn clear(&mut self) {
        self.name_container.clear();
        self.age_container.clear();
        self.hobbies.clear();
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.name_container.heap_size(&mut callback);
        self.age_container.heap_size(&mut callback);
        self.hobbies.heap_size(callback);
    }

    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        PersonRef {
            name: <String as Containerized>::Region::reborrow(item.name),
            age: <u16 as Containerized>::Region::reborrow(item.age),
            hobbies: <Vec<String> as Containerized>::Region::reborrow(item.hobbies),
        }
    }
}

impl Push<&Person> for PersonRegion {
    fn push(&mut self, item: &Person) -> <PersonRegion as ReadRegion>::Index {
        let name = self.name_container.push(&item.name);
        let age = self.age_container.push(item.age);
        let hobbies = self.hobbies.push(&item.hobbies);
        (name, age, hobbies)
    }
}

impl<'a> ReserveItems<&'a Person> for PersonRegion {
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'a Person> + Clone,
    {
        self.name_container
            .reserve_items(items.clone().map(|i| &i.name));
        self.age_container
            .reserve_items(items.clone().map(|i| &i.age));
        self.hobbies.reserve_items(items.map(|i| &i.hobbies));
    }
}

impl Push<PersonRef<'_>> for PersonRegion {
    fn push(&mut self, item: PersonRef<'_>) -> <PersonRegion as ReadRegion>::Index {
        let name = self.name_container.push(item.name);
        let age = self.age_container.push(item.age);
        let hobbies = self.hobbies.push(item.hobbies);
        (name, age, hobbies)
    }
}

#[test]
fn test_person() {
    let hobbies = ["Computers", "Guitar"];
    let p = Person {
        name: "Moritz".to_string(),
        age: 123,
        hobbies: hobbies.iter().map(ToString::to_string).collect(),
    };

    let mut c = FlatStack::default_impl::<Person>();
    c.copy(&p);
    let person_ref = c.get(0);
    assert_eq!("Moritz", person_ref.name);
    assert_eq!(123, person_ref.age);
    assert_eq!(2, person_ref.hobbies.len());
    for (copied_hobby, hobby) in person_ref.hobbies.iter().zip(hobbies) {
        assert_eq!(copied_hobby, hobby);
    }

    let mut cc = FlatStack::default_impl::<Person>();

    cc.copy(c.get(0));

    let person_ref = cc.get(0);
    assert_eq!("Moritz", person_ref.name);
    assert_eq!(123, person_ref.age);
    assert_eq!(2, person_ref.hobbies.len());
    for (copied_hobby, hobby) in person_ref.hobbies.iter().zip(hobbies) {
        assert_eq!(copied_hobby, hobby);
    }
}
