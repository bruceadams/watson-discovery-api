# QueryRelations

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entities** | [**Vec<::models::QueryRelationsEntity>**](QueryRelationsEntity.md) | An array of entities to find relationships for. | [optional] 
**context** | [***::models::QueryEntitiesContext**](QueryEntitiesContext.md) |  | [optional] 
**sort** | **String** | The sorting method for the relationships, can be `score` or `frequency`. `frequency` is the number of unique times each entity is identified. The default is `score`. This parameter cannot be used in the same query as the **bias** parameter. | [optional] 
**filter** | [***::models::QueryRelationsFilter**](QueryRelationsFilter.md) |  | [optional] 
**count** | **i32** | The number of results to return. The default is `10`. The maximum is `1000`. | [optional] 
**evidence_count** | **i32** | The number of evidence items to return for each result. The default is `0`. The maximum number of evidence items per query is 10,000. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


