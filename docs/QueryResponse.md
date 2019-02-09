# QueryResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**matching_results** | **i32** |  | [optional] 
**results** | [**Vec<::models::QueryResult>**](QueryResult.md) |  | [optional] 
**aggregations** | [**Vec<::models::QueryAggregation>**](QueryAggregation.md) |  | [optional] 
**passages** | [**Vec<::models::QueryPassages>**](QueryPassages.md) |  | [optional] 
**duplicates_removed** | **i32** |  | [optional] 
**session_token** | **String** | The session token for this query. The session token can be used to add events associated with this query to the query and event log.  **Important:** Session tokens are case sensitive. | [optional] 
**retrieval_details** | [***::models::RetrievalDetails**](RetrievalDetails.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


