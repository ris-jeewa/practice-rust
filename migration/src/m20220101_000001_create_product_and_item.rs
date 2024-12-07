use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the Product table
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(ColumnDef::new(Product::Description).string())
                    .col(
                        ColumnDef::new(Product::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Product::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?; // Await the Future

        // Create the Item table
        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Item::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Item::ProductId).integer().not_null())
                    .col(ColumnDef::new(Item::Size).string())
                    .col(ColumnDef::new(Item::Color).string())
                    .col(ColumnDef::new(Item::Stock).integer().not_null().default(0))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_item_product")
                            .from(Item::Table, Item::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .to_owned(),
            )
            .await?; // Await the Future

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the Item table
        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await?;

        // Drop the Product table
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Product {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Item {
    Table,
    Id,
    ProductId,
    Size,
    Color,
    Stock,
}
