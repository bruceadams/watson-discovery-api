# \TrainingDataApi

All URIs are relative to *http://http:/discovery/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_training_data**](TrainingDataApi.md#add_training_data) | **Post** /v1/environments/{environment_id}/collections/{collection_id}/training_data | Add query to training data
[**create_training_example**](TrainingDataApi.md#create_training_example) | **Post** /v1/environments/{environment_id}/collections/{collection_id}/training_data/{query_id}/examples | Add example to training data query
[**delete_all_training_data**](TrainingDataApi.md#delete_all_training_data) | **Delete** /v1/environments/{environment_id}/collections/{collection_id}/training_data | Delete all training data
[**delete_training_data**](TrainingDataApi.md#delete_training_data) | **Delete** /v1/environments/{environment_id}/collections/{collection_id}/training_data/{query_id} | Delete a training data query
[**delete_training_example**](TrainingDataApi.md#delete_training_example) | **Delete** /v1/environments/{environment_id}/collections/{collection_id}/training_data/{query_id}/examples/{example_id} | Delete example for training data query
[**get_training_data**](TrainingDataApi.md#get_training_data) | **Get** /v1/environments/{environment_id}/collections/{collection_id}/training_data/{query_id} | Get details about a query
[**get_training_example**](TrainingDataApi.md#get_training_example) | **Get** /v1/environments/{environment_id}/collections/{collection_id}/training_data/{query_id}/examples/{example_id} | Get details for training data example
[**list_training_data**](TrainingDataApi.md#list_training_data) | **Get** /v1/environments/{environment_id}/collections/{collection_id}/training_data | List training data
[**list_training_examples**](TrainingDataApi.md#list_training_examples) | **Get** /v1/environments/{environment_id}/collections/{collection_id}/training_data/{query_id}/examples | List examples for a training data query
[**update_training_example**](TrainingDataApi.md#update_training_example) | **Put** /v1/environments/{environment_id}/collections/{collection_id}/training_data/{query_id}/examples/{example_id} | Change label or cross reference for example


# **add_training_data**
> ::models::TrainingQuery add_training_data(ctx, ctx, environment_id, collection_id, version, new_training_query)
Add query to training data

Adds a query to the training data for this collection. The query can contain a filter and natural language query.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **new_training_query** | [**NewTrainingQuery**](NewTrainingQuery.md)| The body of the training data query that is to be added to the collection's training data. | 

### Return type

[**::models::TrainingQuery**](TrainingQuery.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_training_example**
> ::models::TrainingExample create_training_example(ctx, ctx, environment_id, collection_id, query_id, version, training_example)
Add example to training data query

Adds a example to this training data query.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **query_id** | **String**| The ID of the query used for training. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **training_example** | [**TrainingExample**](TrainingExample.md)| The body of the example that is to be added to the specified query. | 

### Return type

[**::models::TrainingExample**](TrainingExample.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_all_training_data**
> delete_all_training_data(ctx, ctx, environment_id, collection_id, version)
Delete all training data

Deletes all training data from a collection.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

 (empty response body)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_training_data**
> delete_training_data(ctx, ctx, environment_id, collection_id, query_id, version)
Delete a training data query

Removes the training data query and all associated examples from the training data set.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **query_id** | **String**| The ID of the query used for training. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

 (empty response body)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_training_example**
> delete_training_example(ctx, ctx, environment_id, collection_id, query_id, example_id, version)
Delete example for training data query

Deletes the example document with the given ID from the training data query.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **query_id** | **String**| The ID of the query used for training. | 
  **example_id** | **String**| The ID of the document as it is indexed. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

 (empty response body)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_training_data**
> ::models::TrainingQuery get_training_data(ctx, ctx, environment_id, collection_id, query_id, version)
Get details about a query

Gets details for a specific training data query, including the query string and all examples.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **query_id** | **String**| The ID of the query used for training. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::TrainingQuery**](TrainingQuery.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_training_example**
> ::models::TrainingExample get_training_example(ctx, ctx, environment_id, collection_id, query_id, example_id, version)
Get details for training data example

Gets the details for this training example.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **query_id** | **String**| The ID of the query used for training. | 
  **example_id** | **String**| The ID of the document as it is indexed. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::TrainingExample**](TrainingExample.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_training_data**
> ::models::TrainingDataSet list_training_data(ctx, ctx, environment_id, collection_id, version)
List training data

Lists the training data for the specified collection.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::TrainingDataSet**](TrainingDataSet.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_training_examples**
> ::models::TrainingExampleList list_training_examples(ctx, ctx, environment_id, collection_id, query_id, version)
List examples for a training data query

List all examples for this training data query.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **query_id** | **String**| The ID of the query used for training. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::TrainingExampleList**](TrainingExampleList.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_training_example**
> ::models::TrainingExample update_training_example(ctx, ctx, environment_id, collection_id, query_id, example_id, version, training_example_patch)
Change label or cross reference for example

Changes the label or cross reference query for this training data example.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **query_id** | **String**| The ID of the query used for training. | 
  **example_id** | **String**| The ID of the document as it is indexed. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **training_example_patch** | [**TrainingExamplePatch**](TrainingExamplePatch.md)| The body of the example that is to be added to the specified query. | 

### Return type

[**::models::TrainingExample**](TrainingExample.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

