# NluEnrichmentEntities

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sentiment** | **bool** | When `true`, sentiment analysis of entities will be performed on the specified field. | [optional] 
**emotion** | **bool** | When `true`, emotion detection of entities will be performed on the specified field. | [optional] 
**limit** | **i32** | The maximum number of entities to extract for each instance of the specified field. | [optional] 
**mentions** | **bool** | When `true`, the number of mentions of each identified entity is recorded. The default is `false` | [optional] 
**mention_types** | **bool** | When `true`, the types of mentions for each idetifieid entity is recorded. The default is `false`.  | [optional] 
**sentence_locations** | **bool** | When `true`, a list of sentence locations for each instance of each identified entity is recorded. The default is `false`. | [optional] 
**model** | **String** | The enrichement model to use with entity extraction. May be a custom model provided by Watson Knowledge Studio, the public model for use with Knowledge Graph `en-news`, or the default public model `alchemy`. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


