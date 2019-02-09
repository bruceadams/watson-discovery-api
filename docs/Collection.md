# Collection

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_id** | **String** | The unique identifier of the collection. | [optional] 
**name** | **String** | The name of the collection. | [optional] 
**description** | **String** | The description of the collection. | [optional] 
**created** | **String** | The creation date of the collection in the format yyyy-MM-dd'T'HH:mmcon:ss.SSS'Z' | [optional] 
**updated** | **String** | The timestamp of when the collection was last updated in the format yyyy-MM-dd'T'HH:mm:ss.SSS'Z' | [optional] 
**status** | **String** | The status of the collection. | [optional] 
**configuration_id** | **String** | The unique identifier of the collection's configuration. | [optional] 
**language** | **String** | The language of the documents stored in the collection. Permitted values include `en` (English), `de` (German), and `es` (Spanish). | [optional] 
**document_counts** | [***::models::DocumentCounts**](DocumentCounts.md) |  | [optional] 
**disk_usage** | [***::models::CollectionDiskUsage**](CollectionDiskUsage.md) |  | [optional] 
**training_status** | [***::models::TrainingStatus**](TrainingStatus.md) |  | [optional] 
**source_crawl** | [***::models::SourceStatus**](SourceStatus.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


