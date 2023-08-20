pub mod sort {
    use async_graphql::{Enum, InputObject};

    #[derive(InputObject)]
    pub struct Sort {
        pub columns: Vec<Column>,
    }

    #[derive(InputObject)]
    pub struct Column {
        pub direction: Direction,
        pub name: String,
    }

    #[derive(Enum, Copy, Clone, Eq, PartialEq)]
    #[graphql(rename_items = "PascalCase")]
    pub enum Direction {
        Ascending,
        Descending,
    }
}
