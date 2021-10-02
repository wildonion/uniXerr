







pub mod ACL{
    use diesel::prelude::*;
    use crate::handlers::db::pg::establish as pg;
    use crate::entities::users::model::QueryableUser;
    use crate::handlers::error::uniXerr;
    use crate::schema::users;

    pub async fn set_access_level(username: Option<&String>, access_level: Option<&i16>) -> Result<QueryableUser, uniXerr>{
        let conn = pg::connection().await?;
        let username = username.unwrap();
        let access_level = access_level.unwrap();
        let updated_access_level = diesel::update(users::table.filter(users::username.eq(username))).set(users::access_level.eq(access_level)).get_result(&conn)?;
        Ok(updated_access_level)
    }
}
