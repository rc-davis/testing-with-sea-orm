use sea_orm_migration::prelude::*;

use super::m20241031_000001_create_bakery_table::Bakery;

#[derive(Iden)]
pub enum Chef {
    Table,
    Id,
    Name,
    ContactDetails,
    BakeryId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chef::Table)
                    .if_not_exists()
                    // .col(pk_auto(Chef::Id))
                    .col(
                        ColumnDef::new(Chef::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Chef::Name).string().not_null())
                    .col(ColumnDef::new(Chef::ContactDetails).json())
                    .col(ColumnDef::new(Chef::BakeryId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chef-bakery_id")
                            .from(Chef::Table, Chef::BakeryId)
                            .to(Bakery::Table, Bakery::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chef::Table).to_owned())
            .await
    }
}
