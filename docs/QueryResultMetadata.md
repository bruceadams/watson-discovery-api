# QueryResultMetadata

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**score** | **f64** | An unbounded measure of the relevance of a particular result, dependent on the query and matching document. A higher score indicates a greater match to the query parameters. | 
**confidence** | **f64** | The confidence score for the given result. Calculated based on how relevant the result is estimated to be. confidence can range from `0.0` to `1.0`. The higher the number, the more relevant the document. The `confidence` value for a result was calculated using the model specified in the `document_retrieval_strategy` field of the result set. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


