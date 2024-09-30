use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "instruments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub symbol: String,
    pub instrument_type: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_symbol(symbol: &str) -> Select<Entity> {
        Self::find().filter(Column::Symbol.eq(symbol))
    }
}
