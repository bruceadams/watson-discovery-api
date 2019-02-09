# \EnvironmentsApi

All URIs are relative to *http://http:/discovery/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_environment**](EnvironmentsApi.md#create_environment) | **Post** /v1/environments | Create an environment
[**delete_environment**](EnvironmentsApi.md#delete_environment) | **Delete** /v1/environments/{environment_id} | Delete environment
[**get_environment**](EnvironmentsApi.md#get_environment) | **Get** /v1/environments/{environment_id} | Get environment info
[**list_environments**](EnvironmentsApi.md#list_environments) | **Get** /v1/environments | List environments
[**list_fields**](EnvironmentsApi.md#list_fields) | **Get** /v1/environments/{environment_id}/fields | List fields across collections
[**update_environment**](EnvironmentsApi.md#update_environment) | **Put** /v1/environments/{environment_id} | Update an environment


# **create_environment**
> ::models::Environment create_environment(ctx, ctx, version, create_environment_request)
Create an environment

Creates a new environment for private data. An environment must be created before collections can be created.   **Note**: You can create only one environment for private data per service instance. An attempt to create another environment results in an error.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **create_environment_request** | [**CreateEnvironmentRequest**](CreateEnvironmentRequest.md)| An object that defines an environment name and optional description. The fields in this object are not approved for personal information and cannot be deleted based on customer ID. | 

### Return type

[**::models::Environment**](Environment.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_environment**
> ::models::DeleteEnvironmentResponse delete_environment(ctx, ctx, environment_id, version)
Delete environment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::DeleteEnvironmentResponse**](DeleteEnvironmentResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_environment**
> ::models::Environment get_environment(ctx, ctx, environment_id, version)
Get environment info

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::Environment**](Environment.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_environments**
> ::models::ListEnvironmentsResponse list_environments(ctx, ctx, version, optional)
List environments

List existing environments for the service instance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
 **name** | **String**| Show only the environment with the given name. | 

### Return type

[**::models::ListEnvironmentsResponse**](ListEnvironmentsResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_fields**
> ::models::ListCollectionFieldsResponse list_fields(ctx, ctx, environment_id, collection_ids, version)
List fields across collections

Gets a list of the unique fields (and their types) stored in the indexes of the specified collections.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_ids** | [**Vec<String>**](String.md)| A comma-separated list of collection IDs to be queried against. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::ListCollectionFieldsResponse**](ListCollectionFieldsResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_environment**
> ::models::Environment update_environment(ctx, ctx, environment_id, version, update_environment_request)
Update an environment

Updates an environment. The environment's **name** and  **description** parameters can be changed. You must specify a **name** for the environment.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **update_environment_request** | [**UpdateEnvironmentRequest**](UpdateEnvironmentRequest.md)| An object that defines the environment's name and, optionally, description. | 

### Return type

[**::models::Environment**](Environment.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

