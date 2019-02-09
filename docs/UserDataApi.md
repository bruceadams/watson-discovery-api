# \UserDataApi

All URIs are relative to *http://http:/discovery/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user_data**](UserDataApi.md#delete_user_data) | **Delete** /v1/user_data | Delete labeled data


# **delete_user_data**
> delete_user_data(ctx, ctx, customer_id)
Delete labeled data

Deletes all data associated with a specified customer ID. The method has no effect if no data is associated with the customer ID.   You associate a customer ID with data by passing the **X-Watson-Metadata** header with a request that passes data. For more information about personal data and customer IDs, see [Information security](https://console.bluemix.net/docs/services/discovery/information-security.html).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **customer_id** | **String**| The customer ID for which all data is to be deleted. | 

### Return type

 (empty response body)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

