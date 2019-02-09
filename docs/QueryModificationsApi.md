# \QueryModificationsApi

All URIs are relative to *http://http:/discovery/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_expansions**](QueryModificationsApi.md#create_expansions) | **Post** /v1/environments/{environment_id}/collections/{collection_id}/expansions | Create or update expansion list
[**create_stopword_list**](QueryModificationsApi.md#create_stopword_list) | **Post** /v1/environments/{environment_id}/collections/{collection_id}/word_lists/stopwords | Create stopword list
[**create_tokenization_dictionary**](QueryModificationsApi.md#create_tokenization_dictionary) | **Post** /v1/environments/{environment_id}/collections/{collection_id}/word_lists/tokenization_dictionary | Create tokenization dictionary
[**delete_expansions**](QueryModificationsApi.md#delete_expansions) | **Delete** /v1/environments/{environment_id}/collections/{collection_id}/expansions | Delete the expansion list
[**delete_stopword_list**](QueryModificationsApi.md#delete_stopword_list) | **Delete** /v1/environments/{environment_id}/collections/{collection_id}/word_lists/stopwords | Delete a custom stopword list
[**delete_tokenization_dictionary**](QueryModificationsApi.md#delete_tokenization_dictionary) | **Delete** /v1/environments/{environment_id}/collections/{collection_id}/word_lists/tokenization_dictionary | Delete tokenization dictionary
[**get_stopword_list_status**](QueryModificationsApi.md#get_stopword_list_status) | **Get** /v1/environments/{environment_id}/collections/{collection_id}/word_lists/stopwords | Get stopword list status
[**get_tokenization_dictionary_status**](QueryModificationsApi.md#get_tokenization_dictionary_status) | **Get** /v1/environments/{environment_id}/collections/{collection_id}/word_lists/tokenization_dictionary | Get tokenization dictionary status
[**list_expansions**](QueryModificationsApi.md#list_expansions) | **Get** /v1/environments/{environment_id}/collections/{collection_id}/expansions | Get the expansion list


# **create_expansions**
> ::models::Expansions create_expansions(ctx, ctx, environment_id, collection_id, version, expansions)
Create or update expansion list

Create or replace the Expansion list for this collection. The maximum number of expanded terms per collection is `500`. The current expansion list is replaced with the uploaded content.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **expansions** | [**Expansions**](Expansions.md)| An object that defines the expansion list. | 

### Return type

[**::models::Expansions**](Expansions.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_stopword_list**
> ::models::TokenDictStatusResponse create_stopword_list(ctx, ctx, environment_id, collection_id, version, stopword_file)
Create stopword list

Upload a custom stopword list to use with the specified collection.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **stopword_file** | **::models::File**| The content of the stopword list to ingest. | 

### Return type

[**::models::TokenDictStatusResponse**](TokenDictStatusResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: multipart/form-data
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_tokenization_dictionary**
> ::models::TokenDictStatusResponse create_tokenization_dictionary(ctx, ctx, environment_id, collection_id, version, optional)
Create tokenization dictionary

Upload a custom tokenization dictionary to use with the specified collection.

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
 **token_dict** | [**TokenDict**](TokenDict.md)| An object that represents the tokenization dictionary to be uploaded. | 

### Return type

[**::models::TokenDictStatusResponse**](TokenDictStatusResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_expansions**
> delete_expansions(ctx, ctx, environment_id, collection_id, version)
Delete the expansion list

Remove the expansion information for this collection. The expansion list must be deleted to disable query expansion for a collection. 

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

# **delete_stopword_list**
> Value delete_stopword_list(ctx, ctx, environment_id, collection_id, version)
Delete a custom stopword list

Delete a custom stopword list from the collection. After a custom stopword list is deleted, the default list is used for the collection.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**Value**](Value.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_tokenization_dictionary**
> Value delete_tokenization_dictionary(ctx, ctx, environment_id, collection_id, version)
Delete tokenization dictionary

Delete the tokenization dictionary from the collection.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**Value**](Value.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_stopword_list_status**
> ::models::TokenDictStatusResponse get_stopword_list_status(ctx, ctx, environment_id, collection_id, version)
Get stopword list status

Returns the current status of the stopword list for the specified collection.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::TokenDictStatusResponse**](TokenDictStatusResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tokenization_dictionary_status**
> ::models::TokenDictStatusResponse get_tokenization_dictionary_status(ctx, ctx, environment_id, collection_id, version)
Get tokenization dictionary status

Returns the current status of the tokenization dictionary for the specified collection.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::TokenDictStatusResponse**](TokenDictStatusResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_expansions**
> ::models::Expansions list_expansions(ctx, ctx, environment_id, collection_id, version)
Get the expansion list

Returns the current expansion list for the specified collection. If an expansion list is not specified, an object with empty expansion arrays is returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::Expansions**](Expansions.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

