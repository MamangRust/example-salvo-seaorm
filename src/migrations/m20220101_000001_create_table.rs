use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Firstname)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Lastname)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Password)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create categories table
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Categories::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Categories::Name)
                            .string()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await?;

        // Create posts table
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Posts::Title)
                            .string()
                            .not_null()
                            
                    )
                    .col(ColumnDef::new(Posts::Slug).string().not_null())
                    .col(ColumnDef::new(Posts::Img).string().not_null())
                    .col(ColumnDef::new(Posts::Body).text().not_null())
                    .col(ColumnDef::new(Posts::CategoryId).integer().not_null())
                    .col(ColumnDef::new(Posts::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Posts::UserName)
                            .string()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post-category_id")
                            .from(Posts::Table, Posts::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post-user_id")
                            .from(Posts::Table, Posts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create comments table
        manager
            .create_table(
                Table::create()
                    .table(Comments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comments::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comments::IdPostComment).integer().not_null())
                    .col(
                        ColumnDef::new(Comments::UserNameComment)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Comments::Comment)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-post_id")
                            .from(Comments::Table, Comments::IdPostComment)
                            .to(Posts::Table, Posts::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

 
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>  {
        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Firstname,
    Lastname,
    Email,
    Password,
}

#[derive(Iden)]
enum Categories {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Posts {
    Table,
    Id,
    Title,
    Slug,
    Img,
    Body,
    CategoryId,
    UserId,
    UserName,
}

#[derive(Iden)]
enum Comments {
    Table,
    Id,
    IdPostComment,
    UserNameComment,
    Comment,
}

impl Default for Migration {
    fn default() -> Self {
        Migration
    }
}