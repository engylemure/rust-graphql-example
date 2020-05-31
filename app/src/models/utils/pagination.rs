use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;

pub trait Paginate: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            per_page: DEFAULT_PER_PAGE,
            page,
        }
    }
}

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
}

impl<T> Paginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated { per_page, ..self }
    }

    pub fn load_and_count_pages<U>(self, conn: &MysqlConnection) -> QueryResult<PaginatedData<U>>
    where
        Self: LoadQuery<MysqlConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let page = self.page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(PaginatedData::new(
            records,
            page,
            total_pages,
            per_page,
            total,
        ))
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<MysqlConnection> for Paginated<T> {}

impl<T> QueryFragment<Mysql> for Paginated<T>
where
    T: QueryFragment<Mysql>,
{
    fn walk_ast(&self, mut out: AstPass<Mysql>) -> QueryResult<()> {
        out.push_sql("SELECT *, ( SELECT COUNT(*) FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") as count ) FROM(");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page - 1) * self.per_page;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}

pub struct PaginatedData<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub total_pages: i64,
    pub per_page: i64,
    pub total: i64,
}

impl<T> PaginatedData<T> {
    fn new(
        data: Vec<T>,
        page: i64,
        total_pages: i64,
        per_page: i64,
        total: i64,
    ) -> PaginatedData<T> {
        Self {
            data,
            page,
            total_pages,
            per_page,
            total,
        }
    }
}
