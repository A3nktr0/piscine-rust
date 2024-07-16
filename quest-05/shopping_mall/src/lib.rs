pub mod mall;
pub use crate::mall::floor::{self, store};

pub fn biggest_store(mall: mall::Mall) -> store::Store {
    mall.floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .max_by_key(|store| store.square_meters)
        .cloned()
        .unwrap()
}

pub fn highest_paid_employee(mall: mall::Mall) -> Vec<store::employee::Employee> {
    mall.floors
        .into_iter()
        .flat_map(|floor| floor.stores.into_iter().flat_map(|store| store.employees))
        .fold(Vec::new(), |mut acc, employee| {
            if acc.is_empty() || employee.salary == acc[0].salary {
                acc.push(employee);
            } else if employee.salary > acc[0].salary {
                acc.clear();
                acc.push(employee);
            }
            acc
        })
}

pub fn nbr_of_employees(mall: mall::Mall) -> usize {
    let employees = mall.floors.into_iter().fold(0, |acc, floor| {
        acc + floor
            .stores
            .into_iter()
            .fold(0, |acc, store| acc + store.employees.len())
    });
    mall.guards.len() + employees
}

pub fn check_for_securities(mall: &mut mall::Mall, mut guards: Vec<mall::guard::Guard>) {
    let total_area: u64 = mall
        .floors
        .iter()
        .map(|floor| {
            floor
                .stores
                .iter()
                .map(|store| store.square_meters)
                .sum::<u64>()
        })
        .sum();

    let needed = total_area / 200;
    if needed > 0 {
        mall.guards.extend(guards.drain(0..=needed as usize));
    }
}

pub fn cut_or_raise(mall: &mut mall::Mall) {
    mall.floors.iter_mut().for_each(|floor| {
        floor.stores.iter_mut().for_each(|store| {
            store.employees.iter_mut().for_each(|employee| {
                if employee
                    .working_hours
                    .1
                    .checked_sub(employee.working_hours.0)
                    .unwrap_or_default()
                    > 10
                {
                    employee.salary *= 1.1;
                } else {
                    employee.salary *= 0.9;
                }
                employee.salary = (employee.salary * 10000.0).round() / 10000.0;
            });
        });
    });
}
