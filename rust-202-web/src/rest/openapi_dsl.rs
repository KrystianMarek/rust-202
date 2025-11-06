//! OpenAPI documentation for DSL endpoints (when python-dsl feature is enabled)

#[cfg(feature = "python-dsl")]
use utoipa::OpenApi;

#[cfg(feature = "python-dsl")]
#[allow(unused_imports)]
use crate::rest::handlers::*;
#[cfg(feature = "python-dsl")]
#[allow(unused_imports)]
use crate::rest::models::*;

#[cfg(feature = "python-dsl")]
#[derive(OpenApi)]
#[allow(dead_code)]  // Used when python-dsl feature is enabled
#[openapi(
    paths(
        python_eval,
        python_execute,
        python_sandbox,
        python_transform,
    ),
    components(
        schemas(
            PythonEvalRequest,
            PythonEvalResponse,
            PythonScriptRequest,
            PythonScriptResponse,
            PythonSandboxRequest,
            PythonSandboxResponse,
            PythonTransformRequest,
            PythonTransformResponse,
        )
    ),
    tags(
        (name = "dsl", description = "Python DSL: Safe Python embedding with PyO3"),
    )
)]
pub struct DslApiDoc;

