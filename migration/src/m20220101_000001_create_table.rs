use sea_orm_migration::prelude::*;

use sea_orm::{EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())

                    .col(
                        ColumnDef::new(Post::Category)
                            .enumeration(Category::Table, [Category::Feed, Category::Story])
                            // Or, write it like below.
                            // Keep in mind that for it to work,
                            // 1. you need to derive `EnumIter`,
                            // 2. import `Iterable` into scope
                            // 3. and make sure `Category::Table` is the first variant
                            .enumeration(Category::Table, Category::iter().skip(1)),
                    )
                    .to_owned(),
            )
            .await?;


            manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-post_title")
                    .table(Post::Table)
                    .col(Post::Title)                        
                    .to_owned(),
            )
            .await?;

            Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();
        manager.drop_index(Index::drop().name("idx-post-title").to_owned())
        .await?;
        
        manager.drop_table(Table::drop().table(Post::Table).to_owned())
        .await?;
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    #[sea_orm(iden = "text")]
    Text,
    Category,
}


#[derive(Iden, EnumIter)]
pub enum Category {
    Table,
    #[iden = "Feed"]
    Feed,
    #[iden = "Story"]
    Story,
}