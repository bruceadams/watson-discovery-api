# \ConfigurationsApi

All URIs are relative to *http://http:/discovery/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_configuration**](ConfigurationsApi.md#create_configuration) | **Post** /v1/environments/{environment_id}/configurations | Add configuration
[**delete_configuration**](ConfigurationsApi.md#delete_configuration) | **Delete** /v1/environments/{environment_id}/configurations/{configuration_id} | Delete a configuration
[**get_configuration**](ConfigurationsApi.md#get_configuration) | **Get** /v1/environments/{environment_id}/configurations/{configuration_id} | Get configuration details
[**list_configurations**](ConfigurationsApi.md#list_configurations) | **Get** /v1/environments/{environment_id}/configurations | List configurations
[**update_configuration**](ConfigurationsApi.md#update_configuration) | **Put** /v1/environments/{environment_id}/configurations/{configuration_id} | Update a configuration


# **create_configuration**
> ::models::Configuration create_configuration(ctx, ctx, environment_id, version, configuration)
Add configuration

Creates a new configuration.  If the input configuration contains the **configuration_id**, **created**, or **updated** properties, then they are ignored and overridden by the system, and an error is not returned so that the overridden fields do not need to be removed when copying a configuration.  The configuration can contain unrecognized JSON fields. Any such fields are ignored and do not generate an error. This makes it easier to use newer configuration files with older versions of the API and the service. It also makes it possible for the tooling to add additional metadata and information to the configuration.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **configuration** | [**Configuration**](Configuration.md)| Input an object that enables you to customize how your content is ingested and what enrichments are added to your data.   **name** is required and must be unique within the current **environment**. All other properties are optional.  If the input configuration contains the **configuration_id**, **created**, or **updated** properties, then they will be ignored and overridden by the system (an error is not returned so that the overridden fields do not need to be removed when copying a configuration).   The configuration can contain unrecognized JSON fields. Any such fields will be ignored and will not generate an error. This makes it easier to use newer configuration files with older versions of the API and the service. It also makes it possible for the tooling to add additional metadata and information to the configuration. | 

### Return type

[**::models::Configuration**](Configuration.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_configuration**
> ::models::DeleteConfigurationResponse delete_configuration(ctx, ctx, environment_id, configuration_id, version)
Delete a configuration

The deletion is performed unconditionally. A configuration deletion request succeeds even if the configuration is referenced by a collection or document ingestion. However, documents that have already been submitted for processing continue to use the deleted configuration. Documents are always processed with a snapshot of the configuration as it existed at the time the document was submitted.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **configuration_id** | **String**| The ID of the configuration. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::DeleteConfigurationResponse**](DeleteConfigurationResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_configuration**
> ::models::Configuration get_configuration(ctx, ctx, environment_id, configuration_id, version)
Get configuration details

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **configuration_id** | **String**| The ID of the configuration. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::Configuration**](Configuration.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_configurations**
> ::models::ListConfigurationsResponse list_configurations(ctx, ctx, environment_id, version, optional)
List configurations

Lists existing configurations for the service instance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **environment_id** | **String**| The ID of the environment. | 
 **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
 **name** | **String**| Find configurations with the given name. | 

### Return type

[**::models::ListConfigurationsResponse**](ListConfigurationsResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_configuration**
> ::models::Configuration update_configuration(ctx, ctx, environment_id, configuration_id, version, configuration)
Update a configuration

Replaces an existing configuration.   * Completely replaces the original configuration.   * The **configuration_id**, **updated**, and **created** fields are accepted in the request, but they are ignored, and an error is not generated. It is also acceptable for users to submit an updated configuration with none of the three properties.   * Documents are processed with a snapshot of the configuration as it was at the time the document was submitted to be ingested. This means that already submitted documents will not see any updates made to the configuration.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **configuration_id** | **String**| The ID of the configuration. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **configuration** | [**Configuration**](Configuration.md)| Input an object that enables you to update and customize how your data is ingested and what enrichments are added to your data.  The **name** parameter is required and must be unique within the current **environment**. All other properties are optional, but if they are omitted  the default values replace the current value of each omitted property.  If the input configuration contains the **configuration_id**, **created**, or **updated** properties, they are ignored and overridden by the system, and an error is not returned so that the overridden fields do not need to be removed when updating a configuration.   The configuration can contain unrecognized JSON fields. Any such fields are ignored and do not generate an error. This makes it easier to use newer configuration files with older versions of the API and the service. It also makes it possible for the tooling to add additional metadata and information to the configuration. | 

### Return type

[**::models::Configuration**](Configuration.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

