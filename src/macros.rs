#[macro_export]
macro_rules! impl_crud {
    (  $struct:ty , $mod:ident , $into_err: ty, $id: ty) => {
        #[allow(dead_code)]
        pub async fn find_all(db: &DbConn) -> Result<Vec<$mod::Model>, $into_err> {
            use sea_orm::EntityTrait;

            Ok(<$struct>::find().all(db).await.map_err(Into::into)?)
        }

        #[allow(dead_code)]
        pub async fn find_by_id(db: &DbConn, id: $id) -> Result<Option<$mod::Model>, $into_err> {
            use sea_orm::EntityTrait;

            Ok(<$struct>::find_by_id(id)
                .one(db)
                .await
                .map_err(Into::into)?)
        }

        #[allow(dead_code)]
        pub async fn create(
            db: &DbConn,
            item: $mod::ActiveModel,
        ) -> Result<$mod::Model, $into_err> {
            use sea_orm::ActiveModelTrait;

            Ok(item.insert(db).await.map_err(Into::into)?)
        }

        #[allow(dead_code)]
        pub async fn update(
            db: &DbConn,
            item: $mod::ActiveModel,
        ) -> Result<$mod::Model, $into_err> {
            use sea_orm::ActiveModelTrait;

            Ok(item.update(db).await.map_err(Into::into)?)
        }

        #[allow(dead_code)]
        pub async fn delete(db: &DbConn, item: $mod::Model) -> Result<DeleteResult, $into_err> {
            use sea_orm::ModelTrait;

            Ok(item.delete(db).await.map_err(Into::into)?)
        }
    };
}
