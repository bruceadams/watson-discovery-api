# Calculation

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type_** | **String** | The type of aggregation command used. For example: term, filter, max, min, etc. | [optional] 
**results** | [**Vec<::models::AggregationResult>**](AggregationResult.md) |  | [optional] 
**matching_results** | **i32** | Number of matching results. | [optional] 
**aggregations** | [**Vec<::models::QueryAggregation>**](QueryAggregation.md) | Aggregations returned by the Discovery service. | [optional] 
**field** | **String** | The field where the aggregation is located in the document. | [optional] 
**value** | **f64** | Value of the aggregation. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


