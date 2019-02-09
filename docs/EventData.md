# EventData

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**environment_id** | **String** | The **environment_id** associated with the query that the event is associated with. | 
**session_token** | **String** | The session token that was returned as part of the query results that this event is associated with. | 
**client_timestamp** | **String** | The optional timestamp for the event that was created. If not provided, the time that the event was created in the log was used. | [optional] 
**display_rank** | **i32** | The rank of the result item which the event is associated with. | [optional] 
**collection_id** | **String** | The **collection_id** of the document that this event is associated with. | 
**document_id** | **String** | The **document_id** of the document that this event is associated with. | 
**query_id** | **String** | The query identifier stored in the log. The query and any events associated with that query are stored with the same **query_id**. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


