use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string_uniq(User::Email))
                    .col(string(User::Password))
                    .col(string_null(User::EmailVerificationCode))
                    .col(string_null(User::PasswordResetCode))
                    .col(json(User::Roles))
                    .col(
                        timestamp(User::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp())   
                    )
                    .col(
                        timestamp(User::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp())   
                    )
                    .to_owned(),
            )
            .await?;

        // Create a default user with password = "Admin12345@" using raw sql
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                INSERT INTO "user" (email, password, email_verification_code, password_reset_code, roles, created_at, updated_at)
                VALUES ('email@mail.com', '$2b$12$wprkV6CsyOoayEWpbFanLOJKllRNE4Oy5e03wJNvWARViQpoVu9Q2', NULL, NUll, '["admin", "new_account", "unverified_email"]', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                "#
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    Password,
    EmailVerificationCode,
    PasswordResetCode,
    Roles,
    CreatedAt,
    UpdatedAt,
}
