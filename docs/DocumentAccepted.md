# DocumentAccepted

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**document_id** | **String** | The unique identifier of the ingested document. | [optional] 
**status** | **String** | Status of the document in the ingestion process. A status of `processing` is returned for documents that are ingested with a *version* date before `2019-01-01`. The `pending` status is returned for all others. | [optional] 
**notices** | [**Vec<::models::Notice>**](Notice.md) | Array of notices produced by the document-ingestion process. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


