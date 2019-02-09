# QueryEntities

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**feature** | **String** | The entity query feature to perform. Supported features are `disambiguate` and `similar_entities` | [optional] 
**entity** | [***::models::QueryEntitiesEntity**](QueryEntitiesEntity.md) |  | [optional] 
**context** | [***::models::QueryEntitiesContext**](QueryEntitiesContext.md) |  | [optional] 
**count** | **i32** | The number of results to return. The default is `10`. The maximum is `1000`. | [optional] 
**evidence_count** | **i32** | The number of evidence items to return for each result. The default is `0`. The maximum number of evidence items per query is 10,000. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


