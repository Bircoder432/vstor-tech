use utoipa::{OpenApi, openapi::security::SecurityScheme};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Health
        crate::backend::api::health::health_check,
        crate::backend::api::health::ready_check,

        // Skills
        crate::backend::api::skills::get_skills,
        crate::backend::api::skills::get_skill,
        crate::backend::api::skills::create_skill,
        crate::backend::api::skills::update_skill,
        crate::backend::api::skills::delete_skill,
        crate::backend::api::skills::get_skills_by_category,
        crate::backend::api::skills::update_skill_name,
        crate::backend::api::skills::update_skill_category,
        crate::backend::api::skills::update_skill_visibility,
        crate::backend::api::skills::update_skill_level,
        crate::backend::api::skills::update_skill_description,
        crate::backend::api::skills::update_skill_years,
        crate::backend::api::skills::update_skill_image,
        crate::backend::api::skills::remove_skill_image,
        crate::backend::api::skills::delete_skills_batch,

        // Projects
        crate::backend::api::projects::get_projects,
        crate::backend::api::projects::get_project,
        crate::backend::api::projects::create_project,
        crate::backend::api::projects::get_featured_projects,
        crate::backend::api::projects::update_project_name,
        crate::backend::api::projects::update_project_featured,
        crate::backend::api::projects::update_project_status,
        crate::backend::api::projects::add_project_technology,
        crate::backend::api::projects::remove_project_technology,
        crate::backend::api::projects::update_project_description,
        crate::backend::api::projects::update_project_long_description,
        crate::backend::api::projects::update_project_visibility,
        crate::backend::api::projects::update_project_dates,
        crate::backend::api::projects::update_project_image,
        crate::backend::api::projects::remove_project_image,
        crate::backend::api::projects::add_project_url,
        crate::backend::api::projects::remove_project_url,
        crate::backend::api::projects::delete_project,
        crate::backend::api::projects::delete_projects_batch
    ),
    components(
        schemas(
            // Domain types
            crate::domain::types::SkillCategory,
            crate::domain::types::SkillLevel,
            crate::domain::types::ProjectStatus,
            crate::domain::types::ContentVisibility,

            // Shared types
            crate::shared::types::UrlType,
            crate::shared::types::ProjectUrl,
            crate::shared::types::ImageSource,

            // Skills DTOs
            crate::backend::models::responses::skills::SkillResponse,
            crate::backend::models::requests::skills::CreateSkillRequest,
            crate::backend::models::requests::skills::UpdateSkillNameRequest,
            crate::backend::models::requests::skills::UpdateSkillCategoryRequest,
            crate::backend::models::requests::skills::UpdateSkillVisibilityRequest,
            crate::backend::models::requests::skills::UpdateSkillLevelRequest,
            crate::backend::models::requests::skills::UpdateSkillDescriptionRequest,
            crate::backend::models::requests::skills::UpdateSkillYearsRequest,
            crate::backend::models::requests::skills::UpdateSkillImageRequest,
            crate::backend::models::requests::skills::AddSkillImageRequest,
            crate::backend::models::requests::skills::DeleteSkillsBatchRequest,
            crate::backend::models::responses::skills::MessageResponse,
            crate::backend::models::responses::skills::DeleteBatchResponse,

            // Projects DTOs
            crate::backend::models::responses::projects::ProjectResponse,
            crate::backend::models::requests::projects::CreateProjectRequest,
            crate::backend::models::requests::projects::UpdateProjectNameRequest,
            crate::backend::models::requests::projects::UpdateProjectFeaturedRequest,
            crate::backend::models::requests::projects::UpdateProjectStatusRequest,
            crate::backend::models::requests::projects::AddProjectTechnologyRequest,
            crate::backend::models::requests::projects::RemoveProjectTechnologyRequest,
            crate::backend::models::requests::projects::UpdateProjectDescriptionRequest,
            crate::backend::models::requests::projects::UpdateProjectLongDescriptionRequest,
            crate::backend::models::requests::projects::UpdateProjectVisibilityRequest,
            crate::backend::models::requests::projects::UpdateProjectDatesRequest,
            crate::backend::models::requests::projects::UpdateProjectImageRequest,
            crate::backend::models::requests::projects::AddProjectUrlRequest,
            crate::backend::models::requests::projects::RemoveProjectUrlRequest,
            crate::backend::models::requests::projects::DeleteProjectsBatchRequest,
            crate::backend::models::responses::projects::MessageResponse,
            crate::backend::models::responses::projects::DeleteBatchResponse,
        )
    ),
    tags(
        (name = "skills", description = "API для управления навыками"),
        (name = "projects", description = "API для управления проектами"),
        (name = "health", description = "Health checks")
    ),
    info(
        title = "VStor Tech Portfolio API",
        version = "1.0.0",
        description = "API для управления портфолио разработчика"
    ),
    modifiers(&SecurityAddon),
    security(
        ("bearer_auth" = [])
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            )
        }
    }
}
