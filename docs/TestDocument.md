# TestDocument

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**configuration_id** | **String** | The unique identifier for the configuration. | [optional] 
**status** | **String** | Status of the preview operation. | [optional] 
**enriched_field_units** | **i32** | The number of 10-kB chunks of field data that were enriched. This can be used to estimate the cost of running a real ingestion. | [optional] 
**original_media_type** | **String** | Format of the test document. | [optional] 
**snapshots** | [**Vec<::models::DocumentSnapshot>**](DocumentSnapshot.md) | An array of objects that describe each step in the preview process. | [optional] 
**notices** | [**Vec<::models::Notice>**](Notice.md) | An array of notice messages about the preview operation. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


