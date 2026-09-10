mod blocks_routes;
mod charts_routes;
mod docs_routes;
mod workflows_routes;

pub use blocks_routes::BlockRoutes;
pub use charts_routes::ChartRoutes;
pub use docs_routes::{ComponentsRoutes, HooksRoutes};
pub use workflows_routes::WorkflowRoutes;
