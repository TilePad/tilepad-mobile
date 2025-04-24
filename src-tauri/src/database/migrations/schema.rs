use sea_query::{ColumnDef, IntoIden};

pub fn pk_uuid<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name)
        .uuid()
        .not_null()
        .primary_key()
        .to_owned()
}

pub fn string<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name).string().not_null().to_owned()
}

pub fn string_null<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name).string().to_owned()
}

pub fn integer<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name).integer().not_null().to_owned()
}

pub fn date_time<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name).date_time().not_null().to_owned()
}
