use std::sync::Arc;

use derive_new::new;
use domain::ports::{external::auth::IdentityProviderPort, repository::user::UserRepository};
use tracing::error;
use validator::Validate;

use crate::model::user::{
    UserError,
    create::{CreateUserInput, CreateUserOutput},
};

#[derive(new)]
pub struct CreateUserUsecase {
    user_repository: Arc<dyn UserRepository>,
    identity_provider: Arc<dyn IdentityProviderPort>,
}

impl CreateUserUsecase {
    pub async fn run(&self, input: CreateUserInput) -> Result<CreateUserOutput, UserError> {
        input
            .validate()
            .map_err(|e| UserError::BadRequest(e.to_string()))?;
        let identity = self
            .identity_provider
            .create_user(&input.email, &input.password)
            .await
            .map_err(UserError::from)?;
        let email = input.email.clone();
        let user = input.into_user(identity.uid.clone());
        if let Err(repository_error) = self.user_repository.create_user(user.to_owned()).await {
            if let Err(rollback_error) = self.identity_provider.delete_user(&identity.uid).await {
                error!(
                    uid = %identity.uid,
                    email = %email,
                    repository_error = ?repository_error,
                    rollback_error = ?rollback_error,
                    "failed to roll back Firebase Auth user after database user creation failed; administrative cleanup may be required"
                );
            }
            return Err(UserError::from(repository_error));
        }
        Ok(CreateUserOutput::new(user.user_name))
    }
}
