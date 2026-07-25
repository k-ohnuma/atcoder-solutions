use std::sync::Arc;

use derive_new::new;
use domain::ports::{external::auth::IdentityProviderPort, repository::user::UserRepository};
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
        let user = input.into_user(identity.uid.clone());
        if let Err(error) = self.user_repository.create_user(user.to_owned()).await {
            let _ = self.identity_provider.delete_user(&identity.uid).await;
            return Err(UserError::from(error));
        }
        Ok(CreateUserOutput::new(user.user_name))
    }
}
