# QueryEvidence

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**document_id** | **String** | The docuemnt ID (as indexed in Discovery) of the evidence location. | [optional] 
**field** | **String** | The field of the document where the supporting evidence was identified. | [optional] 
**start_offset** | **i32** | The start location of the evidence in the identified field. This value is inclusive. | [optional] 
**end_offset** | **i32** | The end location of the evidence in the identified field. This value is inclusive. | [optional] 
**entities** | [**Vec<::models::QueryEvidenceEntity>**](QueryEvidenceEntity.md) | An array of entity objects that show evidence of the result. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


