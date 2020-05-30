#[derive(Associations, Queryable, Clone)]
pub struct AuthItemChildModel {
    pub parent: String,
    pub child: String,
}
