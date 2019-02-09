# \GatewayConfigurationApi

All URIs are relative to *http://http:/discovery/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_gateway**](GatewayConfigurationApi.md#create_gateway) | **Post** /v1/environments/{environment_id}/gateways | Create Gateway
[**delete_gateway**](GatewayConfigurationApi.md#delete_gateway) | **Delete** /v1/environments/{environment_id}/gateways/{gateway_id} | Delete Gateway
[**get_gateway**](GatewayConfigurationApi.md#get_gateway) | **Get** /v1/environments/{environment_id}/gateways/{gateway_id} | List Gateway Details
[**list_gateways**](GatewayConfigurationApi.md#list_gateways) | **Get** /v1/environments/{environment_id}/gateways | List Gateways


# **create_gateway**
> ::models::Gateway create_gateway(ctx, ctx, environment_id, version, optional)
Create Gateway

Create a gateway configuration to use with a remotely installed gateway.

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
 **gateway_name** | [**GatewayName**](GatewayName.md)| The name of the gateway to created | 

### Return type

[**::models::Gateway**](Gateway.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_gateway**
> ::models::GatewayDelete delete_gateway(ctx, ctx, environment_id, version, gateway_id)
Delete Gateway

Delete the specified gateway configuration.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **gateway_id** | **String**| The requested gateway ID. | 

### Return type

[**::models::GatewayDelete**](GatewayDelete.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_gateway**
> ::models::Gateway get_gateway(ctx, ctx, environment_id, version, gateway_id)
List Gateway Details

List information about the specified gateway.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **gateway_id** | **String**| The requested gateway ID. | 

### Return type

[**::models::Gateway**](Gateway.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_gateways**
> ::models::GatewayList list_gateways(ctx, ctx, environment_id, version)
List Gateways

List the currently configured gateways.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **environment_id** | **String**| The ID of the environment. | 
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]

### Return type

[**::models::GatewayList**](GatewayList.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

