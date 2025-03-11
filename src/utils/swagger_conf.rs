use crate::handler::task_handler::__path_create_task;
use crate::handler::task_handler::__path_delete_task;
use crate::handler::task_handler::__path_get_task;
use crate::handler::task_handler::__path_get_task_by_id;
use crate::handler::task_handler::__path_update_task;
use crate::model::task_model::{CreateTaskReq, TaskModel, UpdateTaskReq};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_task,
        get_task_by_id,
        create_task,
        update_task,
        delete_task,
    ),
    components(
        schemas(TaskModel, CreateTaskReq, UpdateTaskReq)
    ),
    tags(
        (name="Litle MS Project", description = "This is a Little MicroService")
    )
)]
pub struct ApiDoc;
