# QueryNoticesResult

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the document. | [optional] 
**score** | **f64** | *Deprecated* This field is now part of the **result_metadata** object. | [optional] 
**metadata** | [***Value**](.md) | Metadata of the document. | [optional] 
**collection_id** | **String** | The collection ID of the collection containing the document for this result. | [optional] 
**result_metadata** | [***::models::QueryResultMetadata**](QueryResultMetadata.md) |  | [optional] 
**code** | **i32** | The internal status code returned by the ingestion subsystem indicating the overall result of ingesting the source document. | [optional] 
**filename** | **String** | Name of the original source file (if available). | [optional] 
**file_type** | **String** | The type of the original source file. | [optional] 
**sha1** | **String** | The SHA-1 hash of the original source file (formatted as a hexadecimal string). | [optional] 
**notices** | [**Vec<::models::Notice>**](Notice.md) | Array of notices for the document. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


