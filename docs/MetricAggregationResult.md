# MetricAggregationResult

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key_as_string** | **String** | Date in string form representing the start of this interval. | [optional] 
**key** | **i64** | Unix epoch time equivalent of the **key_as_string**, that represents the start of this interval | [optional] 
**matching_results** | **i32** | Number of matching results. | [optional] 
**event_rate** | **f64** | The number of queries with associated events divided by the total number of queries for the interval. Only returned with **event_rate** metrics. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


