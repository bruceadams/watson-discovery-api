# \CollectionsApi

All URIs are relative to *http://http:/discovery/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_collection**](CollectionsApi.md#create_collection) | **Post** /v1/environments/{environment_id}/collections | Create a collection
[**delete_collection**](CollectionsApi.md#delete_collection) | **Delete** /v1/environments/{environment_id}/collections/{collection_id} | Delete a collection
[**get_collection**](CollectionsApi.md#get_collection) | **Get** /v1/environments/{environment_id}/collections/{collection_id} | Get collection details
[**list_collection_fields**](CollectionsApi.md#list_collection_fields) | **Get** /v1/environments/{environment_id}/collections/{collection_id}/fields | List collection fields
[**list_collections**](CollectionsApi.md#list_collections) | **Get** /v1/environments/{environment_id}/collections | List collections
[**update_collection**](CollectionsApi.md#update_collection) | **Put** /v1/environments/{environment_id}/collections/{collection_id} | Update a collection


# **create_collection**
> ::models::Collection create_collection(ctx, ctx, environment_id, version, create_collection_request)
Create a collection

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **create_collection_request** | [**CreateCollectionRequest**](CreateCollectionRequest.md)| Input an object that allows you to add a collection. | 

### Return type

[**::models::Collection**](Collection.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_collection**
> ::models::DeleteCollectionResponse delete_collection(ctx, ctx, environment_id, collection_id, version)
Delete a collection

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::DeleteCollectionResponse**](DeleteCollectionResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_collection**
> ::models::Collection get_collection(ctx, ctx, environment_id, collection_id, version)
Get collection details

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::Collection**](Collection.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_collection_fields**
> ::models::ListCollectionFieldsResponse list_collection_fields(ctx, ctx, environment_id, collection_id, version)
List collection fields

Gets a list of the unique fields (and their types) stored in the index.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::ListCollectionFieldsResponse**](ListCollectionFieldsResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_collections**
> ::models::ListCollectionsResponse list_collections(ctx, ctx, environment_id, version, optional)
List collections

Lists existing collections for the service instance.

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
 **name** | **String**| Find collections with the given name. | 

### Return type

[**::models::ListCollectionsResponse**](ListCollectionsResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_collection**
> ::models::Collection update_collection(ctx, ctx, environment_id, collection_id, version, optional)
Update a collection

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **environment_id** | **String**| The ID of the environment. | 
 **collection_id** | **String**| The ID of the collection. | 
 **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
 **update_collection_request** | [**UpdateCollectionRequest**](UpdateCollectionRequest.md)| Input an object that allows you to update a collection. | 

### Return type

[**::models::Collection**](Collection.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

