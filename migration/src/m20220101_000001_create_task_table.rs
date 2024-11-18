use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(pk_auto(Task::Id))
                    .col(string(Task::Title))
                    .col(string(Task::Description))
                    .col(string(Task::Status).default("pending"))
                    .col(date_time(Task::DateCreated).default(Expr::current_timestamp()))
                    .col(string_null(Task::DateUpdated))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TRIGGER set_date_updated
                AFTER UPDATE ON task
                FOR EACH ROW
                BEGIN
                    UPDATE task SET date_updated = CURRENT_TIMESTAMP WHERE id = NEW.id;
                END;",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    Title,
    Description,
    Status,
    DateCreated,
    DateUpdated,
}
