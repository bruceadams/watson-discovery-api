# \DocumentsApi

All URIs are relative to *http://http:/discovery/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_document**](DocumentsApi.md#add_document) | **Post** /v1/environments/{environment_id}/collections/{collection_id}/documents | Add a document
[**delete_document**](DocumentsApi.md#delete_document) | **Delete** /v1/environments/{environment_id}/collections/{collection_id}/documents/{document_id} | Delete a document
[**get_document_status**](DocumentsApi.md#get_document_status) | **Get** /v1/environments/{environment_id}/collections/{collection_id}/documents/{document_id} | Get document details
[**update_document**](DocumentsApi.md#update_document) | **Post** /v1/environments/{environment_id}/collections/{collection_id}/documents/{document_id} | Update a document


# **add_document**
> ::models::DocumentAccepted add_document(ctx, ctx, environment_id, collection_id, version, optional)
Add a document

Add a document to a collection with optional metadata.    * The **version** query parameter is still required.    * Returns immediately after the system has accepted the document for processing.    * The user must provide document content, metadata, or both. If the request is missing both document content and metadata, it is rejected.    * The user can set the **Content-Type** parameter on the **file** part to indicate the media type of the document. If the **Content-Type** parameter is missing or is one of the generic media types (for example, `application/octet-stream`), then the service attempts to automatically detect the document's media type.    * The following field names are reserved and will be filtered out if present after normalization: `id`, `score`, `highlight`, and any field with the prefix of: `_`, `+`, or `-`    * Fields with empty name values after normalization are filtered out before indexing.    * Fields containing the following characters after normalization are filtered out before indexing: `#` and `,` 

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
 **file** | **::models::File**| The content of the document to ingest. The maximum supported file size is 50 megabytes. Files larger than 50 megabytes is rejected. | 
 **metadata** | **String**| If you're using the Data Crawler to upload your documents, you can test a document against the type of metadata that the Data Crawler might send. The maximum supported metadata file size is 1 MB. Metadata parts larger than 1 MB are rejected. Example:  ``` {   \\\"Creator\\\": \\\"Johnny Appleseed\\\",   \\\"Subject\\\": \\\"Apples\\\" } ``` | 

### Return type

[**::models::DocumentAccepted**](DocumentAccepted.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: multipart/form-data
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_document**
> ::models::DeleteDocumentResponse delete_document(ctx, ctx, environment_id, collection_id, document_id, version)
Delete a document

If the given document ID is invalid, or if the document is not found, then the a success response is returned (HTTP status code `200`) with the status set to 'deleted'.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **document_id** | **String**| The ID of the document. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::DeleteDocumentResponse**](DeleteDocumentResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_document_status**
> ::models::DocumentStatus get_document_status(ctx, ctx, environment_id, collection_id, document_id, version)
Get document details

Fetch status details about a submitted document. **Note:** this operation does not return the document itself. Instead, it returns only the document's processing status and any notices (warnings or errors) that were generated when the document was ingested. Use the query API to retrieve the actual document content.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **document_id** | **String**| The ID of the document. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::DocumentStatus**](DocumentStatus.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_document**
> ::models::DocumentAccepted update_document(ctx, ctx, environment_id, collection_id, document_id, version, optional)
Update a document

Replace an existing document. Starts ingesting a document with optional metadata.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **collection_id** | **String**| The ID of the collection. | 
  **document_id** | **String**| The ID of the document. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **environment_id** | **String**| The ID of the environment. | 
 **collection_id** | **String**| The ID of the collection. | 
 **document_id** | **String**| The ID of the document. | 
 **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
 **file** | **::models::File**| The content of the document to ingest. The maximum supported file size is 50 megabytes. Files larger than 50 megabytes is rejected. | 
 **metadata** | **String**| If you're using the Data Crawler to upload your documents, you can test a document against the type of metadata that the Data Crawler might send. The maximum supported metadata file size is 1 MB. Metadata parts larger than 1 MB are rejected. Example:  ``` {   \\\"Creator\\\": \\\"Johnny Appleseed\\\",   \\\"Subject\\\": \\\"Apples\\\" } ``` | 

### Return type

[**::models::DocumentAccepted**](DocumentAccepted.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: multipart/form-data
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

