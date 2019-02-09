# QueryResult

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the document. | [optional] 
**score** | **f64** | *Deprecated* This field is now part of the **result_metadata** object. | [optional] 
**metadata** | [***Value**](.md) | Metadata of the document. | [optional] 
**collection_id** | **String** | The collection ID of the collection containing the document for this result. | [optional] 
**result_metadata** | [***::models::QueryResultMetadata**](QueryResultMetadata.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


