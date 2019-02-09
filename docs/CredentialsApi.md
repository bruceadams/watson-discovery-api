# \CredentialsApi

All URIs are relative to *http://http:/discovery/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_credentials**](CredentialsApi.md#create_credentials) | **Post** /v1/environments/{environment_id}/credentials | Create credentials
[**delete_credentials**](CredentialsApi.md#delete_credentials) | **Delete** /v1/environments/{environment_id}/credentials/{credential_id} | Delete credentials
[**get_credentials**](CredentialsApi.md#get_credentials) | **Get** /v1/environments/{environment_id}/credentials/{credential_id} | View Credentials
[**list_credentials**](CredentialsApi.md#list_credentials) | **Get** /v1/environments/{environment_id}/credentials | List credentials
[**update_credentials**](CredentialsApi.md#update_credentials) | **Put** /v1/environments/{environment_id}/credentials/{credential_id} | Update credentials


# **create_credentials**
> ::models::Credentials create_credentials(ctx, ctx, version, environment_id, credentials)
Create credentials

Creates a set of credentials to connect to a remote source. Created credentials are used in a configuration to associate a collection with the remote source.  **Note:** All credentials are sent over an encrypted connection and encrypted at rest.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **environment_id** | **String**| The ID of the environment. | 
  **credentials** | [**Credentials**](Credentials.md)| An object that defines an individual set of source credentials. | 

### Return type

[**::models::Credentials**](Credentials.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_credentials**
> ::models::DeleteCredentials delete_credentials(ctx, ctx, version, environment_id, credential_id)
Delete credentials

Deletes a set of stored credentials from your Discovery instance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **environment_id** | **String**| The ID of the environment. | 
  **credential_id** | **String**| The unique identifier for a set of source credentials. | 

### Return type

[**::models::DeleteCredentials**](DeleteCredentials.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_credentials**
> ::models::Credentials get_credentials(ctx, ctx, version, environment_id, credential_id)
View Credentials

Returns details about the specified credentials.   **Note:** Secure credential information such as a password or SSH key is never returned and must be obtained from the source system.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **environment_id** | **String**| The ID of the environment. | 
  **credential_id** | **String**| The unique identifier for a set of source credentials. | 

### Return type

[**::models::Credentials**](Credentials.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_credentials**
> ::models::CredentialsList list_credentials(ctx, ctx, version, environment_id)
List credentials

List all the source credentials that have been created for this service instance.   **Note:**  All credentials are sent over an encrypted connection and encrypted at rest.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **environment_id** | **String**| The ID of the environment. | 

### Return type

[**::models::CredentialsList**](CredentialsList.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_credentials**
> ::models::Credentials update_credentials(ctx, ctx, version, environment_id, credential_id, credentials)
Update credentials

Updates an existing set of source credentials.  **Note:** All credentials are sent over an encrypted connection and encrypted at rest.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **environment_id** | **String**| The ID of the environment. | 
  **credential_id** | **String**| The unique identifier for a set of source credentials. | 
  **credentials** | [**Credentials**](Credentials.md)| An object that defines an individual set of source credentials. | 

### Return type

[**::models::Credentials**](Credentials.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

