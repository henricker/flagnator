pub trait ApiMapper<Entity, Presenter, Payload> {
    fn to_api(&self, entity: Entity) -> Presenter;
    fn to_entity(&self, payload: Payload) -> Entity;
}

pub trait DbMapper<Entity, DbModel> {
    fn to_db(&self, entity: &Entity) -> DbModel;
    fn to_entity(&self, model: &DbModel) -> Entity;
}

pub trait HttpMapper<Entity, HttpObj> {
    fn to_http(&self, entity: Entity) -> HttpObj;
    fn to_entity(&self, http_obj: HttpObj) -> Entity;
}
