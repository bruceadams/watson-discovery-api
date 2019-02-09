# LogQueryResponseResultDocumentsResult

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**position** | **i32** | The result rank of this document. A position of `1` indicates that it was the first returned result. | [optional] 
**document_id** | **String** | The **document_id** of the document that this result represents | [optional] 
**score** | **f64** | The raw score of this result. A higher score indicates a greater match to the query parameters. | [optional] 
**confidence** | **f64** | The confidence score of the result's analysis. A higher score indicating greater confidence. | [optional] 
**collection_id** | **String** | The **collection_id** of the document represented by this result. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


