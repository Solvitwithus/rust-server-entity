pub use sea_orm_migration::prelude::*;


mod m20251230_170836_students_information;
mod m20251230_170856_staff_information;
mod m20251230_170908_departments_information;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
          
            Box::new(m20251230_170836_students_information::Migration),
            Box::new(m20251230_170856_staff_information::Migration),
            Box::new(m20251230_170908_departments_information::Migration),
        ]
    }
}
