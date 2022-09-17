use crate::Result;

///
/// ORMのActiveModel生成トレイト
///
pub trait ActiveModelGenerator{
    type Entity;
    type ActiveModel;
    fn active_model(entity: &Self::Entity) -> Self::ActiveModel;
}
///
///  EntityとORM側のモデル変換トレイト
///
pub trait ModelAndEntity{
    type Entity;
    type Model;
    // ORMモデルからEntityに変換する
    fn model_to_entity(model: &Self::Model) -> Result<Self::Entity>;
    // EntityからORMモデルに変換する
    fn entity_to_model(entity: &Self::Entity) -> Self::Model;
}

///
/// Vec<ORMモデル>をVec<Entity>に変換するトレイト
///
pub trait VecModelToVecEntity {
    type Entity;
    type Model;
    type JoinModel;
    // 1種類のVec<ORMモデル>をVe<Entity>に変換する
    fn entities(_: &Vec<Self::Model>) -> Result<Vec<Self::Entity>>{
        todo!()
    }
    // 結合結果のVec<ORMモデル>をVec<Entity>に変換する
    fn join_model_to_entities(_: &Vec<(Self::Model , Option<Self::JoinModel>)>) -> Result<Vec<Self::Entity>>{
        todo!()
    }
}