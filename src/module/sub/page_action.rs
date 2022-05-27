use sqlx::postgres::PgRow;
use sqlx::{Row, FromRow, PgPool, Error};

use crate::module::{Page, Action};

#[derive(Clone, Debug)]
pub struct PageAction {
    pub page: Page,
    pub action: Action,
}

impl<'r> FromRow<'r, PgRow> for PageAction {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(
            PageAction {
                page: Page {
                    id:    row.try_get("page_id")?,
                    mode:  row.try_get("page_mode")?,
                    page:  row.try_get("page_page")?,
                    path:  row.try_get("page_path")?,
                    param: row.try_get("page_param")?,
                },
                action: Action {
                    id:      row.try_get("action_id")?,
                    mode:    row.try_get("action_mode")?,
                    page:    row.try_get("action_page")?, 
                    template: row.try_get("action_template")?,
                    method:  row.try_get("action_method")?,
                    content: row.try_get("action_content")?,
                    auth:    row.try_get("action_auth")?,
                },
            }
        )
    }
}

impl<'a> PageAction {
    /*
    pub fn fetch(sql: &'a str, pool: &'a PgPool) -> impl TryStreamExt<Item = sqlx::Result<PageAction>> + 'a {
        sqlx::query_as::<_, PageAction>(sql).fetch(pool)   
    }
*/
    pub fn get_sql_all() -> String {
        "SELECT
            p.id as page_id,
            p.mode as page_mode,
            p.page as page_page,
            p.path as page_path,
            p.param as page_param,
            a.id as action_id,
            a.mode as action_mode,
            a.page as action_page,
            a.template as action_template,
            a.method as action_method,
            a.content as action_content,
            a.auth as action_auth
        FROM
            t_page p join t_action a
        ON
            p.id = a.page
        ORDER BY
            p.path ASC
        ".to_string()
    }

}