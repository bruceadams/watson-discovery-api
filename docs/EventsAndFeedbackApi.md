# \EventsAndFeedbackApi

All URIs are relative to *http://http:/discovery/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_event**](EventsAndFeedbackApi.md#create_event) | **Post** /v1/events | Create event
[**get_metrics_event_rate**](EventsAndFeedbackApi.md#get_metrics_event_rate) | **Get** /v1/metrics/event_rate | Percentage of queries with an associated event
[**get_metrics_query**](EventsAndFeedbackApi.md#get_metrics_query) | **Get** /v1/metrics/number_of_queries | Number of queries over time
[**get_metrics_query_event**](EventsAndFeedbackApi.md#get_metrics_query_event) | **Get** /v1/metrics/number_of_queries_with_event | Number of queries with an event over time
[**get_metrics_query_no_results**](EventsAndFeedbackApi.md#get_metrics_query_no_results) | **Get** /v1/metrics/number_of_queries_with_no_search_results | Number of queries with no search results over time
[**get_metrics_query_token_event**](EventsAndFeedbackApi.md#get_metrics_query_token_event) | **Get** /v1/metrics/top_query_tokens_with_event_rate | Most frequent query tokens with an event
[**query_log**](EventsAndFeedbackApi.md#query_log) | **Get** /v1/logs | Search the query and event log


# **create_event**
> ::models::CreateEventResponse create_event(ctx, ctx, version, create_event_object)
Create event

The **Events** API can be used to create log entries that are associated with specific queries. For example, you can record which documents in the results set were \"clicked\" by a user and when that click occured.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **version** | **String**| A date (`YYYY-MM-DD`) that identifies the specific version of the API to use when processing the request. | [default to 2018-12-03]
  **create_event_object** | [**CreateEventObject**](CreateEventObject.md)| An object that defines a query event to be added to the log. | 

### Return type

[**::models::CreateEventResponse**](CreateEventResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_metrics_event_rate**
> ::models::MetricResponse get_metrics_event_rate(ctx, ctx, version, optional)
Percentage of queries with an associated event

The percentage of queries using the **natural_language_query** parameter that have a corresponding \"click\" event over a specified time window.  This metric requires having integrated event tracking in your application using the **Events** API.

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
 **start_time** | **String**| Metric is computed from data recorded after this timestamp; must be in `YYYY-MM-DDThh:mm:ssZ` format. | 
 **end_time** | **String**| Metric is computed from data recorded before this timestamp; must be in `YYYY-MM-DDThh:mm:ssZ` format. | 
 **result_type** | **String**| The type of result to consider when calculating the metric. | 

### Return type

[**::models::MetricResponse**](MetricResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_metrics_query**
> ::models::MetricResponse get_metrics_query(ctx, ctx, version, optional)
Number of queries over time

Total number of queries using the **natural_language_query** parameter over a specific time window.

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
 **start_time** | **String**| Metric is computed from data recorded after this timestamp; must be in `YYYY-MM-DDThh:mm:ssZ` format. | 
 **end_time** | **String**| Metric is computed from data recorded before this timestamp; must be in `YYYY-MM-DDThh:mm:ssZ` format. | 
 **result_type** | **String**| The type of result to consider when calculating the metric. | 

### Return type

[**::models::MetricResponse**](MetricResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_metrics_query_event**
> ::models::MetricResponse get_metrics_query_event(ctx, ctx, version, optional)
Number of queries with an event over time

Total number of queries using the **natural_language_query** parameter that have a corresponding \"click\" event over a specified time window. This metric requires having integrated event tracking in your application using the **Events** API.

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
 **start_time** | **String**| Metric is computed from data recorded after this timestamp; must be in `YYYY-MM-DDThh:mm:ssZ` format. | 
 **end_time** | **String**| Metric is computed from data recorded before this timestamp; must be in `YYYY-MM-DDThh:mm:ssZ` format. | 
 **result_type** | **String**| The type of result to consider when calculating the metric. | 

### Return type

[**::models::MetricResponse**](MetricResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_metrics_query_no_results**
> ::models::MetricResponse get_metrics_query_no_results(ctx, ctx, version, optional)
Number of queries with no search results over time

Total number of queries using the **natural_language_query** parameter that have no results returned over a specified time window.

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
 **start_time** | **String**| Metric is computed from data recorded after this timestamp; must be in `YYYY-MM-DDThh:mm:ssZ` format. | 
 **end_time** | **String**| Metric is computed from data recorded before this timestamp; must be in `YYYY-MM-DDThh:mm:ssZ` format. | 
 **result_type** | **String**| The type of result to consider when calculating the metric. | 

### Return type

[**::models::MetricResponse**](MetricResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_metrics_query_token_event**
> ::models::MetricTokenResponse get_metrics_query_token_event(ctx, ctx, version, optional)
Most frequent query tokens with an event

The most frequent query tokens parsed from the **natural_language_query** parameter and their corresponding \"click\" event rate within the recording period (queries and events are stored for 30 days). A query token is an individual word or unigram within the query string.

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
 **count** | **i32**| Number of results to return. | [default to 10]

### Return type

[**::models::MetricTokenResponse**](MetricTokenResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **query_log**
> ::models::LogQueryResponse query_log(ctx, ctx, version, optional)
Search the query and event log

Searches the query and event log to find query sessions that match the specified criteria. Searching the **logs** endpoint uses the standard Discovery query syntax for the parameters that are supported.

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
 **filter** | **String**| A cacheable query that excludes documents that don't mention the query content. Filter searches are better for metadata-type searches and for assessing the concepts in the data set. | 
 **query** | **String**| A query search returns all documents in your data set with full enrichments and full text, but with the most relevant documents listed first. Use a query search when you want to find the most relevant search results. You cannot use **natural_language_query** and **query** at the same time. | 
 **count** | **i32**| Number of results to return. | [default to 10]
 **offset** | **i32**| The number of query results to skip at the beginning. For example, if the total number of results that are returned is 10 and the offset is 8, it returns the last two results. | 
 **sort** | [**Vec<String>**](String.md)| A comma-separated list of fields in the document to sort on. You can optionally specify a sort direction by prefixing the field with `-` for descending or `+` for ascending. Ascending is the default sort direction if no prefix is specified. | 

### Return type

[**::models::LogQueryResponse**](LogQueryResponse.md)

### Authorization

[IAM](../README.md#IAM), [basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

