pub trait ApiMapper<Entity, Presenter, Payload> {
    // Map an Entity to a Presenter
    fn to_api(entity: Entity) -> Presenter;

    // Map a Payload to an Entity
    fn to_entity(payload: Payload) -> Entity;
}

pub trait DbMapper<Entity, DbModel> {
    // Map an Entity to a DbModel
    fn to_db(entity: Entity) -> DbModel;

    // Map a DbModel to an Entity
    fn to_entity(model: DbModel) -> Entity;
}

pub trait HttpMapper<Entity, HttpObj> {
    // Map an Entity to an HttpObj
    fn to_http(entity: Entity) -> HttpObj;

    // Map an HttpObj to an Entity
    fn to_entity(http_obj: HttpObj) -> Entity;
}