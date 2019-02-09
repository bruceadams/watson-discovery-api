# DocumentStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**document_id** | **String** | The unique identifier of the document. | 
**configuration_id** | **String** | The unique identifier for the configuration. | [optional] 
**created** | **String** | The creation date of the document in the format yyyy-MM-dd'T'HH:mm:ss.SSS'Z' | [optional] 
**updated** | **String** | Date of the most recent document update, in the format yyyy-MM-dd'T'HH:mm:ss.SSS'Z' | [optional] 
**status** | **String** | Status of the document in the ingestion process. | 
**status_description** | **String** | Description of the document status. | 
**filename** | **String** | Name of the original source file (if available). | [optional] 
**file_type** | **String** | The type of the original source file. | [optional] 
**sha1** | **String** | The SHA-1 hash of the original source file (formatted as a hexadecimal string). | [optional] 
**notices** | [**Vec<::models::Notice>**](Notice.md) | Array of notices produced by the document-ingestion process. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


