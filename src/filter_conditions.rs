use sea_orm::Condition;

pub struct SeaOrmFilterConditions {
    pub must_conditions: Condition,
    pub any_conditions: Condition,
}
