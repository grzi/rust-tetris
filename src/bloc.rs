use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub enum BlocKind {
    Border,
    Moving,
    Static,
}

pub struct Bloc {
    pub kind: BlocKind,
}

pub struct FallingBloc;

impl Bloc {
    pub fn new(k: BlocKind) -> Bloc {
        Bloc { kind: k }
    }
}

impl Component for Bloc {
    type Storage = DenseVecStorage<Self>;
}

impl Component for FallingBloc {
    type Storage = DenseVecStorage<Self>;
}
