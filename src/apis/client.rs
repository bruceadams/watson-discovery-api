use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  collections_api: Box<::apis::CollectionsApi>,
  configurations_api: Box<::apis::ConfigurationsApi>,
  credentials_api: Box<::apis::CredentialsApi>,
  documents_api: Box<::apis::DocumentsApi>,
  environments_api: Box<::apis::EnvironmentsApi>,
  events_and_feedback_api: Box<::apis::EventsAndFeedbackApi>,
  gateway_configuration_api: Box<::apis::GatewayConfigurationApi>,
  queries_api: Box<::apis::QueriesApi>,
  query_modifications_api: Box<::apis::QueryModificationsApi>,
  test_your_configuration_on_a_document_api: Box<::apis::TestYourConfigurationOnADocumentApi>,
  training_data_api: Box<::apis::TrainingDataApi>,
  user_data_api: Box<::apis::UserDataApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      collections_api: Box::new(::apis::CollectionsApiClient::new(rc.clone())),
      configurations_api: Box::new(::apis::ConfigurationsApiClient::new(rc.clone())),
      credentials_api: Box::new(::apis::CredentialsApiClient::new(rc.clone())),
      documents_api: Box::new(::apis::DocumentsApiClient::new(rc.clone())),
      environments_api: Box::new(::apis::EnvironmentsApiClient::new(rc.clone())),
      events_and_feedback_api: Box::new(::apis::EventsAndFeedbackApiClient::new(rc.clone())),
      gateway_configuration_api: Box::new(::apis::GatewayConfigurationApiClient::new(rc.clone())),
      queries_api: Box::new(::apis::QueriesApiClient::new(rc.clone())),
      query_modifications_api: Box::new(::apis::QueryModificationsApiClient::new(rc.clone())),
      test_your_configuration_on_a_document_api: Box::new(::apis::TestYourConfigurationOnADocumentApiClient::new(rc.clone())),
      training_data_api: Box::new(::apis::TrainingDataApiClient::new(rc.clone())),
      user_data_api: Box::new(::apis::UserDataApiClient::new(rc.clone())),
    }
  }

  pub fn collections_api(&self) -> &::apis::CollectionsApi{
    self.collections_api.as_ref()
  }

  pub fn configurations_api(&self) -> &::apis::ConfigurationsApi{
    self.configurations_api.as_ref()
  }

  pub fn credentials_api(&self) -> &::apis::CredentialsApi{
    self.credentials_api.as_ref()
  }

  pub fn documents_api(&self) -> &::apis::DocumentsApi{
    self.documents_api.as_ref()
  }

  pub fn environments_api(&self) -> &::apis::EnvironmentsApi{
    self.environments_api.as_ref()
  }

  pub fn events_and_feedback_api(&self) -> &::apis::EventsAndFeedbackApi{
    self.events_and_feedback_api.as_ref()
  }

  pub fn gateway_configuration_api(&self) -> &::apis::GatewayConfigurationApi{
    self.gateway_configuration_api.as_ref()
  }

  pub fn queries_api(&self) -> &::apis::QueriesApi{
    self.queries_api.as_ref()
  }

  pub fn query_modifications_api(&self) -> &::apis::QueryModificationsApi{
    self.query_modifications_api.as_ref()
  }

  pub fn test_your_configuration_on_a_document_api(&self) -> &::apis::TestYourConfigurationOnADocumentApi{
    self.test_your_configuration_on_a_document_api.as_ref()
  }

  pub fn training_data_api(&self) -> &::apis::TrainingDataApi{
    self.training_data_api.as_ref()
  }

  pub fn user_data_api(&self) -> &::apis::UserDataApi{
    self.user_data_api.as_ref()
  }


}
