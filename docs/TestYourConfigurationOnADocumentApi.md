# \TestYourConfigurationOnADocumentApi

All URIs are relative to *http://http:/discovery/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**test_configuration_in_environment**](TestYourConfigurationOnADocumentApi.md#test_configuration_in_environment) | **Post** /v1/environments/{environment_id}/preview | Test configuration


# **test_configuration_in_environment**
> ::models::TestDocument test_configuration_in_environment(ctx, ctx, environment_id, version, optional)
Test configuration

Runs a sample document through the default or your configuration and returns diagnostic information designed to help you understand how the document was processed. The document is not added to the index.

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
 **step** | **String**| Specify to only run the input document through the given step instead of running the input document through the entire ingestion workflow. Valid values are `convert`, `enrich`, and `normalize`. | 
 **configuration_id** | **String**| The ID of the configuration to use to process the document. If the **configuration** form part is also provided (both are present at the same time), then the request will be rejected. | 
 **configuration** | **String**| The configuration to use to process the document. If this part is provided, then the provided configuration is used to process the document. If the **configuration_id** is also provided (both are present at the same time), then request is rejected. The maximum supported configuration size is 1 MB. Configuration parts larger than 1 MB are rejected. See the `GET /configurations/{configuration_id}` operation for an example configuration. | 
 **file** | **::models::File**| The content of the document to ingest. The maximum supported file size is 50 megabytes. Files larger than 50 megabytes is rejected. | 
 **metadata** | **String**| If you're using the Data Crawler to upload your documents, you can test a document against the type of metadata that the Data Crawler might send. The maximum supported metadata file size is 1 MB. Metadata parts larger than 1 MB are rejected. Example:  ``` {   \\\"Creator\\\": \\\"Johnny Appleseed\\\",   \\\"Subject\\\": \\\"Apples\\\" } ``` | 

### Return type

[**::models::TestDocument**](TestDocument.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: multipart/form-data
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

