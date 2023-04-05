pub trait ApiMapper<Entity, Dto> {
    fn map_to_entity(dto: Dto) -> Entity;

    fn map_to_dto(entity: &Entity) -> Dto;
}
